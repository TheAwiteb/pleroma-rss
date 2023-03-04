#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Unknown argument `{0}`")]
    UnknownArgument(String),
    #[error("The argument `{0}` is required")]
    MissingArgument(String),
    #[error("`{0}` argument is missing a value.")]
    UncompletedArgument(String),
    #[error("In argument `{0}`: {1}")]
    Parsing(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;
