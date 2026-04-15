use crate::errors::{ApiErrorBody, Error, RateLimitPolicyEntry, RateLimitStateEntry, ResponseMeta};
use crate::services::{ClipzyZaiXianJianTieBanService,ConvertService,DailyService,GameService,ImageService,MiscService,NetworkService,PoemService,RandomService,SocialService,StatusService,TextService,TranslateService,WebparseService,MinGanCiShiBieService,ZhiNengSouSuoService
};
use crate::Result;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, RETRY_AFTER, USER_AGENT};
use reqwest::multipart::{Form, Part};
use reqwest::StatusCode;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, instrument};
use url::Url;

static DEFAULT_BASE: &str = "https://uapis.cn/";
static DEFAULT_UA: &str = "uapi-sdk-rust/0.1.0";
static DEFAULT_BASE_URL: Lazy<Url> = Lazy::new(|| Url::parse(DEFAULT_BASE).expect("valid default base"));

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) api_key: Option<String>,
    pub(crate) user_agent: String,
    pub(crate) disable_cache_default: bool,
    pub(crate) last_response_meta: Arc<RwLock<Option<ResponseMeta>>>,
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
            disable_cache_default: false,
            last_response_meta: Arc::new(RwLock::new(None)),
        }
    }

    pub fn from_env() -> Option<Self> {
        let token = std::env::var("UAPI_TOKEN").ok()?;
        let mut cli = Self::new(token);
        if let Ok(base) = std::env::var("UAPI_BASE_URL") {
            if let Ok(url) = Url::parse(&base) {
                cli.base_url = normalize_base_url(url);
            }
        }
        Some(cli)
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn last_response_meta(&self) -> Option<ResponseMeta> {
        self.last_response_meta.read().ok().and_then(|guard| guard.clone())
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

    fn create_request(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<Vec<(String, String)>>,
        disable_cache: Option<bool>,
    ) -> Result<(Url, reqwest::RequestBuilder)> {
        let clean_path = path.trim_start_matches('/');
        let url = self.base_url.join(clean_path)?;
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

        if let Some(q) = self.apply_cache_control(&method, query, disable_cache) {
            req = req.query(&q);
        }
        Ok((url, req))
    }

    #[instrument(skip(self, headers, query), fields(method=%method, path=%path))]
    pub(crate) async fn request_json<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<Vec<(String, String)>>,
        json_body: Option<serde_json::Value>,
        disable_cache: Option<bool>,
    ) -> Result<T> {
        let (url, mut req) = self.create_request(method, path, headers, query, disable_cache)?;
        if let Some(body) = json_body {
            req = req.json(&body);
        }

        debug!("request {}", url);
        let resp = req.send().await?;
        self.handle_json_response(resp).await
    }

    #[instrument(skip(self, headers, query), fields(method=%method, path=%path))]
    pub(crate) async fn request_bytes(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<Vec<(String, String)>>,
        json_body: Option<serde_json::Value>,
        disable_cache: Option<bool>,
    ) -> Result<Vec<u8>> {
        let (url, mut req) = self.create_request(method, path, headers, query, disable_cache)?;
        if let Some(body) = json_body {
            req = req.json(&body);
        }

        debug!("request {}", url);
        let resp = req.send().await?;
        self.handle_bytes_response(resp).await
    }

    #[instrument(skip(self, headers, query, multipart_body), fields(method=%method, path=%path))]
    pub(crate) async fn request_multipart_json<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<Vec<(String, String)>>,
        multipart_body: Option<serde_json::Value>,
        file_fields: &[&str],
        disable_cache: Option<bool>,
    ) -> Result<T> {
        let (url, mut req) = self.create_request(method, path, headers, query, disable_cache)?;
        req = req.multipart(build_multipart_form(multipart_body, file_fields)?);

        debug!("request {}", url);
        let resp = req.send().await?;
        self.handle_json_response(resp).await
    }

    #[instrument(skip(self, headers, query, multipart_body), fields(method=%method, path=%path))]
    pub(crate) async fn request_multipart_bytes(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<Vec<(String, String)>>,
        multipart_body: Option<serde_json::Value>,
        file_fields: &[&str],
        disable_cache: Option<bool>,
    ) -> Result<Vec<u8>> {
        let (url, mut req) = self.create_request(method, path, headers, query, disable_cache)?;
        req = req.multipart(build_multipart_form(multipart_body, file_fields)?);

        debug!("request {}", url);
        let resp = req.send().await?;
        self.handle_bytes_response(resp).await
    }

    async fn handle_json_response<T: serde::de::DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        let status = resp.status();
        let meta = extract_response_meta(resp.headers());
        if let Ok(mut guard) = self.last_response_meta.write() {
            *guard = Some(meta.clone());
        }
        let req_id = meta.request_id.clone();
        let retry_after = meta.retry_after_seconds;
        if status.is_success() {
            return Ok(resp.json::<T>().await?);
        }
        let text = resp.text().await.unwrap_or_default();
        let parsed = serde_json::from_str::<ApiErrorBody>(&text).ok();
        let msg = parsed.as_ref().and_then(|b| b.message.clone()).or_else(|| non_empty(text.clone()));
        let code = parsed
            .as_ref()
            .and_then(|b| b.code.clone().or_else(|| b.error.clone()));
        let details = parsed
            .as_ref()
            .and_then(|b| b.details.clone().or_else(|| b.quota.clone()).or_else(|| b.docs.clone()));
        Err(map_status_to_error(status, code, msg, details, req_id, retry_after, Some(meta)))
    }

    async fn handle_bytes_response(&self, resp: reqwest::Response) -> Result<Vec<u8>> {
        let status = resp.status();
        let meta = extract_response_meta(resp.headers());
        if let Ok(mut guard) = self.last_response_meta.write() {
            *guard = Some(meta.clone());
        }
        let req_id = meta.request_id.clone();
        let retry_after = meta.retry_after_seconds;
        if status.is_success() {
            return Ok(resp.bytes().await?.to_vec());
        }
        let text = resp.text().await.unwrap_or_default();
        let parsed = serde_json::from_str::<ApiErrorBody>(&text).ok();
        let msg = parsed.as_ref().and_then(|b| b.message.clone()).or_else(|| non_empty(text.clone()));
        let code = parsed
            .as_ref()
            .and_then(|b| b.code.clone().or_else(|| b.error.clone()));
        let details = parsed
            .as_ref()
            .and_then(|b| b.details.clone().or_else(|| b.quota.clone()).or_else(|| b.docs.clone()));
        Err(map_status_to_error(status, code, msg, details, req_id, retry_after, Some(meta)))
    }

    fn apply_cache_control(
        &self,
        method: &reqwest::Method,
        query: Option<Vec<(String, String)>>,
        disable_cache: Option<bool>,
    ) -> Option<Vec<(String, String)>> {
        if *method != reqwest::Method::GET {
            return query;
        }
        if let Some(items) = &query {
            if items.iter().any(|(key, _)| key == "_t") {
                return query;
            }
        }
        let effective_disable_cache = disable_cache.unwrap_or(self.disable_cache_default);
        if !effective_disable_cache {
            return query;
        }
        let mut next = query.unwrap_or_default();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .to_string();
        next.push(("_t".to_string(), now));
        Some(next)
    }
}

fn build_multipart_form(
    body: Option<serde_json::Value>,
    file_fields: &[&str],
) -> Result<Form> {
    let mut form = Form::new();
    let Some(body) = body else {
        return Ok(form);
    };
    let Some(object) = body.as_object() else {
        return Err(Error::InvalidMultipartBody {
            message: "multipart body must be a JSON object".to_string(),
        });
    };
    for (key, value) in object {
        if value.is_null() {
            continue;
        }
        if file_fields.iter().any(|field| field == &key.as_str()) {
            form = form.part(key.clone(), json_value_to_file_part(key, value)?);
        } else {
            form = form.text(key.clone(), json_value_to_form_text(value));
        }
    }
    Ok(form)
}

fn json_value_to_form_text(value: &serde_json::Value) -> String {
    value.as_str().map(str::to_owned).unwrap_or_else(|| value.to_string())
}

fn json_value_to_file_part(name: &str, value: &serde_json::Value) -> Result<Part> {
    if let Some(path) = value.as_str() {
        let bytes = std::fs::read(path)?;
        let file_name = Path::new(path)
            .file_name()
            .and_then(|segment| segment.to_str())
            .unwrap_or("upload.bin")
            .to_string();
        return Ok(Part::bytes(bytes).file_name(file_name));
    }
    if let Some(items) = value.as_array() {
        let mut bytes = Vec::with_capacity(items.len());
        for item in items {
            let Some(number) = item.as_u64() else {
                return Err(Error::InvalidMultipartBody {
                    message: format!("multipart file field `{name}` only accepts a path string or byte array"),
                });
            };
            if number > 255 {
                return Err(Error::InvalidMultipartBody {
                    message: format!("multipart file field `{name}` contains a byte outside 0-255"),
                });
            }
            bytes.push(number as u8);
        }
        return Ok(Part::bytes(bytes).file_name("upload.bin"));
    }
    Err(Error::InvalidMultipartBody {
        message: format!("multipart file field `{name}` only accepts a path string or byte array"),
    })
}

#[derive(Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<Url>,
    timeout: Option<Duration>,
    client: Option<reqwest::Client>,
    user_agent: Option<String>,
    disable_cache: Option<bool>,
}

impl ClientBuilder {
    pub fn api_key<T: Into<String>>(mut self, api_key: T) -> Self { self.api_key = Some(api_key.into()); self }
    pub fn base_url(mut self, base: Url) -> Self { self.base_url = Some(normalize_base_url(base)); self }
    pub fn timeout(mut self, secs: u64) -> Self { self.timeout = Some(Duration::from_secs(secs)); self }
    pub fn user_agent<T: Into<String>>(mut self, ua: T) -> Self { self.user_agent = Some(ua.into()); self }
    pub fn http_client(mut self, cli: reqwest::Client) -> Self { self.client = Some(cli); self }
    pub fn disable_cache(mut self, disable_cache: bool) -> Self { self.disable_cache = Some(disable_cache); self }

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
            disable_cache_default: self.disable_cache.unwrap_or(false),
            last_response_meta: Arc::new(RwLock::new(None)),
        })
    }
}

fn normalize_base_url(mut base: Url) -> Url {
    let trimmed = base.path().trim_end_matches('/');
    let without_api_prefix = trimmed.strip_suffix("/api/v1").unwrap_or(trimmed);
    let normalized = without_api_prefix.trim_end_matches('/');
    if normalized.is_empty() {
        base.set_path("/");
    } else {
        base.set_path(&format!("{normalized}/"));
    }
    base
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
    if trimmed.is_empty() { None } else { Some(trimmed.to_owned()) }
}

fn map_status_to_error(
    status: StatusCode,
    code: Option<String>,
    message: Option<String>,
    details: Option<serde_json::Value>,
    request_id: Option<String>,
    retry_after: Option<u64>,
    meta: Option<ResponseMeta>,
) -> Error {
    let s = status.as_u16();
    let normalized_code = code.clone().unwrap_or_default().to_uppercase();
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::AuthenticationError { status: s, message, request_id, meta },
        StatusCode::PAYMENT_REQUIRED if normalized_code == "INSUFFICIENT_CREDITS" || normalized_code.is_empty() => Error::InsufficientCredits { status: s, message, details, request_id, meta },
        StatusCode::TOO_MANY_REQUESTS if normalized_code == "VISITOR_MONTHLY_QUOTA_EXHAUSTED" => Error::VisitorMonthlyQuotaExhausted { status: s, message, details, request_id, meta },
        StatusCode::TOO_MANY_REQUESTS => Error::RateLimitError { status: s, message, retry_after_seconds: retry_after, request_id, meta },
        StatusCode::NOT_FOUND => Error::NotFound { status: s, message, request_id, meta },
        StatusCode::BAD_REQUEST => Error::ValidationError { status: s, message, details, request_id, meta },
        _ if status.is_server_error() => Error::ServerError { status: s, message, request_id, meta },
        _ if status.is_client_error() => Error::ApiError { status: s, code, message, details, request_id, meta },
        _ => Error::ApiError { status: s, code, message, details, request_id, meta },
    }
}

fn extract_response_meta(headers: &HeaderMap) -> ResponseMeta {
    let mut meta = ResponseMeta {
        raw_headers: BTreeMap::new(),
        ..Default::default()
    };
    for (name, value) in headers {
        if let Ok(text) = value.to_str() {
            meta.raw_headers.insert(name.as_str().to_ascii_lowercase(), text.to_string());
        }
    }
    meta.request_id = meta.raw_headers.get("x-request-id").cloned();
    meta.retry_after_seconds = parse_retry_after(headers);
    meta.debit_status = meta.raw_headers.get("uapi-debit-status").cloned();
    meta.credits_requested = meta.raw_headers.get("uapi-credits-requested").and_then(|v| v.parse::<i64>().ok());
    meta.credits_charged = meta.raw_headers.get("uapi-credits-charged").and_then(|v| v.parse::<i64>().ok());
    meta.credits_pricing = meta.raw_headers.get("uapi-credits-pricing").cloned();
    meta.active_quota_buckets = meta.raw_headers.get("uapi-quota-active-buckets").and_then(|v| v.parse::<u64>().ok());
    meta.stop_on_empty = meta.raw_headers.get("uapi-stop-on-empty").and_then(|value| match value.trim().to_ascii_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    });
    meta.rate_limit_policy_raw = meta.raw_headers.get("ratelimit-policy").cloned();
    meta.rate_limit_raw = meta.raw_headers.get("ratelimit").cloned();

    for item in parse_structured_items(meta.rate_limit_policy_raw.as_deref()) {
        let entry = RateLimitPolicyEntry {
            name: item.name.clone(),
            quota: item.params.get("q").and_then(|v| v.parse::<i64>().ok()),
            unit: item.params.get("uapi-unit").cloned(),
            window_seconds: item.params.get("w").and_then(|v| v.parse::<u64>().ok()),
        };
        meta.rate_limit_policies.insert(item.name, entry);
    }
    for item in parse_structured_items(meta.rate_limit_raw.as_deref()) {
        let entry = RateLimitStateEntry {
            name: item.name.clone(),
            remaining: item.params.get("r").and_then(|v| v.parse::<i64>().ok()),
            unit: item.params.get("uapi-unit").cloned(),
            reset_after_seconds: item.params.get("t").and_then(|v| v.parse::<u64>().ok()),
        };
        meta.rate_limits.insert(item.name, entry);
    }
    meta.balance_limit_cents = meta.rate_limit_policies.get("billing-balance").and_then(|entry| entry.quota);
    meta.balance_remaining_cents = meta.rate_limits.get("billing-balance").and_then(|entry| entry.remaining);
    meta.quota_limit_credits = meta.rate_limit_policies.get("billing-quota").and_then(|entry| entry.quota);
    meta.quota_remaining_credits = meta.rate_limits.get("billing-quota").and_then(|entry| entry.remaining);
    meta.visitor_quota_limit_credits = meta.rate_limit_policies.get("visitor-quota").and_then(|entry| entry.quota);
    meta.visitor_quota_remaining_credits = meta.rate_limits.get("visitor-quota").and_then(|entry| entry.remaining);
    meta
}

struct StructuredItem {
    name: String,
    params: BTreeMap<String, String>,
}

fn parse_structured_items(raw: Option<&str>) -> Vec<StructuredItem> {
    raw.unwrap_or_default()
        .split(',')
        .filter_map(|chunk| {
            let trimmed = chunk.trim();
            if trimmed.is_empty() {
                return None;
            }
            let mut segments = trimmed.split(';');
            let name = unquote(segments.next()?);
            let mut params = BTreeMap::new();
            for segment in segments {
                let part = segment.trim();
                if let Some((key, value)) = part.split_once('=') {
                    params.insert(key.trim().to_string(), unquote(value));
                }
            }
            Some(StructuredItem { name, params })
        })
        .collect()
}

fn unquote(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}
