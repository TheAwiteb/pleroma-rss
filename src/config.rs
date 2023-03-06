use crate::{bot::Feed, cli::Cli, errors::Result as PResult, utils};
#[cfg(feature = "preview-image")]
use std::path::PathBuf;

/// The bot configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// The bot token.
    pub bot_token: String,
    /// Base url of Pleroma instance.
    pub base_url: url::Url,
    /// The sleep time between each feed in seconds.
    pub items_sleep: u64,
    /// The sleep time after end all feeds (wait for new items) in seconds.
    pub watting_new: u64,
    /// Rss feeds file path.
    pub feeds: Vec<Feed>,
    /// Only new flag.
    pub only_new: bool,
    /// Dry run flag.
    pub dry_run: bool,
    /// The preview image html template.
    #[cfg(feature = "preview-image")]
    pub preview_image_template: PathBuf,
    /// The default image of the preview image.
    #[cfg(feature = "preview-image")]
    pub default_preview_image: PathBuf,
}

impl Config {
    /// Creates a new config.
    pub fn new(cli: &Cli) -> PResult<Self> {
        Ok(Self {
            bot_token: cli.access_token.clone(),
            base_url: cli.base_url.clone(),
            items_sleep: cli.items_sleep,
            watting_new: cli.watting_new,
            feeds: utils::parse_feeds(&cli.feeds_file, cli.only_new)?,
            only_new: cli.only_new,
            dry_run: cli.dry_run,
            #[cfg(feature = "preview-image")]
            preview_image_template: cli.preview_image_template.clone(),
            #[cfg(feature = "preview-image")]
            default_preview_image: cli.default_preview_image.clone(),
        })
    }
}
