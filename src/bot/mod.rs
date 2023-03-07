pub use self::rss::Feed;
use crate::{cli::Cli, config::Config, errors::Error as PError, errors::Result as PResult};
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
                    log::info!(
                        "Sleeping for {} seconds, before sending the next item.",
                        self.config.items_sleep
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(self.config.items_sleep))
                        .await;
                }
            }
        }
        Ok(())
    }
}

/// Runs the bot. Infinite loop.
pub async fn run(cli: Cli) -> PResult<()> {
    let config = Config::new(&cli)?;
    let mut bot = Bot::new(config)?;
    loop {
        // If the error is a request error, print it and continue.
        // Otherwise, return the error.
        match bot.post_new_contents().await {
            Ok(_) => log::info!("Finished checking for new contents."),
            Err(PError::Request(err)) => {
                log::error!("Error: {}", err);
                eprintln!("Error: {}", err)
            }
            Err(PError::Megalodon(err)) => {
                log::error!("Error: {}", err);
                eprintln!("Error: {}", err)
            }
            Err(err) => return Err(err),
        }
        // Sleep before checking for new contents.
        log::info!(
            "Waiting for new contents. Sleeping for {} seconds.",
            bot.config.watting_new
        );
        tokio::time::sleep(std::time::Duration::from_secs(bot.config.watting_new)).await;
    }
}
