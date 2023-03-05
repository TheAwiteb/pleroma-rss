use super::utils::get_flag;
use crate::{
    errors::{Error as PError, Result as PResult},
    utils,
};
use std::path::PathBuf;

/// The CLI parser. This is the main entry point for the CLI. It parses the CLI arguments.
#[derive(Debug)]
pub struct Cli {
    /// The number of arguments.
    pub(crate) argc: usize,
    /// The bot token. (Required)
    /// The flag is `-a` or `--access-token`.
    pub bot_token: String,
    /// Rss feeds file path. (Required)
    /// This is the file that contains the rss feeds. Separated by newlines.
    /// The flag is `-f` or `--feeds-file`.
    pub rss_feeds_file: PathBuf,
    /// Base url of Pleroma instance. (Required)
    /// This is the base url of the Pleroma instance that the bot will post to.
    /// The flag is `-b` or `--base-url`.
    pub pleroma_base_url: url::Url,
    /// Preview image html template. (Required)
    /// This is the html template that will be used to generate the preview image.
    /// The flag is `-t` or `--preview-image-template`.
    #[cfg(feature = "with-image")]
    pub preview_image_template: PathBuf,
    /// Default image of the preview image. (Required)
    /// This is the default image that will be used if the feed item does not have an image.
    /// The flag is `-i` or `--default-preview-image`.
    #[cfg(feature = "with-image")]
    pub default_preview_image: PathBuf,
    /// Help flag. (Optional)
    /// If this is set, the help message will be printed and the program will exit.
    /// The flag is `-h` or `--help`.
    pub help: bool,
    /// Only new flag. (Optional)
    /// If this is set, the bot will only send new feeds.
    /// The flag is `-n` or `--only-new`.
    pub only_new: bool,
    /// Dry run flag. (Optional)
    /// If this is set, the bot will only print the feed items that would be posted.
    /// The flag is `-d` or `--dry-run`.
    pub dry_run: bool,
    /// Version flag. (Optional)
    /// If this is set, the version will be printed and the program will exit.
    /// The flag is `-V` or `--version`.
    pub version: bool,
}

impl Cli {
    /// Parses the CLI arguments.
    pub fn parse(args: Vec<String>) -> PResult<Self> {
        // Parse the CLI arguments.
        let mut cli = Cli::default();
        for arg in args.iter() {
            if arg == "-h" || arg == "--help" {
                log::debug!("Help flag is set.");
                cli.help = true;
                cli.argc += 1;
            } else if arg == "-V" || arg == "--version" {
                log::debug!("Version flag is set.");
                cli.version = true;
                cli.argc += 1;
            } else if arg == "-n" || arg == "----only-new" {
                log::debug!("Only new flag is set.");
                cli.only_new = true;
                cli.argc += 1;
            } else if arg == "-d" || arg == "--dry-run" {
                log::debug!("Dry run flag is set.");
                cli.dry_run = true;
                cli.argc += 1;
            } else if arg == "-a" || arg == "--access-token" {
                cli.bot_token = get_flag(arg, &args)?;
                log::debug!(
                    "Bot token is set to: {}...",
                    &cli.bot_token.chars().take(5).collect::<String>()
                );
                cli.argc += 1;
            } else if arg == "-f" || arg == "--feed-file" {
                cli.rss_feeds_file = get_flag(arg, &args)?;
                log::debug!("Rss feeds file is set to: {}", cli.rss_feeds_file.display());
                cli.argc += 1;
            } else if arg == "-b" || arg == "--base-url" {
                cli.pleroma_base_url = get_flag(arg, &args)?;
                log::debug!("Pleroma base url is set to: {}", cli.pleroma_base_url);
                cli.argc += 1;
            } else if cfg!(feature = "with-image")
                && (arg == "-t" || arg == "--preview-image-template")
            {
                cli.preview_image_template = get_flag(arg, &args)?;
                log::debug!(
                    "Preview image template is set to: {}",
                    cli.preview_image_template.display()
                );
                cli.argc += 1;
            } else if cfg!(feature = "with-image")
                && (arg == "-i" || arg == "--default-preview-image")
            {
                cli.default_preview_image = get_flag(arg, &args)?;
                log::debug!(
                    "Default preview image is set to: {}",
                    cli.default_preview_image.display()
                );
                cli.argc += 1;
            } else if arg.starts_with('-') {
                log::error!("Unknown argument: {}", arg);
                return Err(PError::UnknownArgument(arg.to_string()));
            }
        }
        cli.check_required_args()
    }

    /// Checks if all required arguments are present.
    fn check_required_args(mut self) -> PResult<Self> {
        // If the help or version flag is set, the other arguments are not required.
        if self.help || self.version {
            return Ok(self);
        } else if self.argc == 0 {
            log::info!("No arguments are passed. Printing help message.");
            self.help = true;
            return Ok(self);
        }

        if self.bot_token.is_empty() {
            log::error!("Bot token is not set.");
            return Err(PError::MissingArgument("--access-token".to_string()));
        }
        if self.rss_feeds_file.to_str() == Some("") {
            log::error!("Rss feeds file is not set.");
            return Err(PError::MissingArgument("--feed-file".to_string()));
        }
        if self.pleroma_base_url.to_string() == "https://example.com/" {
            log::error!("Pleroma base url is not set.");
            return Err(PError::MissingArgument("--base-url".to_string()));
        }
        #[cfg(feature = "with-image")]
        if self.preview_image_template.to_str() == Some("") {
            log::error!("Preview image template is not set.");
            return Err(PError::MissingArgument(
                "--preview-image-template".to_string(),
            ));
        }
        #[cfg(feature = "with-image")]
        if self.default_preview_image.to_str() == Some("") {
            log::error!("Default preview image is not set.");
            return Err(PError::MissingArgument(
                "--default-preview-image".to_string(),
            ));
        }
        utils::check_file(&self.rss_feeds_file)?;
        #[cfg(feature = "with-image")]
        utils::check_file(&self.preview_image_template)?;
        #[cfg(feature = "with-image")]
        utils::check_file(&self.default_preview_image)?;

        Ok(self)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            argc: 0,
            bot_token: String::new(),
            rss_feeds_file: PathBuf::new(),
            pleroma_base_url: url::Url::parse("https://example.com").unwrap(),
            help: false,
            only_new: false,
            dry_run: false,
            version: false,
            #[cfg(feature = "with-image")]
            preview_image_template: PathBuf::new(),
            #[cfg(feature = "with-image")]
            default_preview_image: PathBuf::new(),
        }
    }
}
