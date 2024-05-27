use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    sabi_trait::prelude::TD_Opaque,
    std_types::{RBox, RStr, RString, RVec},
};
use quick_search_lib::{ColoredChar, Log, PluginId, SearchLib, SearchLib_Ref, SearchResult, Searchable, Searchable_TO};

static NAME: &str = "WolframAlpha";

#[export_root_module]
pub fn get_library() -> SearchLib_Ref {
    SearchLib { get_searchable }.leak_into_prefix()
}

#[sabi_extern_fn]
fn get_searchable(id: PluginId, logger: quick_search_lib::ScopedLogger) -> Searchable_TO<'static, RBox<()>> {
    let this = WolframAlpha::new(id, logger);
    Searchable_TO::from_value(this, TD_Opaque)
}

struct WolframAlpha {
    id: PluginId,
    client: reqwest::blocking::Client,
    config: quick_search_lib::Config,
    logger: quick_search_lib::ScopedLogger,
}

impl WolframAlpha {
    fn new(id: PluginId, logger: quick_search_lib::ScopedLogger) -> Self {
        Self {
            id,
            client: reqwest::blocking::Client::new(),
            config: default_config(),
            logger,
        }
    }
}

impl Searchable for WolframAlpha {
    fn search(&self, query: RString) -> RVec<SearchResult> {
        let mut res: Vec<SearchResult> = vec![];
        let app_id = match self.config.get("Wolfram Alpha Application ID").and_then(|e| e.as_string()) {
            Some(app_id) => app_id,
            None => {
                self.logger.error("Wolfram Alpha Application ID not set");
                return res.into();
            }
        };
        let trim_end = self.config.get("Required suffix (will be trimmed)").and_then(|e| e.as_string()).unwrap_or("!w");
        let split = query.split(&trim_end);

        let mut query = String::new();
        let mut last_was_empty = false;
        for part in split {
            if !part.is_empty() {
                query.push_str(part);
                last_was_empty = false;
            } else {
                last_was_empty = true;
            }
        }
        if !last_was_empty {
            self.logger.trace("Query did not end with the required string");
            return res.into();
        }

        let response = match self
            .client
            .get(format!("http://api.wolframalpha.com/v1/result?appid={}&i={}", app_id, urlencoding::encode(&query)))
            .send()
        {
            Ok(response) => response,
            Err(e) => {
                self.logger.error(&format!("Failed to send request: {}", e));
                return res.into();
            }
        };
        if !response.status().is_success() {
            self.logger.error(&format!("Failed to get a successful response: {}", response.status()));
            return res.into();
        }

        let text = match response.text() {
            Ok(text) => text,
            Err(e) => {
                self.logger.error(&format!("Failed to get text from response: {}", e));
                return res.into();
            }
        };
        let include_query = self.config.get("Include query in clipboard").and_then(|e| e.as_bool()).unwrap_or(true);
        res.push(SearchResult::new(&format!("{}: {}", query, text)).set_extra_info(&if include_query { format!("{}\n{}", query, text) } else { text }));

        res.sort_by(|a, b| a.title().cmp(b.title()));
        res.dedup_by(|a, b| a.title() == b.title());

        res.into()
    }
    fn name(&self) -> RStr<'static> {
        NAME.into()
    }
    fn colored_name(&self) -> RVec<quick_search_lib::ColoredChar> {
        vec![
            ColoredChar::new('W', 0xFF6600FF),
            ColoredChar::new('o', 0xFF6600FF),
            ColoredChar::new('l', 0xFF6600FF),
            ColoredChar::new('f', 0xFF6600FF),
            ColoredChar::new('r', 0xFF6600FF),
            ColoredChar::new('a', 0xFF6600FF),
            ColoredChar::new('m', 0xFF6600FF),
            ColoredChar::new('A', 0xD40022FF),
            ColoredChar::new('l', 0xD40022FF),
            ColoredChar::new('p', 0xD40022FF),
            ColoredChar::new('h', 0xD40022FF),
            ColoredChar::new('a', 0xD40022FF),
        ]
        .into()
    }
    fn execute(&self, result: &SearchResult) {
        let s = result.extra_info();
        if let Ok::<clipboard::ClipboardContext, Box<dyn std::error::Error>>(mut clipboard) = clipboard::ClipboardProvider::new() {
            if let Ok(()) = clipboard::ClipboardProvider::set_contents(&mut clipboard, s.to_owned()) {
                println!("copied to clipboard: {}", s);
            } else {
                println!("failed to copy to clipboard: {}", s);
            }
        } else {
            self.logger.error("Failed to get clipboard context");
        }
    }
    fn plugin_id(&self) -> PluginId {
        self.id.clone()
    }
    fn get_config_entries(&self) -> quick_search_lib::Config {
        default_config()
    }
    fn lazy_load_config(&mut self, config: quick_search_lib::Config) {
        self.config = config;
    }
}

fn default_config() -> quick_search_lib::Config {
    let mut config = quick_search_lib::Config::new();

    config.insert("Example:\nWhat is the capital of France? !w\nParis".into(), quick_search_lib::EntryType::None);
    config.insert("Wolfram Alpha Application ID".into(), quick_search_lib::EntryType::String { value: RString::new() });
    config.insert("Required suffix (will be trimmed)".into(), quick_search_lib::EntryType::String { value: RString::from("!w") });
    config.insert("Include query in clipboard".into(), quick_search_lib::EntryType::Bool { value: false });
    config
}
