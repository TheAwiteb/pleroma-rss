pub use self::rss::Feed;
use crate::{cli::Cli, config::Config, errors::Error as PError, errors::Result as PResult, utils};
#[cfg(feature = "preview-image")]
mod image;
mod rss;

/// A bot struct that handles the communication with the pleroma instance.
/// It also handles the RSS feed parsing.
#[derive(Debug)]
pub struct Bot {
    config: Config,
}

impl Bot {
    /// Creates a new bot.
    pub fn new(config: Config) -> PResult<Self> {
        log::debug!(
            "Creating a new bot. The base url is: {}. The feeds is: {}",
            config.base_url,
            config
                .feeds
                .iter()
                .map(|f| f.url.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        Ok(Self { config })
    }

    /// Posts the new content to the pleroma instance.
    pub async fn post_new_contents(&mut self) -> PResult<()> {
        log::info!("Checking for new contents.");
        let config = self.config.clone();
        for feed in &mut self.config.feeds {
            log::info!("Checking feed: {}", feed.url);
            for content in &feed.check().await? {
                log::info!("Found new content: {}", content.title);
                if config.dry_run {
                    log::info!("Dry run. Not posting.");
                    println!("{content:#?}");
                } else {
                    content.post(&config).await?;
                    log::info!("Sleeping for 0.5 seconds. Pleroma rate limit.");
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            }
        }
        Ok(())
    }
}

/// Runs the bot. Infinite loop.
pub async fn run(cli: Cli) -> PResult<()> {
    let config = Config::new(
        cli.access_token,
        cli.base_url,
        utils::parse_feeds(&cli.feeds_file, cli.only_new)?,
        cli.only_new,
        cli.dry_run,
        #[cfg(feature = "preview-image")]
        cli.preview_image_template,
        #[cfg(feature = "preview-image")]
        cli.default_preview_image,
    );
    let mut bot = Bot::new(config)?;
    loop {
        // If the error is a request error, print it and continue.
        // Otherwise, return the error.
        match bot.post_new_contents().await {
            Ok(_) => log::info!("Finished checking for new contents."),
            Err(PError::RequestError(err)) => {
                log::error!("Error: {}", err);
                eprintln!("Error: {}", err)
            }
            Err(PError::MegalodonError(err)) => {
                log::error!("Error: {}", err);
                eprintln!("Error: {}", err)
            }
            Err(err) => return Err(err),
        }
        // Sleep for 30 seconds.
        log::info!("Waiting for new contents. Sleeping for 30 seconds.");
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    }
}
