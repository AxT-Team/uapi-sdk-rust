use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("authentication failed (status {status})")]
    AuthenticationError { status: u16, message: Option<String>, request_id: Option<String> },

    #[error("rate limited (status {status})")]
    RateLimitError {
        status: u16,
        message: Option<String>,
        retry_after_seconds: Option<u64>,
        request_id: Option<String>,
    },

    #[error("validation failed (status {status})")]
    ValidationError { status: u16, message: Option<String>, details: Option<serde_json::Value>, request_id: Option<String> },

    #[error("resource not found (status {status})")]
    NotFound { status: u16, message: Option<String>, request_id: Option<String> },

    #[error("server error (status {status})")]
    ServerError { status: u16, message: Option<String>, request_id: Option<String> },

    #[error("api error (status {status})")]
    ApiError {
        status: u16,
        code: Option<String>,
        message: Option<String>,
        details: Option<serde_json::Value>,
        request_id: Option<String>,
    },

    #[error(transparent)]
    Transport(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),
}

impl Error {
    pub fn status(&self) -> Option<u16> {
        use Error::*;
        match self {
            AuthenticationError { status, .. }
            | RateLimitError { status, .. }
            | ValidationError { status, .. }
            | NotFound { status, .. }
            | ServerError { status, .. }
            | ApiError { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        use Error::*;
        match self {
            AuthenticationError { request_id, .. }
            | RateLimitError { request_id, .. }
            | ValidationError { request_id, .. }
            | NotFound { request_id, .. }
            | ServerError { request_id, .. }
            | ApiError { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorBody {
    pub code: Option<String>,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}
