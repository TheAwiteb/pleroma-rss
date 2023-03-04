use std::str::FromStr;

use crate::errors::{Error as PError, Result as PResult};

pub const fn version_message() -> &'static str {
    concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"))
}

pub const fn help_message() -> &'static str {
    r#"USAGE:
    pleroma-rss [FLAGS] [OPTIONS]
FLAGS:
    -h, --help    Prints help information
    -V, --version Prints version information
OPTIONS:
    -a, --access-token <access-token> The access token of the bot account
    -b, --base-url <base-url>         The base url of the pleroma instance
    -f, --feed-file <feed-file>       The path to the feeds file "#
}

/// Gets the value of a flag. If the flag is not present, an error is returned.
pub fn get_flag<T: FromStr>(flag: &str, args: &[String]) -> PResult<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug + std::fmt::Display,
{
    log::debug!("Getting flag: {}", flag);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if arg == flag {
            if let Some(value) = args.next() {
                log::debug!("Found flag: {} with value: {}", flag, value);
                return T::from_str(value)
                    .map_err(|err| PError::Parsing(flag.to_string(), err.to_string()));
            } else {
                log::error!("Missing value for flag: {}", flag);
                return Err(PError::UncompletedArgument(flag.to_string()));
            }
        }
    }
    Err(PError::MissingArgument(flag.to_string()))
}
