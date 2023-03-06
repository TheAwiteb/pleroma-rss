#[cfg(feature = "preview-image")]
use super::image::get_image_id;
use crate::{
    config::Config,
    errors::{Error as PError, Result as PResult},
    utils::remove_html_tags,
};
use chrono::DateTime;
use megalodon::megalodon::PostStatusInputOptions;
#[cfg(feature = "preview-image")]
use std::fs;
#[cfg(feature = "preview-image")]
use std::path::PathBuf;
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

/// The content of a feed.
#[derive(Debug)]
pub struct Content {
    #[cfg(feature = "preview-image")]
    pub uuid: String,
    pub title: String,
    pub link: String,
    #[cfg(feature = "preview-image")]
    pub image_url: Option<String>,
    pub description: String,
}

impl Content {
    pub fn new(
        title: impl AsRef<str>,
        link: impl AsRef<str>,
        description: impl AsRef<str>,
        #[cfg(feature = "preview-image")] image: Option<String>,
    ) -> Self {
        Self {
            #[cfg(feature = "preview-image")]
            uuid: uuid::Uuid::new_v4().to_string(),
            title: title.as_ref().to_string(),
            link: link.as_ref().to_string(),
            #[cfg(feature = "preview-image")]
            image_url: image,
            description: description.as_ref().to_string(),
        }
    }

    pub async fn post(&self, config: &Config) -> PResult<()> {
        log::info!("Posting: {}", self.title);

        let base_url = config.base_url.as_str();
        megalodon::generator(
            config.sns(),
            // Remove the last slash.
            base_url
                .chars()
                .take(base_url.len() - 1)
                .collect::<String>(),
            Some(config.bot_token.to_owned()),
            None,
        )
        .post_status(
            format!("{}\n\n{}\n\n{}", self.title, self.description, self.link),
            self.options(config).await.as_ref().ok(),
        )
        .await?;
        log::info!("Posted: {} successfully.", self.title);
        Ok(())
    }

    /// Returns the options for the post.
    /// Will contain the image if the `with-image` feature is enabled.
    #[cfg(feature = "preview-image")]
    pub async fn options(&self, config: &Config) -> PResult<PostStatusInputOptions> {
        let image = self.image_url.clone().unwrap_or_else(|| {
            fs::canonicalize(&config.default_preview_image)
                .expect("This should not happen.")
                .display()
                .to_string()
        });
        let preview = self.create_preview(&image, config)?;
        log::debug!("Image src: {}", image);
        log::debug!("Feed preview: {}", preview.display());
        let image_id = get_image_id(preview, config).await?;
        fs::remove_file(format!("{}.html", self.uuid))?;
        fs::remove_file(format!("{}.png", self.uuid))?;
        log::info!("Image preview and html template removed");
        Ok(PostStatusInputOptions {
            media_ids: Some(vec![image_id]),
            ..Default::default()
        })
    }

    /// Returns the options for the post.
    /// Will be empty if the `with-image` feature is disabled.
    #[cfg(not(feature = "preview-image"))]
    pub async fn options(&self, _config: &Config) -> PResult<PostStatusInputOptions> {
        Ok(PostStatusInputOptions::default())
    }

    /// Create a preview image.
    #[cfg(feature = "preview-image")]
    pub fn create_preview(&self, image: &str, config: &Config) -> PResult<PathBuf> {
        log::debug!(
            "Creating preview for: {}. The template is: {}",
            self.title,
            config.preview_image_template.display()
        );
        let html_content = fs::read_to_string(config.preview_image_template.as_path())?;
        log::info!("Template readded successfully.");
        let html_content = html_content
            .replace("{{title}}", &self.title)
            .replace(
                "{{description}}",
                &self
                    .description
                    .chars()
                    .enumerate()
                    .take_while(|(idx, c)| idx < &320 || c != &' ')
                    .map(|(_, c)| c)
                    .collect::<String>(),
            )
            .replace("{{link}}", &self.link)
            .replace("{{image-src}}", image);
        log::info!("Template replaced successfully.");
        let content_path = format!("{}.html", self.uuid);
        let image_path = format!("{}.png", self.uuid);
        fs::write(&content_path, html_content)?;
        fs::File::create(&image_path)?;
        log::info!("Template written successfully.");
        std::process::Command::new("wkhtmltoimage")
            .arg("--enable-local-file-access")
            .arg("--enable-smart-width")
            .arg(content_path)
            .arg(&image_path)
            .spawn()?
            .wait()?;
        fs::canonicalize(&image_path).map_err(From::from)
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
                let status = if let Some(last_post) = self.last_post {
                    date > &last_post
                } else {
                    true
                };
                if status {
                    log::debug!("New post found: {}", date);
                    self.last_post = Some(*date);
                }
                status
            })
            .map(|(_, item)| {
                Ok(Content::new(
                    item.title()
                        .ok_or_else(|| PError::NoTitle(self.url.clone()))?,
                    urlencoding::decode(
                        item.link()
                            .ok_or_else(|| PError::NoLink(self.url.clone()))?,
                    )
                    .unwrap(),
                    remove_html_tags(
                        item.description()
                            .ok_or_else(|| PError::NoDescription(self.url.clone()))?,
                    ),
                    #[cfg(feature = "preview-image")]
                    item.extensions().get("media").and_then(|ext| {
                        ext.get("content").and_then(|content| {
                            content
                                .iter()
                                .next()
                                .and_then(|c| c.attrs.get("url").map(ToString::to_string))
                        })
                    }),
                ))
            })
            .collect()
    }
}
