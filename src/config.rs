use crate::bot::Feed;

/// The bot configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// The bot token.
    pub bot_token: String,
    /// Base url of Pleroma instance.
    pub base_url: url::Url,
    /// Rss feeds file path.
    pub feeds: Vec<Feed>,
    /// Only new flag.
    pub only_new: bool,
    /// Dry run flag.
    pub dry_run: bool,
}

impl Config {
    /// Creates a new config.
    pub fn new(
        bot_token: String,
        base_url: url::Url,
        feeds: Vec<Feed>,
        only_new: bool,
        dry_run: bool,
    ) -> Self {
        Self {
            bot_token,
            base_url,
            feeds,
            only_new,
            dry_run,
        }
    }
}
