use std::path::PathBuf;

use self::rss::Feed;
mod rss;
use crate::{cli::Cli, errors::Error as PError, errors::Result as PResult};

/// A bot struct that handles the communication with the pleroma instance.
/// It also handles the RSS feed parsing.
#[derive(Debug)]
pub struct Bot {
    /// The bot token.
    pub bot_token: String,
    /// The base url of the pleroma instance.
    pub base_url: url::Url,
    /// The RSS feeds.
    pub feeds: Vec<Feed>,
}

impl Bot {
    /// Creates a new bot.
    pub fn new(bot_token: String, base_url: url::Url, rss_feeds_file: PathBuf) -> PResult<Self> {
        Ok(Bot {
            bot_token,
            base_url,
            feeds: rss::parse_feeds(rss_feeds_file)?,
        })
    }

    /// Posts the new content to the pleroma instance.
    pub async fn post_new_contents(&mut self) -> PResult<()> {
        for feed in &mut self.feeds {
            for content in &feed.check().await? {
                content
                    .post(self.base_url.as_str(), &self.bot_token)
                    .await?;
                // Sleep for 0.5 second to avoid rate limiting.
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
        Ok(())
    }
}

/// Runs the bot. Infinite loop.
pub async fn run(cli: Cli) -> PResult<()> {
    let mut bot = Bot::new(cli.bot_token, cli.pleroma_base_url, cli.rss_feeds_file)?;
    loop {
        // If the error is a request error, print it and continue.
        // Otherwise, return the error.
        match bot.post_new_contents().await {
            Ok(_) => (),
            Err(PError::RequestError(err)) => eprintln!("Error: {}", err),
            Err(err) => return Err(err),
        }
        // Sleep for 30 seconds.
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    }
}
