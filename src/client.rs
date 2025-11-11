use crate::errors::{ApiErrorBody, Error};
use crate::services::{
    ClipzyZaiXianJianTieBanService, ConvertService, DailyService, GameService, ImageService,
    MinGanCiShiBieService, MiscService, NetworkService, PoemService, RandomService, SocialService,
    StatusService, TextService, TranslateService, WebparseService, ZhiNengSouSuoService,
};
use crate::Result;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, RETRY_AFTER, USER_AGENT};
use reqwest::StatusCode;
use std::time::Duration;
use tracing::{debug, instrument};
use url::Url;

static DEFAULT_BASE: &str = "https://uapis.cn/api/v1";
static DEFAULT_UA: &str = "uapi-sdk-rust/0.1.2";
static DEFAULT_BASE_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(DEFAULT_BASE).expect("valid default base"));

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) api_key: Option<String>,
    pub(crate) user_agent: String,
}

impl Client {
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("reqwest client");
        Self {
            http,
            base_url: DEFAULT_BASE_URL.clone(),
            api_key: Some(api_key.into()),
            user_agent: DEFAULT_UA.to_string(),
        }
    }

    pub fn from_env() -> Option<Self> {
        let token = std::env::var("UAPI_TOKEN").ok()?;
        let mut cli = Self::new(token);
        if let Ok(base) = std::env::var("UAPI_BASE_URL") {
            if let Ok(url) = Url::parse(&base) {
                cli.base_url = url;
            }
        }
        Some(cli)
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
    pub fn clipzy_zai_xian_jian_tie_ban(&self) -> ClipzyZaiXianJianTieBanService<'_> {
        ClipzyZaiXianJianTieBanService { client: self }
    }
    pub fn convert(&self) -> ConvertService<'_> {
        ConvertService { client: self }
    }
    pub fn daily(&self) -> DailyService<'_> {
        DailyService { client: self }
    }
    pub fn game(&self) -> GameService<'_> {
        GameService { client: self }
    }
    pub fn image(&self) -> ImageService<'_> {
        ImageService { client: self }
    }
    pub fn misc(&self) -> MiscService<'_> {
        MiscService { client: self }
    }
    pub fn network(&self) -> NetworkService<'_> {
        NetworkService { client: self }
    }
    pub fn poem(&self) -> PoemService<'_> {
        PoemService { client: self }
    }
    pub fn random(&self) -> RandomService<'_> {
        RandomService { client: self }
    }
    pub fn social(&self) -> SocialService<'_> {
        SocialService { client: self }
    }
    pub fn status(&self) -> StatusService<'_> {
        StatusService { client: self }
    }
    pub fn text(&self) -> TextService<'_> {
        TextService { client: self }
    }
    pub fn translate(&self) -> TranslateService<'_> {
        TranslateService { client: self }
    }
    pub fn webparse(&self) -> WebparseService<'_> {
        WebparseService { client: self }
    }
    pub fn min_gan_ci_shi_bie(&self) -> MinGanCiShiBieService<'_> {
        MinGanCiShiBieService { client: self }
    }
    pub fn zhi_neng_sou_suo(&self) -> ZhiNengSouSuoService<'_> {
        ZhiNengSouSuoService { client: self }
    }

    #[instrument(skip(self, headers, query), fields(method=%method, path=%path))]
    pub(crate) async fn request_json<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<Vec<(String, String)>>,
        json_body: Option<serde_json::Value>,
    ) -> Result<T> {
        let url = self.base_url.join(path)?;
        let mut req = self.http.request(method.clone(), url.clone());

        let mut merged = HeaderMap::new();
        merged.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_UA));
        if let Some(t) = &self.api_key {
            let value = format!("Bearer {}", t);
            if let Ok(h) = HeaderValue::from_str(&value) {
                merged.insert(AUTHORIZATION, h);
            }
        }
        if let Some(h) = headers {
            merged.extend(h);
        }
        req = req.headers(merged);

        if let Some(q) = query {
            req = req.query(&q);
        }
        if let Some(body) = json_body {
            req = req.json(&body);
        }

        debug!("request {}", url);
        let resp = req.send().await?;
        self.handle_json_response(resp).await
    }

    async fn handle_json_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        let status = resp.status();
        let req_id = find_request_id(resp.headers());
        let retry_after = parse_retry_after(resp.headers());
        if status.is_success() {
            return Ok(resp.json::<T>().await?);
        }
        let text = resp.text().await.unwrap_or_default();
        let parsed = serde_json::from_str::<ApiErrorBody>(&text).ok();
        let msg = parsed
            .as_ref()
            .and_then(|b| b.message.clone())
            .or_else(|| non_empty(text.clone()));
        let code = parsed.as_ref().and_then(|b| b.code.clone());
        let details = parsed.as_ref().and_then(|b| b.details.clone());
        Err(map_status_to_error(
            status,
            code,
            msg,
            details,
            req_id,
            retry_after,
        ))
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<Url>,
    timeout: Option<Duration>,
    client: Option<reqwest::Client>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    pub fn api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    pub fn base_url(mut self, base: Url) -> Self {
        self.base_url = Some(base);
        self
    }
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Some(Duration::from_secs(secs));
        self
    }
    pub fn user_agent<T: Into<String>>(mut self, ua: T) -> Self {
        self.user_agent = Some(ua.into());
        self
    }
    pub fn http_client(mut self, cli: reqwest::Client) -> Self {
        self.client = Some(cli);
        self
    }

    pub fn build(self) -> Result<Client> {
        let http = if let Some(cli) = self.client {
            cli
        } else {
            reqwest::Client::builder()
                .timeout(self.timeout.unwrap_or(Duration::from_secs(20)))
                .build()?
        };
        Ok(Client {
            http,
            base_url: self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.clone()),
            api_key: self.api_key,
            user_agent: self.user_agent.unwrap_or_else(|| DEFAULT_UA.to_string()),
        })
    }
}

fn find_request_id(headers: &HeaderMap) -> Option<String> {
    const CANDIDATES: &[&str] = &["x-request-id", "x-amzn-requestid", "traceparent"];
    for key in CANDIDATES {
        if let Some(v) = headers.get(*key) {
            if let Ok(text) = v.to_str() {
                return Some(text.to_string());
            }
        }
    }
    None
}

fn parse_retry_after(headers: &HeaderMap) -> Option<u64> {
    headers
        .get(RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
}

fn non_empty(s: String) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn map_status_to_error(
    status: StatusCode,
    code: Option<String>,
    message: Option<String>,
    details: Option<serde_json::Value>,
    request_id: Option<String>,
    retry_after: Option<u64>,
) -> Error {
    let s = status.as_u16();
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::AuthenticationError {
            status: s,
            message,
            request_id,
        },
        StatusCode::TOO_MANY_REQUESTS => Error::RateLimitError {
            status: s,
            message,
            retry_after_seconds: retry_after,
            request_id,
        },
        StatusCode::NOT_FOUND => Error::NotFound {
            status: s,
            message,
            request_id,
        },
        StatusCode::BAD_REQUEST => Error::ValidationError {
            status: s,
            message,
            details,
            request_id,
        },
        _ if status.is_server_error() => Error::ServerError {
            status: s,
            message,
            request_id,
        },
        _ if status.is_client_error() => Error::ApiError {
            status: s,
            code,
            message,
            details,
            request_id,
        },
        _ => Error::ApiError {
            status: s,
            code,
            message,
            details,
            request_id,
        },
    }
}
