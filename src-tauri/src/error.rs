use thiserror::Error;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Database error: {0}")]
    Database(#[from] tokio_rusqlite::Error),

    #[error("AI provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error, Clone)]
pub enum ProviderError {
    #[error("Provider '{provider}' is unavailable")]
    Unavailable { provider: String },

    #[error("HTTP request failed: {message}")]
    Http { message: String },

    #[error("Provider response could not be parsed: {0}")]
    ParseError(String),

    #[error("Context overflow: available_tokens={available}, required={required}")]
    ContextOverflow { available: usize, required: usize },

    #[error("Authentication required for provider '{provider}'")]
    AuthRequired { provider: String },

    #[error("Rate limited by provider '{provider}'")]
    RateLimited { provider: String },

    #[error("Stream interrupted: {0}")]
    StreamInterrupted(String),
}

impl From<reqwest::Error> for ProviderError {
    fn from(e: reqwest::Error) -> Self {
        ProviderError::Http {
            message: e.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, DaemonError>;
