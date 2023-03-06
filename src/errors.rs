#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The {0} `{1}` is not found")]
    /// First argument is the type of the file (e.g. "config file"), second argument is the path to the file.
    NotFound(String, String),
    #[error("The {0} `{1}` is not a file")]
    /// First argument is the type of the file (e.g. "config file"), second argument is the path to the file.
    NotAFile(String, String),
    #[error("The {0} `{1}` is not readable")]
    /// First argument is the type of the file (e.g. "config file"), second argument is the path to the file.
    NotReadable(String, String),
    #[error("The {0} `{1}` is empty")]
    /// First argument is the type of the file (e.g. "config file"), second argument is the path to the file.
    EmptyFile(String, String),
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
    #[error("The image takes too long to upload: {0}")]
    #[cfg(feature = "preview-image")]
    ImageTimeout(String),
    #[error("Invalid feed URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("RSS error: {0}")]
    Rss(#[from] rss::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Megalodon error: {0}")]
    Megalodon(#[from] megalodon::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
