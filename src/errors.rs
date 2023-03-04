#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown argument `{0}`")]
    UnknownArgument(String),
    #[error("The argument `{0}` is required")]
    MissingArgument(String),
    #[error("`{0}` argument is missing a value.")]
    UncompletedArgument(String),
    #[error("In argument `{0}`: {1}")]
    Parsing(String, String),
    #[error("The feeds file `{0}` is not found")]
    FeedsFileNotFound(String),
    #[error("The feeds file `{0}` is not a file")]
    FeedsFileNotAFile(String),
    #[error("The feeds file `{0}` is not readable")]
    FeedsFileNotReadable(String),
    #[error("The feeds file `{0}` is empty")]
    FeedsFileEmpty(String),
    #[error("There is no publish date in items of the feed `{0}`")]
    NoPublishDate(url::Url),
    #[error("The publish date of the feed `{0}` is invalid")]
    InvalidPublishDate(url::Url),
    #[error("There is no title in items of the feed `{0}`")]
    NoTitle(url::Url),
    #[error("There is no link in items of the feed `{0}`")]
    NoLink(url::Url),
    #[error("There is no description in items of the feed `{0}`")]
    NoDescription(url::Url),
    #[error("Invalid feed URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("RSS error: {0}")]
    RssError(#[from] rss::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Megalodon error: {0}")]
    MegalodonError(#[from] megalodon::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
