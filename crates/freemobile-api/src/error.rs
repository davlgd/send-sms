use thiserror::Error;

#[derive(Error, Debug)]
pub enum FreeMobileError {
    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("Too many requests sent (rate limit exceeded)")]
    TooManyRequests,

    #[error("Access denied - check your FreeMobile subscription")]
    AccessDenied,

    #[error("FreeMobile server error")]
    ServerError,

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Message is empty")]
    EmptyMessage,

    #[error("Invalid message format: {0}")]
    InvalidMessage(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error occurred")]
    Unknown,
}

impl FreeMobileError {
    pub fn from_status_code(status: u16) -> Self {
        use crate::constants::status_codes::*;

        match status {
            INVALID_CREDENTIALS => Self::InvalidCredentials,
            TOO_MANY_REQUESTS => Self::TooManyRequests,
            ACCESS_DENIED => Self::AccessDenied,
            SERVER_ERROR => Self::ServerError,
            _ => Self::Unknown,
        }
    }
}
