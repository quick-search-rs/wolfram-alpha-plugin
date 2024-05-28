# Wolfram Alpha Plugin
This is a plugin for [Quick Search](https://github.com/quick-search-rs/quick-search) that allows you to search Wolfram Alpha.

![Screenshot](/docs/assets/demo.png)

## Installation
1. Install [Quick Search](https://github.com/quick-search-rs/quick-search/releases/latest) if you haven't already.
2. Download the [latest release](https://github.com/quick-search-rs/wolfram-alpha-plugin/releases/latest) of this plugin.
3. Open the Quick Search settings (right-click the tray icon -> Configure) and click "Add Plugin" at the bottom.
4. Select the downloaded file.
5. The plugin should now be installed and ready to use.

## Setup
To use the plugin, go to [Wolfram Alpha](https://www.wolframalpha.com/) and create an account if you don't already have one.

Then, go to the [developer portal](https://developer.wolframalpha.com/portal/myapps/) and click "Get an App ID" to create a new app, the name and description are not important.

Set the API to "Short Answers" and click "Submit".

Now you should have an App ID, you copy this ID and open the Quick Search settings.

Click the name of the Wolfram Alpha plugin to open the plugin settings.

Paste the App ID into the "Wolfram Alpha Application ID" field then click "Close" on the popup window, and then "Save" to save your settings.

You can now use the plugin to search Wolfram Alpha from Quick Search.

## Usage
Simply type your question into the search bar, followed by the configured suffix (default is `!w`), you can also change the suffix in the plugin settings.

If the suffix is empty it will always search Wolfram Alpha, but an unpaid Wolfram Alpha account has a limit of 2000 queries per month. So it is not reccomended.