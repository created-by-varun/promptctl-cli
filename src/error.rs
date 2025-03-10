use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Clipboard operation failed: {0}")]
    Clipboard(#[from] arboard::Error),

    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Claude API error: {0}")]
    Claude(String),

    #[error("API key not found: {0}")]
    MissingApiKey(String),

    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Dialog error: {0}")]
    DialogError(#[from] dialoguer::Error),
}
