# Wolfram Alpha Plugin
This is a plugin for [Quick Search](https://github.com/quick-search-rs/quick-search) that allows you to search Wolfram Alpha.

![Screenshot](/docs/assets/demo.png)

## Usage
Simply type your question into the search bar, followed by the configured suffix (default is `!w`), you can also change the suffix in the plugin settings.

If the suffix is empty it will always search Wolfram Alpha, but an unpaid Wolfram Alpha account has a limit of 2000 queries per month. So it is not reccomended.

## Installation
1. Download and install [Quick Search](https://github.com/quick-search-rs/quick-search/releases/latest) if you haven't already.
2. Download the [latest release](https://github.com/quick-search-rs/wolfram-alpha-plugin/releases/latest) of this plugin.
3. Open the Quick Search settings (right-click the tray icon -> Configure) and click "Add Plugin" at the bottom.
4. Select the downloaded file.
5. The plugin should now be installed and ready to use.

## Setup
1. Get a Wolfram Alpha App ID.
    1. Go to [Wolfram Alpha](https://www.wolframalpha.com/) and create an account if you don't already have one.
    2. Then, go to the [developer portal](https://developer.wolframalpha.com/portal/myapps/) and click "Get an App ID" to create a new app, the name and description are not important.
    3. Set the API to "Short Answers" and click "Submit".
2. Put the App ID into the plugin settings.
    1. Open the Quick Search settings.
    2. Click the name of the Wolfram Alpha plugin to open the plugin settings.
    3. Paste the App ID into the "Wolfram Alpha Application ID" field.
    4. Click "Close" on the popup window, and then "Save" to save your settings.
3. You can now use the plugin to search Wolfram Alpha from Quick Search.
