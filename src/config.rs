use std::path::PathBuf;

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
    /// The preview image html template.
    #[cfg(feature = "with-image")]
    pub preview_image_template: PathBuf,
    /// The default image of the preview image.
    #[cfg(feature = "with-image")]
    pub default_preview_image: PathBuf,
}

impl Config {
    /// Creates a new config.
    pub fn new(
        bot_token: String,
        base_url: url::Url,
        feeds: Vec<Feed>,
        only_new: bool,
        dry_run: bool,
        #[cfg(feature = "with-image")] preview_image_template: PathBuf,
        #[cfg(feature = "with-image")] default_preview_image: PathBuf,
    ) -> Self {
        Self {
            bot_token,
            base_url,
            feeds,
            only_new,
            dry_run,
            #[cfg(feature = "with-image")]
            preview_image_template,
            #[cfg(feature = "with-image")]
            default_preview_image,
        }
    }
}
