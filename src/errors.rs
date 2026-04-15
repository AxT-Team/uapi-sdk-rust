use thiserror::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimitPolicyEntry {
    pub name: String,
    pub quota: Option<i64>,
    pub unit: Option<String>,
    pub window_seconds: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimitStateEntry {
    pub name: String,
    pub remaining: Option<i64>,
    pub unit: Option<String>,
    pub reset_after_seconds: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseMeta {
    pub request_id: Option<String>,
    pub retry_after_seconds: Option<u64>,
    pub debit_status: Option<String>,
    pub credits_requested: Option<i64>,
    pub credits_charged: Option<i64>,
    pub credits_pricing: Option<String>,
    pub active_quota_buckets: Option<u64>,
    pub stop_on_empty: Option<bool>,
    pub rate_limit_policy_raw: Option<String>,
    pub rate_limit_raw: Option<String>,
    pub rate_limit_policies: std::collections::BTreeMap<String, RateLimitPolicyEntry>,
    pub rate_limits: std::collections::BTreeMap<String, RateLimitStateEntry>,
    pub balance_limit_cents: Option<i64>,
    pub balance_remaining_cents: Option<i64>,
    pub quota_limit_credits: Option<i64>,
    pub quota_remaining_credits: Option<i64>,
    pub visitor_quota_limit_credits: Option<i64>,
    pub visitor_quota_remaining_credits: Option<i64>,
    pub raw_headers: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("authentication failed (status {status})")]
    AuthenticationError { status: u16, message: Option<String>, request_id: Option<String>, meta: Option<ResponseMeta> },

    #[error("rate limited (status {status})")]
    RateLimitError {
        status: u16,
        message: Option<String>,
        retry_after_seconds: Option<u64>,
        request_id: Option<String>,
        meta: Option<ResponseMeta>,
    },

    #[error("insufficient credits (status {status})")]
    InsufficientCredits {
        status: u16,
        message: Option<String>,
        details: Option<serde_json::Value>,
        request_id: Option<String>,
        meta: Option<ResponseMeta>,
    },

    #[error("visitor monthly quota exhausted (status {status})")]
    VisitorMonthlyQuotaExhausted {
        status: u16,
        message: Option<String>,
        details: Option<serde_json::Value>,
        request_id: Option<String>,
        meta: Option<ResponseMeta>,
    },

    #[error("validation failed (status {status})")]
    ValidationError { status: u16, message: Option<String>, details: Option<serde_json::Value>, request_id: Option<String>, meta: Option<ResponseMeta> },

    #[error("resource not found (status {status})")]
    NotFound { status: u16, message: Option<String>, request_id: Option<String>, meta: Option<ResponseMeta> },

    #[error("server error (status {status})")]
    ServerError { status: u16, message: Option<String>, request_id: Option<String>, meta: Option<ResponseMeta> },

    #[error("api error (status {status})")]
    ApiError {
        status: u16,
        code: Option<String>,
        message: Option<String>,
        details: Option<serde_json::Value>,
        request_id: Option<String>,
        meta: Option<ResponseMeta>,
    },

    #[error("invalid header `{name}`")]
    InvalidHeader { name: String },

    #[error("invalid multipart body: {message}")]
    InvalidMultipartBody { message: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),

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
            | InsufficientCredits { status, .. }
            | VisitorMonthlyQuotaExhausted { status, .. }
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
            | InsufficientCredits { request_id, .. }
            | VisitorMonthlyQuotaExhausted { request_id, .. }
            | ValidationError { request_id, .. }
            | NotFound { request_id, .. }
            | ServerError { request_id, .. }
            | ApiError { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    pub fn meta(&self) -> Option<&ResponseMeta> {
        use Error::*;
        match self {
            AuthenticationError { meta, .. }
            | RateLimitError { meta, .. }
            | InsufficientCredits { meta, .. }
            | VisitorMonthlyQuotaExhausted { meta, .. }
            | ValidationError { meta, .. }
            | NotFound { meta, .. }
            | ServerError { meta, .. }
            | ApiError { meta, .. } => meta.as_ref(),
            _ => None,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorBody {
    pub code: Option<String>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
    pub quota: Option<serde_json::Value>,
    pub docs: Option<serde_json::Value>,
}
