use crate::{errors::Result as Presult, utils};
use clap::Parser;
use std::path::PathBuf;

/// The CLI parser. This is the main entry point for the CLI. It parses the CLI arguments.
#[derive(Parser)]
#[command(version, about, verbatim_doc_comment, long_about = None)]
pub struct Cli {
    /// Your bot access token.
    #[arg(short, long, value_name = "TOKEN")]
    pub access_token: String,
    /// The file that contains the feeds.
    #[arg(short, long, value_name = "PATH")]
    pub feeds_file: PathBuf,
    /// The server URL.
    #[arg(short, long, value_name = "URL")]
    pub base_url: url::Url,
    /// The sleep time between each feed in seconds.
    #[arg(short = 's', long, value_name = "SECONDS", default_value = "1")]
    pub items_sleep: u64,
    /// The sleep time after end all feeds (wait for new items) in seconds.
    #[arg(short, long, value_name = "SECONDS", default_value = "30")]
    pub watting_new: u64,
    /// The HTML template for the preview image.
    #[cfg(feature = "preview-image")]
    #[arg(short = 't', long, value_name = "PATH")]
    pub preview_image_template: PathBuf,
    /// The default image if the feed does not have an image.
    #[cfg(feature = "preview-image")]
    #[arg(short = 'i', long, value_name = "PATH")]
    pub default_preview_image: PathBuf,
    /// Use Mastodon instead of Pleroma.
    #[arg(short = 'm', long = "mastodon")]
    pub is_mastodon: bool,
    /// Only post new items. Without this flag, the bot will post all the items in the feed.
    #[arg(short = 'n', long)]
    pub only_new: bool,
    /// Do not post anything, will print the items that would be posted.
    #[arg(short, long)]
    pub dry_run: bool,
}

impl Cli {
    /// Check the CLI arguments.
    pub fn check(&self) -> Presult<()> {
        utils::check_file("feeds file", &self.feeds_file)?;
        #[cfg(feature = "preview-image")]
        {
            utils::check_file("image teplate", &self.preview_image_template)?;
            utils::check_file("default preview image", &self.default_preview_image)?;
        }
        Ok(())
    }
}

#[cfg(not(feature = "preview-image"))]
impl std::fmt::Debug for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cli")
            .field("access_token", &"***")
            .field("feeds_file", &self.feeds_file)
            .field("base_url", &self.base_url)
            .field("items_sleep", &self.items_sleep)
            .field("watting_new", &self.watting_new)
            .field("only_new", &self.only_new)
            .field("dry_run", &self.dry_run)
            .finish()
    }
}

#[cfg(feature = "preview-image")]
impl std::fmt::Debug for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cli")
            .field("access_token", &"***")
            .field("feeds_file", &self.feeds_file)
            .field("base_url", &self.base_url)
            .field("items_sleep", &self.items_sleep)
            .field("watting_new", &self.watting_new)
            .field("preview_image_template", &self.preview_image_template)
            .field("default_preview_image", &self.default_preview_image)
            .field("only_new", &self.only_new)
            .field("dry_run", &self.dry_run)
            .finish()
    }
}
