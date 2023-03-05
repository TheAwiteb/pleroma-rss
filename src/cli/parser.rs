use super::utils::get_flag;
use crate::errors::{Error as PError, Result as PResult};
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
    /// Help flag. (Optional)
    /// If this is set, the help message will be printed and the program will exit.
    /// The flag is `-h` or `--help`.
    pub help: bool,
    /// Only new flag. (Optional)
    /// If this is set, the bot will only send new feeds.
    /// The flag is `-n` or `--only-new`.
    pub only_new: bool,
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
        if matches!(self.rss_feeds_file.to_str(), None | Some("")) {
            log::error!("Rss feeds file is not set.");
            return Err(PError::MissingArgument("--feed-file".to_string()));
        } else if !self.rss_feeds_file.exists() {
            log::error!("Rss feeds file does not exist.");
            return Err(PError::FeedsFileNotFound(
                self.rss_feeds_file.display().to_string(),
            ));
        } else if !self.rss_feeds_file.is_file() {
            log::error!("Rss feeds file is not a file.");
            return Err(PError::FeedsFileNotAFile(
                self.rss_feeds_file.display().to_string(),
            ));
        } else if self.rss_feeds_file.metadata()?.len() == 0 {
            log::error!("Rss feeds file is empty.");
            return Err(PError::FeedsFileEmpty(
                self.rss_feeds_file.display().to_string(),
            ));
        } else if std::fs::File::open(&self.rss_feeds_file).is_err() {
            log::error!("Rss feeds file is not readable.");
            return Err(PError::FeedsFileNotReadable(
                self.rss_feeds_file.display().to_string(),
            ));
        }
        if self.pleroma_base_url.to_string() == "https://example.com/" {
            log::error!("Pleroma base url is not set.");
            return Err(PError::MissingArgument("--base-url".to_string()));
        }
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
            version: false,
        }
    }
}
