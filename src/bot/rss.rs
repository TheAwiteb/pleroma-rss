use crate::{
    config::Config,
    errors::{Error as PError, Result as PResult},
    utils::remove_html_tags,
};
use chrono::DateTime;
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

    #[allow(unused)]
    pub async fn post(&self, config: &Config) -> PResult<()> {
        log::info!("Posting: {}", self.title);

        let description = remove_html_tags(&self.description);
        let link = urlencoding::decode(self.link.as_str()).unwrap();
        let base_url = config.base_url.as_str();
        megalodon::generator(
            megalodon::SNS::Pleroma,
            // Remove the last slash.
            base_url
                .chars()
                .take(base_url.len() - 1)
                .collect::<String>(),
            Some(config.bot_token.to_owned()),
            None,
        )
        .post_status(format!("{}\n\n{description}\n\n{link}", self.title), None)
        .await?;
        log::info!("Posted: {} successfully.", self.title);
        Ok(())
    }
}

impl Feed {
    /// Creates a new feed.
    /// If `only_new` is true, it will only return new posts when checking with [`check`].
    ///
    /// [`check`]: #method.check
    pub fn new(url: Url, only_new: bool) -> Self {
        Self {
            url,
            last_post: if only_new {
                Some(chrono::Utc::now().timestamp() as u64)
            } else {
                None
            },
        }
    }

    /// Checks if a new post has been made.
    /// If a new post has been made, it returns the post content.
    pub async fn check(&mut self) -> PResult<Vec<Content>> {
        log::info!("Checking feed: {}", self.url);
        let feed = reqwest::get(self.url.as_str()).await?.text().await?;
        log::info!("Feed: {} has been downloaded.", self.url);
        let feed = feed.parse::<rss::Channel>()?;
        log::info!("Feed: {} has been parsed.", self.url);
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
            .filter(|(date, _)| {
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
