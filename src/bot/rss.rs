use crate::{
    errors::{Error as PError, Result as PResult},
    utils::remove_html_tags,
};
use chrono::DateTime;
use std::io::BufRead;
use url::Url;

/// The RSS feed struct. It contains the feed url and the last post date.
/// This is used to check if a new post has been made.
#[derive(Debug, Clone)]
pub struct Feed {
    /// The feed url.
    pub url: Url,
    /// The last post date.
    pub last_post: Option<u64>,
}

pub struct Content {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl Content {
    pub fn new(
        title: impl AsRef<str>,
        link: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Self {
        Self {
            title: title.as_ref().to_string(),
            link: link.as_ref().to_string(),
            description: description.as_ref().to_string(),
        }
    }

    pub async fn post(&self, base_url: &str, bot_token: &str) -> PResult<()> {
        let description = remove_html_tags(&self.description);
        let link = urlencoding::decode(self.link.as_str()).unwrap();
        megalodon::generator(
            megalodon::SNS::Pleroma,
            // Remove the last slash.
            base_url[..base_url.len() - 1].to_owned(),
            Some(bot_token.to_owned()),
            None,
        )
        .post_status(format!("{}\n\n{description}\n\n{link}", self.title), None)
        .await?;

        Ok(())
    }
}

impl Feed {
    /// Creates a new feed.
    pub fn new(url: Url) -> Self {
        Self {
            url,
            last_post: None,
        }
    }

    /// Checks if a new post has been made.
    /// If a new post has been made, it returns the post content.
    pub async fn check(&mut self) -> PResult<Vec<Content>> {
        let feed = reqwest::get(self.url.as_str()).await?.text().await?;
        let feed = feed.parse::<rss::Channel>()?;
        // Start from the last post and go backwards.
        // If a post is newer than the last post, it is added to the list.
        let feeds: Vec<_> = feed
            .items
            .into_iter()
            .rev()
            .map(|item| {
                let date = DateTime::parse_from_rfc2822(
                    item.pub_date()
                        .ok_or_else(|| PError::NoPublishDate(self.url.clone()))?,
                )
                .map_err(|_| PError::InvalidPublishDate(self.url.clone()))?
                .timestamp() as u64;
                Ok::<_, PError>((date, item))
            })
            .collect::<PResult<_>>()?;
        feeds
            .iter()
            .take_while(|(date, _)| {
                if let Some(last_post) = self.last_post {
                    date > &last_post
                } else {
                    true
                }
            })
            .map(|(_, item)| {
                Ok(Content::new(
                    item.title()
                        .ok_or_else(|| PError::NoTitle(self.url.clone()))?,
                    item.link()
                        .ok_or_else(|| PError::NoLink(self.url.clone()))?,
                    item.description()
                        .ok_or_else(|| PError::NoDescription(self.url.clone()))?,
                ))
            })
            .collect()
    }
}

impl From<Url> for Feed {
    fn from(url: Url) -> Self {
        Self::new(url)
    }
}

/// Parses the RSS feeds file. It returns a list of feeds.
/// ### File format
/// The file must contain one feed url per line.
/// The url must be valid.
/// ```text
/// https://example.com/feed
/// https://example.com/feed2
/// ```
pub fn parse_feeds(rss_feeds_file: std::path::PathBuf) -> PResult<Vec<Feed>> {
    let file = std::fs::File::open(rss_feeds_file)?;
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .map(|line| Ok(line?.parse::<Url>()?.into()))
        .collect()
}
