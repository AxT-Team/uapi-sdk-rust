use crate::client::Client;
use crate::errors::Error;
use crate::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use serde_json::Value;
use tracing::instrument;
use urlencoding::encode;
#[derive(Debug, Clone)]
pub struct ClipzyZaiXianJianTieBanService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClipzyZaiXianJianTieBanService<'a> {
/// 步骤2 (方法一): 获取加密数据
    #[instrument(skip(self, params))]
    pub async fn get_clipzy_get(&self, params: GetClipzyGetParams) -> Result<crate::models::GetClipzyGet200Response> {
        let mut path = "/api/v1/api/get".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("id".to_string(), params.id_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 步骤2 (方法二): 获取原始文本
    #[instrument(skip(self, params))]
    pub async fn get_clipzy_raw(&self, params: GetClipzyRawParams) -> Result<String> {
        let mut path = "/api/v1/api/raw/{id}".to_string();
        {
            let encoded = encode(&params.id_path).into_owned();
            path = path.replace("{id}", &encoded);
        }

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("key".to_string(), params.key_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 步骤1：上传加密数据
    #[instrument(skip(self, params))]
    pub async fn post_clipzy_store(&self, params: PostClipzyStoreParams) -> Result<crate::models::PostClipzyStore200Response> {
        let mut path = "/api/v1/api/store".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetClipzyGetParams {
    pub id_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetClipzyGetParams {
    pub fn new(id_query: impl Into<String>) -> Self {
        Self {
            id_query: id_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetClipzyRawParams {
    pub id_path: String,
    pub key_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetClipzyRawParams {
    pub fn new(id_path: impl Into<String>, key_query: impl Into<String>) -> Self {
        Self {
            id_path: id_path.into(),
            key_query: key_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostClipzyStoreParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostClipzyStoreParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ConvertService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ConvertService<'a> {
/// 时间戳转换
    #[instrument(skip(self, params))]
    pub async fn get_convert_unixtime(&self, params: GetConvertUnixtimeParams) -> Result<crate::models::GetConvertUnixtime200Response> {
        let mut path = "/api/v1/convert/unixtime".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("time".to_string(), params.time_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// JSON 格式化
    #[instrument(skip(self, params))]
    pub async fn post_convert_json(&self, params: PostConvertJsonParams) -> Result<crate::models::PostConvertJson200Response> {
        let mut path = "/api/v1/convert/json".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetConvertUnixtimeParams {
    pub time_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetConvertUnixtimeParams {
    pub fn new(time_query: impl Into<String>) -> Self {
        Self {
            time_query: time_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostConvertJsonParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostConvertJsonParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct DailyService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> DailyService<'a> {
/// 每日新闻图
    #[instrument(skip(self))]
    pub async fn get_daily_news_image(&self) -> Result<Vec<u8>> {
        let mut path = "/api/v1/daily/news-image".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GameService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> GameService<'a> {
/// Epic 免费游戏
    #[instrument(skip(self))]
    pub async fn get_game_epic_free(&self) -> Result<crate::models::GetGameEpicFree200Response> {
        let mut path = "/api/v1/game/epic-free".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
/// 查询 MC 曾用名
    #[instrument(skip(self, params))]
    pub async fn get_game_minecraft_historyid(&self, params: GetGameMinecraftHistoryidParams) -> Result<crate::models::GetGameMinecraftHistoryid200Response> {
        let mut path = "/api/v1/game/minecraft/historyid".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.name_query {
            query.push(("name".to_string(), value.clone()));
        }
        if let Some(value) = &params.uuid_query {
            query.push(("uuid".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 MC 服务器
    #[instrument(skip(self, params))]
    pub async fn get_game_minecraft_serverstatus(&self, params: GetGameMinecraftServerstatusParams) -> Result<crate::models::GetGameMinecraftServerstatus200Response> {
        let mut path = "/api/v1/game/minecraft/serverstatus".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("server".to_string(), params.server_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 MC 玩家
    #[instrument(skip(self, params))]
    pub async fn get_game_minecraft_userinfo(&self, params: GetGameMinecraftUserinfoParams) -> Result<crate::models::GetGameMinecraftUserinfo200Response> {
        let mut path = "/api/v1/game/minecraft/userinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("username".to_string(), params.username_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 Steam 用户
    #[instrument(skip(self, params))]
    pub async fn get_game_steam_summary(&self, params: GetGameSteamSummaryParams) -> Result<crate::models::GetGameSteamSummary200Response> {
        let mut path = "/api/v1/game/steam/summary".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.steamid_query {
            query.push(("steamid".to_string(), value.clone()));
        }
        if let Some(value) = &params.id_query {
            query.push(("id".to_string(), value.clone()));
        }
        if let Some(value) = &params.id_3_query {
            query.push(("id3".to_string(), value.clone()));
        }
        if let Some(value) = &params.key_query {
            query.push(("key".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}


#[derive(Debug, Clone)]
pub struct GetGameMinecraftHistoryidParams {
    pub name_query: Option<String>,
    pub uuid_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetGameMinecraftHistoryidParams {
    pub fn new() -> Self {
        Self {
            name_query: None,
            uuid_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn name_query(mut self, value: impl Into<String>) -> Self {
        self.name_query = Some(value.into());
        self
    }
    pub fn uuid_query(mut self, value: impl Into<String>) -> Self {
        self.uuid_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetGameMinecraftServerstatusParams {
    pub server_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetGameMinecraftServerstatusParams {
    pub fn new(server_query: impl Into<String>) -> Self {
        Self {
            server_query: server_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetGameMinecraftUserinfoParams {
    pub username_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetGameMinecraftUserinfoParams {
    pub fn new(username_query: impl Into<String>) -> Self {
        Self {
            username_query: username_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetGameSteamSummaryParams {
    pub steamid_query: Option<String>,
    pub id_query: Option<String>,
    pub id_3_query: Option<String>,
    pub key_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetGameSteamSummaryParams {
    pub fn new() -> Self {
        Self {
            steamid_query: None,
            id_query: None,
            id_3_query: None,
            key_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn steamid_query(mut self, value: impl Into<String>) -> Self {
        self.steamid_query = Some(value.into());
        self
    }
    pub fn id_query(mut self, value: impl Into<String>) -> Self {
        self.id_query = Some(value.into());
        self
    }
    pub fn id_3_query(mut self, value: impl Into<String>) -> Self {
        self.id_3_query = Some(value.into());
        self
    }
    pub fn key_query(mut self, value: impl Into<String>) -> Self {
        self.key_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct ImageService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ImageService<'a> {
/// 获取Gravatar头像
    #[instrument(skip(self, params))]
    pub async fn get_avatar_gravatar(&self, params: GetAvatarGravatarParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/avatar/gravatar".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.email_query {
            query.push(("email".to_string(), value.clone()));
        }
        if let Some(value) = &params.hash_query {
            query.push(("hash".to_string(), value.clone()));
        }
        if let Some(value) = &params.s_query {
            query.push(("s".to_string(), value.clone()));
        }
        if let Some(value) = &params.d_query {
            query.push(("d".to_string(), value.clone()));
        }
        if let Some(value) = &params.r_query {
            query.push(("r".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 获取必应每日壁纸
    #[instrument(skip(self, params))]
    pub async fn get_image_bing_daily(&self, params: GetImageBingDailyParams) -> Result<crate::models::FormatJson> {
        let mut path = "/api/v1/image/bing-daily".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.date_query {
            query.push(("date".to_string(), value.clone()));
        }
        if let Some(value) = &params.resolution_query {
            query.push(("resolution".to_string(), value.clone()));
        }
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询必应壁纸历史
    #[instrument(skip(self, params))]
    pub async fn get_image_bing_daily_history(&self, params: GetImageBingDailyHistoryParams) -> Result<crate::models::GetImageBingDailyHistory200Response> {
        let mut path = "/api/v1/image/bing-daily/history".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.date_query {
            query.push(("date".to_string(), value.clone()));
        }
        if let Some(value) = &params.resolution_query {
            query.push(("resolution".to_string(), value.clone()));
        }
        if let Some(value) = &params.page_query {
            query.push(("page".to_string(), value.clone()));
        }
        if let Some(value) = &params.page_size_query {
            query.push(("page_size".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 生成摸摸头GIF (QQ号)
    #[instrument(skip(self, params))]
    pub async fn get_image_motou(&self, params: GetImageMotouParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/image/motou".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("qq".to_string(), params.qq_query.clone()));
        if let Some(value) = &params.bg_color_query {
            query.push(("bg_color".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 生成二维码
    #[instrument(skip(self, params))]
    pub async fn get_image_qrcode(&self, params: GetImageQrcodeParams) -> Result<crate::models::GetImageQrcode200Response> {
        let mut path = "/api/v1/image/qrcode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("text".to_string(), params.text_query.clone()));
        if let Some(value) = &params.size_query {
            query.push(("size".to_string(), value.clone()));
        }
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
        }
        if let Some(value) = &params.transparent_query {
            query.push(("transparent".to_string(), value.clone()));
        }
        if let Some(value) = &params.fgcolor_query {
            query.push(("fgcolor".to_string(), value.clone()));
        }
        if let Some(value) = &params.bgcolor_query {
            query.push(("bgcolor".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 图片转 Base64
    #[instrument(skip(self, params))]
    pub async fn get_image_tobase_64(&self, params: GetImageTobase64Params) -> Result<crate::models::GetImageTobase64200Response> {
        let mut path = "/api/v1/image/tobase64".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 无损压缩图片
    #[instrument(skip(self, params))]
    pub async fn post_image_compress(&self, params: PostImageCompressParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/image/compress".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.level_query {
            query.push(("level".to_string(), value.clone()));
        }
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 解码并缩放图片
    #[instrument(skip(self, params))]
    pub async fn post_image_decode(&self, params: PostImageDecodeParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/image/decode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.width_query {
            query.push(("width".to_string(), value.clone()));
        }
        if let Some(value) = &params.height_query {
            query.push(("height".to_string(), value.clone()));
        }
        if let Some(value) = &params.max_width_query {
            query.push(("max_width".to_string(), value.clone()));
        }
        if let Some(value) = &params.max_height_query {
            query.push(("max_height".to_string(), value.clone()));
        }
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
        }
        if let Some(value) = &params.color_mode_query {
            query.push(("color_mode".to_string(), value.clone()));
        }
        if let Some(value) = &params.fit_query {
            query.push(("fit".to_string(), value.clone()));
        }
        if let Some(value) = &params.background_query {
            query.push(("background".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 通过Base64编码上传图片
    #[instrument(skip(self, params))]
    pub async fn post_image_frombase_64(&self, params: PostImageFrombase64Params) -> Result<crate::models::PostImageFrombase64200Response> {
        let mut path = "/api/v1/image/frombase64".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 生成摸摸头GIF
    #[instrument(skip(self, params))]
    pub async fn post_image_motou(&self, params: PostImageMotouParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/image/motou".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 图片敏感检测
    #[instrument(skip(self, params))]
    pub async fn post_image_nsfw(&self, params: PostImageNsfwParams) -> Result<crate::models::PostImageNsfw200Response> {
        let mut path = "/api/v1/image/nsfw".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 通用 OCR 文字识别
    #[instrument(skip(self, params))]
    pub async fn post_image_ocr(&self, params: PostImageOcrParams) -> Result<crate::models::PostImageOcr200Response> {
        let mut path = "/api/v1/image/ocr".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 生成你们怎么不说话了表情包
    #[instrument(skip(self, params))]
    pub async fn post_image_speechless(&self, params: PostImageSpeechlessParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/image/speechless".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// SVG转图片
    #[instrument(skip(self, params))]
    pub async fn post_image_svg(&self, params: PostImageSvgParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/image/svg".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
        }
        if let Some(value) = &params.width_query {
            query.push(("width".to_string(), value.clone()));
        }
        if let Some(value) = &params.height_query {
            query.push(("height".to_string(), value.clone()));
        }
        if let Some(value) = &params.quality_query {
            query.push(("quality".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetAvatarGravatarParams {
    pub email_query: Option<String>,
    pub hash_query: Option<String>,
    pub s_query: Option<String>,
    pub d_query: Option<String>,
    pub r_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetAvatarGravatarParams {
    pub fn new() -> Self {
        Self {
            email_query: None,
            hash_query: None,
            s_query: None,
            d_query: None,
            r_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn email_query(mut self, value: impl Into<String>) -> Self {
        self.email_query = Some(value.into());
        self
    }
    pub fn hash_query(mut self, value: impl Into<String>) -> Self {
        self.hash_query = Some(value.into());
        self
    }
    pub fn s_query(mut self, value: impl Into<String>) -> Self {
        self.s_query = Some(value.into());
        self
    }
    pub fn d_query(mut self, value: impl Into<String>) -> Self {
        self.d_query = Some(value.into());
        self
    }
    pub fn r_query(mut self, value: impl Into<String>) -> Self {
        self.r_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetImageBingDailyParams {
    pub date_query: Option<String>,
    pub resolution_query: Option<String>,
    pub format_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetImageBingDailyParams {
    pub fn new() -> Self {
        Self {
            date_query: None,
            resolution_query: None,
            format_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn date_query(mut self, value: impl Into<String>) -> Self {
        self.date_query = Some(value.into());
        self
    }
    pub fn resolution_query(mut self, value: impl Into<String>) -> Self {
        self.resolution_query = Some(value.into());
        self
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetImageBingDailyHistoryParams {
    pub date_query: Option<String>,
    pub resolution_query: Option<String>,
    pub page_query: Option<String>,
    pub page_size_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetImageBingDailyHistoryParams {
    pub fn new() -> Self {
        Self {
            date_query: None,
            resolution_query: None,
            page_query: None,
            page_size_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn date_query(mut self, value: impl Into<String>) -> Self {
        self.date_query = Some(value.into());
        self
    }
    pub fn resolution_query(mut self, value: impl Into<String>) -> Self {
        self.resolution_query = Some(value.into());
        self
    }
    pub fn page_query(mut self, value: impl Into<String>) -> Self {
        self.page_query = Some(value.into());
        self
    }
    pub fn page_size_query(mut self, value: impl Into<String>) -> Self {
        self.page_size_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetImageMotouParams {
    pub qq_query: String,
    pub bg_color_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetImageMotouParams {
    pub fn new(qq_query: impl Into<String>) -> Self {
        Self {
            qq_query: qq_query.into(),
            bg_color_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn bg_color_query(mut self, value: impl Into<String>) -> Self {
        self.bg_color_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetImageQrcodeParams {
    pub text_query: String,
    pub size_query: Option<String>,
    pub format_query: Option<String>,
    pub transparent_query: Option<String>,
    pub fgcolor_query: Option<String>,
    pub bgcolor_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetImageQrcodeParams {
    pub fn new(text_query: impl Into<String>) -> Self {
        Self {
            text_query: text_query.into(),
            size_query: None,
            format_query: None,
            transparent_query: None,
            fgcolor_query: None,
            bgcolor_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn size_query(mut self, value: impl Into<String>) -> Self {
        self.size_query = Some(value.into());
        self
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
    pub fn transparent_query(mut self, value: impl Into<String>) -> Self {
        self.transparent_query = Some(value.into());
        self
    }
    pub fn fgcolor_query(mut self, value: impl Into<String>) -> Self {
        self.fgcolor_query = Some(value.into());
        self
    }
    pub fn bgcolor_query(mut self, value: impl Into<String>) -> Self {
        self.bgcolor_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetImageTobase64Params {
    pub url_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetImageTobase64Params {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageCompressParams {
    pub level_query: Option<String>,
    pub format_query: Option<String>,
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageCompressParams {
    pub fn new() -> Self {
        Self {
            level_query: None,
            format_query: None,
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn level_query(mut self, value: impl Into<String>) -> Self {
        self.level_query = Some(value.into());
        self
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageDecodeParams {
    pub width_query: Option<String>,
    pub height_query: Option<String>,
    pub max_width_query: Option<String>,
    pub max_height_query: Option<String>,
    pub format_query: Option<String>,
    pub color_mode_query: Option<String>,
    pub fit_query: Option<String>,
    pub background_query: Option<String>,
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageDecodeParams {
    pub fn new() -> Self {
        Self {
            width_query: None,
            height_query: None,
            max_width_query: None,
            max_height_query: None,
            format_query: None,
            color_mode_query: None,
            fit_query: None,
            background_query: None,
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn width_query(mut self, value: impl Into<String>) -> Self {
        self.width_query = Some(value.into());
        self
    }
    pub fn height_query(mut self, value: impl Into<String>) -> Self {
        self.height_query = Some(value.into());
        self
    }
    pub fn max_width_query(mut self, value: impl Into<String>) -> Self {
        self.max_width_query = Some(value.into());
        self
    }
    pub fn max_height_query(mut self, value: impl Into<String>) -> Self {
        self.max_height_query = Some(value.into());
        self
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
    pub fn color_mode_query(mut self, value: impl Into<String>) -> Self {
        self.color_mode_query = Some(value.into());
        self
    }
    pub fn fit_query(mut self, value: impl Into<String>) -> Self {
        self.fit_query = Some(value.into());
        self
    }
    pub fn background_query(mut self, value: impl Into<String>) -> Self {
        self.background_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageFrombase64Params {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageFrombase64Params {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageMotouParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageMotouParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageNsfwParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageNsfwParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageOcrParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageOcrParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageSpeechlessParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageSpeechlessParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageSvgParams {
    pub format_query: Option<String>,
    pub width_query: Option<String>,
    pub height_query: Option<String>,
    pub quality_query: Option<String>,
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostImageSvgParams {
    pub fn new() -> Self {
        Self {
            format_query: None,
            width_query: None,
            height_query: None,
            quality_query: None,
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
    pub fn width_query(mut self, value: impl Into<String>) -> Self {
        self.width_query = Some(value.into());
        self
    }
    pub fn height_query(mut self, value: impl Into<String>) -> Self {
        self.height_query = Some(value.into());
        self
    }
    pub fn quality_query(mut self, value: impl Into<String>) -> Self {
        self.quality_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MiscService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> MiscService<'a> {
/// 程序员历史事件
    #[instrument(skip(self, params))]
    pub async fn get_history_programmer(&self, params: GetHistoryProgrammerParams) -> Result<crate::models::GetHistoryProgrammer200Response> {
        let mut path = "/api/v1/history/programmer".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("month".to_string(), params.month_query.clone()));
        query.push(("day".to_string(), params.day_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 程序员历史上的今天
    #[instrument(skip(self))]
    pub async fn get_history_programmer_today(&self) -> Result<crate::models::GetHistoryProgrammerToday200Response> {
        let mut path = "/api/v1/history/programmer/today".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
/// Adcode 国内外行政区域查询
    #[instrument(skip(self, params))]
    pub async fn get_misc_district(&self, params: GetMiscDistrictParams) -> Result<crate::models::GetMiscDistrict200Response> {
        let mut path = "/api/v1/misc/district".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.keywords_query {
            query.push(("keywords".to_string(), value.clone()));
        }
        if let Some(value) = &params.adcode_query {
            query.push(("adcode".to_string(), value.clone()));
        }
        if let Some(value) = &params.lat_query {
            query.push(("lat".to_string(), value.clone()));
        }
        if let Some(value) = &params.lng_query {
            query.push(("lng".to_string(), value.clone()));
        }
        if let Some(value) = &params.level_query {
            query.push(("level".to_string(), value.clone()));
        }
        if let Some(value) = &params.country_query {
            query.push(("country".to_string(), value.clone()));
        }
        if let Some(value) = &params.limit_query {
            query.push(("limit".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询节假日与万年历
    #[instrument(skip(self, params))]
    pub async fn get_misc_holiday_calendar(&self, params: GetMiscHolidayCalendarParams) -> Result<crate::models::GetMiscHolidayCalendar200Response> {
        let mut path = "/api/v1/misc/holiday-calendar".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.date_query {
            query.push(("date".to_string(), value.clone()));
        }
        if let Some(value) = &params.month_query {
            query.push(("month".to_string(), value.clone()));
        }
        if let Some(value) = &params.year_query {
            query.push(("year".to_string(), value.clone()));
        }
        if let Some(value) = &params.timezone_query {
            query.push(("timezone".to_string(), value.clone()));
        }
        if let Some(value) = &params.holiday_type_query {
            query.push(("holiday_type".to_string(), value.clone()));
        }
        if let Some(value) = &params.include_nearby_query {
            query.push(("include_nearby".to_string(), value.clone()));
        }
        if let Some(value) = &params.nearby_limit_query {
            query.push(("nearby_limit".to_string(), value.clone()));
        }
        if let Some(value) = &params.exclude_past_query {
            query.push(("exclude_past".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询热榜
    #[instrument(skip(self, params))]
    pub async fn get_misc_hotboard(&self, params: GetMiscHotboardParams) -> Result<crate::models::GetMiscHotboard200Response> {
        let mut path = "/api/v1/misc/hotboard".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("type".to_string(), params.type_query.clone()));
        if let Some(value) = &params.time_query {
            query.push(("time".to_string(), value.clone()));
        }
        if let Some(value) = &params.keyword_query {
            query.push(("keyword".to_string(), value.clone()));
        }
        if let Some(value) = &params.time_start_query {
            query.push(("time_start".to_string(), value.clone()));
        }
        if let Some(value) = &params.time_end_query {
            query.push(("time_end".to_string(), value.clone()));
        }
        if let Some(value) = &params.limit_query {
            query.push(("limit".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询农历时间
    #[instrument(skip(self, params))]
    pub async fn get_misc_lunartime(&self, params: GetMiscLunartimeParams) -> Result<crate::models::GetMiscLunartime200Response> {
        let mut path = "/api/v1/misc/lunartime".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.ts_query {
            query.push(("ts".to_string(), value.clone()));
        }
        if let Some(value) = &params.timezone_query {
            query.push(("timezone".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询手机归属地
    #[instrument(skip(self, params))]
    pub async fn get_misc_phoneinfo(&self, params: GetMiscPhoneinfoParams) -> Result<crate::models::GetMiscPhoneinfo200Response> {
        let mut path = "/api/v1/misc/phoneinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("phone".to_string(), params.phone_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 随机数生成
    #[instrument(skip(self, params))]
    pub async fn get_misc_randomnumber(&self, params: GetMiscRandomnumberParams) -> Result<crate::models::GetMiscRandomnumber200Response> {
        let mut path = "/api/v1/misc/randomnumber".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.min_query {
            query.push(("min".to_string(), value.clone()));
        }
        if let Some(value) = &params.max_query {
            query.push(("max".to_string(), value.clone()));
        }
        if let Some(value) = &params.count_query {
            query.push(("count".to_string(), value.clone()));
        }
        if let Some(value) = &params.allow_repeat_query {
            query.push(("allow_repeat".to_string(), value.clone()));
        }
        if let Some(value) = &params.allow_decimal_query {
            query.push(("allow_decimal".to_string(), value.clone()));
        }
        if let Some(value) = &params.decimal_places_query {
            query.push(("decimal_places".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 转换时间戳 (旧版，推荐使用/convert/unixtime)
    #[instrument(skip(self, params))]
    pub async fn get_misc_timestamp(&self, params: GetMiscTimestampParams) -> Result<crate::models::GetMiscTimestamp200Response> {
        let mut path = "/api/v1/misc/timestamp".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("ts".to_string(), params.ts_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 获取支持的快递公司列表
    #[instrument(skip(self))]
    pub async fn get_misc_tracking_carriers(&self) -> Result<crate::models::GetMiscTrackingCarriers200Response> {
        let mut path = "/api/v1/misc/tracking/carriers".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
/// 识别快递公司
    #[instrument(skip(self, params))]
    pub async fn get_misc_tracking_detect(&self, params: GetMiscTrackingDetectParams) -> Result<crate::models::GetMiscTrackingDetect200Response> {
        let mut path = "/api/v1/misc/tracking/detect".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("tracking_number".to_string(), params.tracking_number_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询快递物流信息
    #[instrument(skip(self, params))]
    pub async fn get_misc_tracking_query(&self, params: GetMiscTrackingQueryParams) -> Result<crate::models::GetMiscTrackingQuery200Response> {
        let mut path = "/api/v1/misc/tracking/query".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("tracking_number".to_string(), params.tracking_number_query.clone()));
        if let Some(value) = &params.carrier_code_query {
            query.push(("carrier_code".to_string(), value.clone()));
        }
        if let Some(value) = &params.phone_query {
            query.push(("phone".to_string(), value.clone()));
        }
        if let Some(value) = &params.full_query {
            query.push(("full".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询天气
    #[instrument(skip(self, params))]
    pub async fn get_misc_weather(&self, params: GetMiscWeatherParams) -> Result<crate::models::GetMiscWeather200Response> {
        let mut path = "/api/v1/misc/weather".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.city_query {
            query.push(("city".to_string(), value.clone()));
        }
        if let Some(value) = &params.adcode_query {
            query.push(("adcode".to_string(), value.clone()));
        }
        if let Some(value) = &params.extended_query {
            query.push(("extended".to_string(), value.clone()));
        }
        if let Some(value) = &params.forecast_query {
            query.push(("forecast".to_string(), value.clone()));
        }
        if let Some(value) = &params.hourly_query {
            query.push(("hourly".to_string(), value.clone()));
        }
        if let Some(value) = &params.minutely_query {
            query.push(("minutely".to_string(), value.clone()));
        }
        if let Some(value) = &params.indices_query {
            query.push(("indices".to_string(), value.clone()));
        }
        if let Some(value) = &params.lang_query {
            query.push(("lang".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询世界时间
    #[instrument(skip(self, params))]
    pub async fn get_misc_worldtime(&self, params: GetMiscWorldtimeParams) -> Result<crate::models::GetMiscWorldtime200Response> {
        let mut path = "/api/v1/misc/worldtime".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("city".to_string(), params.city_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 计算两个日期之间的时间差值
    #[instrument(skip(self, params))]
    pub async fn post_misc_date_diff(&self, params: PostMiscDateDiffParams) -> Result<crate::models::PostMiscDateDiff200Response> {
        let mut path = "/api/v1/misc/date-diff".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetHistoryProgrammerParams {
    pub month_query: String,
    pub day_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetHistoryProgrammerParams {
    pub fn new(month_query: impl Into<String>, day_query: impl Into<String>) -> Self {
        Self {
            month_query: month_query.into(),
            day_query: day_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}


#[derive(Debug, Clone)]
pub struct GetMiscDistrictParams {
    pub keywords_query: Option<String>,
    pub adcode_query: Option<String>,
    pub lat_query: Option<String>,
    pub lng_query: Option<String>,
    pub level_query: Option<String>,
    pub country_query: Option<String>,
    pub limit_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscDistrictParams {
    pub fn new() -> Self {
        Self {
            keywords_query: None,
            adcode_query: None,
            lat_query: None,
            lng_query: None,
            level_query: None,
            country_query: None,
            limit_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn keywords_query(mut self, value: impl Into<String>) -> Self {
        self.keywords_query = Some(value.into());
        self
    }
    pub fn adcode_query(mut self, value: impl Into<String>) -> Self {
        self.adcode_query = Some(value.into());
        self
    }
    pub fn lat_query(mut self, value: impl Into<String>) -> Self {
        self.lat_query = Some(value.into());
        self
    }
    pub fn lng_query(mut self, value: impl Into<String>) -> Self {
        self.lng_query = Some(value.into());
        self
    }
    pub fn level_query(mut self, value: impl Into<String>) -> Self {
        self.level_query = Some(value.into());
        self
    }
    pub fn country_query(mut self, value: impl Into<String>) -> Self {
        self.country_query = Some(value.into());
        self
    }
    pub fn limit_query(mut self, value: impl Into<String>) -> Self {
        self.limit_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscHolidayCalendarParams {
    pub date_query: Option<String>,
    pub month_query: Option<String>,
    pub year_query: Option<String>,
    pub timezone_query: Option<String>,
    pub holiday_type_query: Option<String>,
    pub include_nearby_query: Option<String>,
    pub nearby_limit_query: Option<String>,
    pub exclude_past_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscHolidayCalendarParams {
    pub fn new() -> Self {
        Self {
            date_query: None,
            month_query: None,
            year_query: None,
            timezone_query: None,
            holiday_type_query: None,
            include_nearby_query: None,
            nearby_limit_query: None,
            exclude_past_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn date_query(mut self, value: impl Into<String>) -> Self {
        self.date_query = Some(value.into());
        self
    }
    pub fn month_query(mut self, value: impl Into<String>) -> Self {
        self.month_query = Some(value.into());
        self
    }
    pub fn year_query(mut self, value: impl Into<String>) -> Self {
        self.year_query = Some(value.into());
        self
    }
    pub fn timezone_query(mut self, value: impl Into<String>) -> Self {
        self.timezone_query = Some(value.into());
        self
    }
    pub fn holiday_type_query(mut self, value: impl Into<String>) -> Self {
        self.holiday_type_query = Some(value.into());
        self
    }
    pub fn include_nearby_query(mut self, value: impl Into<String>) -> Self {
        self.include_nearby_query = Some(value.into());
        self
    }
    pub fn nearby_limit_query(mut self, value: impl Into<String>) -> Self {
        self.nearby_limit_query = Some(value.into());
        self
    }
    pub fn exclude_past_query(mut self, value: impl Into<String>) -> Self {
        self.exclude_past_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscHotboardParams {
    pub type_query: String,
    pub time_query: Option<String>,
    pub keyword_query: Option<String>,
    pub time_start_query: Option<String>,
    pub time_end_query: Option<String>,
    pub limit_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscHotboardParams {
    pub fn new(type_query: impl Into<String>) -> Self {
        Self {
            type_query: type_query.into(),
            time_query: None,
            keyword_query: None,
            time_start_query: None,
            time_end_query: None,
            limit_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn time_query(mut self, value: impl Into<String>) -> Self {
        self.time_query = Some(value.into());
        self
    }
    pub fn keyword_query(mut self, value: impl Into<String>) -> Self {
        self.keyword_query = Some(value.into());
        self
    }
    pub fn time_start_query(mut self, value: impl Into<String>) -> Self {
        self.time_start_query = Some(value.into());
        self
    }
    pub fn time_end_query(mut self, value: impl Into<String>) -> Self {
        self.time_end_query = Some(value.into());
        self
    }
    pub fn limit_query(mut self, value: impl Into<String>) -> Self {
        self.limit_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscLunartimeParams {
    pub ts_query: Option<String>,
    pub timezone_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscLunartimeParams {
    pub fn new() -> Self {
        Self {
            ts_query: None,
            timezone_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn ts_query(mut self, value: impl Into<String>) -> Self {
        self.ts_query = Some(value.into());
        self
    }
    pub fn timezone_query(mut self, value: impl Into<String>) -> Self {
        self.timezone_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscPhoneinfoParams {
    pub phone_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscPhoneinfoParams {
    pub fn new(phone_query: impl Into<String>) -> Self {
        Self {
            phone_query: phone_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscRandomnumberParams {
    pub min_query: Option<String>,
    pub max_query: Option<String>,
    pub count_query: Option<String>,
    pub allow_repeat_query: Option<String>,
    pub allow_decimal_query: Option<String>,
    pub decimal_places_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscRandomnumberParams {
    pub fn new() -> Self {
        Self {
            min_query: None,
            max_query: None,
            count_query: None,
            allow_repeat_query: None,
            allow_decimal_query: None,
            decimal_places_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn min_query(mut self, value: impl Into<String>) -> Self {
        self.min_query = Some(value.into());
        self
    }
    pub fn max_query(mut self, value: impl Into<String>) -> Self {
        self.max_query = Some(value.into());
        self
    }
    pub fn count_query(mut self, value: impl Into<String>) -> Self {
        self.count_query = Some(value.into());
        self
    }
    pub fn allow_repeat_query(mut self, value: impl Into<String>) -> Self {
        self.allow_repeat_query = Some(value.into());
        self
    }
    pub fn allow_decimal_query(mut self, value: impl Into<String>) -> Self {
        self.allow_decimal_query = Some(value.into());
        self
    }
    pub fn decimal_places_query(mut self, value: impl Into<String>) -> Self {
        self.decimal_places_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscTimestampParams {
    pub ts_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscTimestampParams {
    pub fn new(ts_query: impl Into<String>) -> Self {
        Self {
            ts_query: ts_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}


#[derive(Debug, Clone)]
pub struct GetMiscTrackingDetectParams {
    pub tracking_number_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscTrackingDetectParams {
    pub fn new(tracking_number_query: impl Into<String>) -> Self {
        Self {
            tracking_number_query: tracking_number_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscTrackingQueryParams {
    pub tracking_number_query: String,
    pub carrier_code_query: Option<String>,
    pub phone_query: Option<String>,
    pub full_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscTrackingQueryParams {
    pub fn new(tracking_number_query: impl Into<String>) -> Self {
        Self {
            tracking_number_query: tracking_number_query.into(),
            carrier_code_query: None,
            phone_query: None,
            full_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn carrier_code_query(mut self, value: impl Into<String>) -> Self {
        self.carrier_code_query = Some(value.into());
        self
    }
    pub fn phone_query(mut self, value: impl Into<String>) -> Self {
        self.phone_query = Some(value.into());
        self
    }
    pub fn full_query(mut self, value: impl Into<String>) -> Self {
        self.full_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscWeatherParams {
    pub city_query: Option<String>,
    pub adcode_query: Option<String>,
    pub extended_query: Option<String>,
    pub forecast_query: Option<String>,
    pub hourly_query: Option<String>,
    pub minutely_query: Option<String>,
    pub indices_query: Option<String>,
    pub lang_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscWeatherParams {
    pub fn new() -> Self {
        Self {
            city_query: None,
            adcode_query: None,
            extended_query: None,
            forecast_query: None,
            hourly_query: None,
            minutely_query: None,
            indices_query: None,
            lang_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn city_query(mut self, value: impl Into<String>) -> Self {
        self.city_query = Some(value.into());
        self
    }
    pub fn adcode_query(mut self, value: impl Into<String>) -> Self {
        self.adcode_query = Some(value.into());
        self
    }
    pub fn extended_query(mut self, value: impl Into<String>) -> Self {
        self.extended_query = Some(value.into());
        self
    }
    pub fn forecast_query(mut self, value: impl Into<String>) -> Self {
        self.forecast_query = Some(value.into());
        self
    }
    pub fn hourly_query(mut self, value: impl Into<String>) -> Self {
        self.hourly_query = Some(value.into());
        self
    }
    pub fn minutely_query(mut self, value: impl Into<String>) -> Self {
        self.minutely_query = Some(value.into());
        self
    }
    pub fn indices_query(mut self, value: impl Into<String>) -> Self {
        self.indices_query = Some(value.into());
        self
    }
    pub fn lang_query(mut self, value: impl Into<String>) -> Self {
        self.lang_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscWorldtimeParams {
    pub city_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetMiscWorldtimeParams {
    pub fn new(city_query: impl Into<String>) -> Self {
        Self {
            city_query: city_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostMiscDateDiffParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostMiscDateDiffParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct NetworkService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> NetworkService<'a> {
/// 执行DNS解析查询
    #[instrument(skip(self, params))]
    pub async fn get_network_dns(&self, params: GetNetworkDnsParams) -> Result<crate::models::GetNetworkDns200Response> {
        let mut path = "/api/v1/network/dns".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
        if let Some(value) = &params.type_query {
            query.push(("type".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询域名ICP备案信息
    #[instrument(skip(self, params))]
    pub async fn get_network_icp(&self, params: GetNetworkIcpParams) -> Result<crate::models::GetNetworkIcp200Response> {
        let mut path = "/api/v1/network/icp".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 IP
    #[instrument(skip(self, params))]
    pub async fn get_network_ipinfo(&self, params: GetNetworkIpinfoParams) -> Result<crate::models::GetNetworkIpinfo200Response> {
        let mut path = "/api/v1/network/ipinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("ip".to_string(), params.ip_query.clone()));
        if let Some(value) = &params.source_query {
            query.push(("source".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询我的 IP
    #[instrument(skip(self, params))]
    pub async fn get_network_myip(&self, params: GetNetworkMyipParams) -> Result<crate::models::GetNetworkMyip200Response> {
        let mut path = "/api/v1/network/myip".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.source_query {
            query.push(("source".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// Ping 主机
    #[instrument(skip(self, params))]
    pub async fn get_network_ping(&self, params: GetNetworkPingParams) -> Result<crate::models::GetNetworkPing200Response> {
        let mut path = "/api/v1/network/ping".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("host".to_string(), params.host_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// Ping 我的 IP
    #[instrument(skip(self))]
    pub async fn get_network_pingmyip(&self) -> Result<crate::models::GetNetworkPingmyip200Response> {
        let mut path = "/api/v1/network/pingmyip".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
/// 端口扫描
    #[instrument(skip(self, params))]
    pub async fn get_network_portscan(&self, params: GetNetworkPortscanParams) -> Result<crate::models::GetNetworkPortscan200Response> {
        let mut path = "/api/v1/network/portscan".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("host".to_string(), params.host_query.clone()));
        query.push(("port".to_string(), params.port_query.clone()));
        if let Some(value) = &params.protocol_query {
            query.push(("protocol".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 检查URL的可访问性状态
    #[instrument(skip(self, params))]
    pub async fn get_network_urlstatus(&self, params: GetNetworkUrlstatusParams) -> Result<crate::models::GetNetworkUrlstatus200Response> {
        let mut path = "/api/v1/network/urlstatus".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询域名的WHOIS注册信息
    #[instrument(skip(self, params))]
    pub async fn get_network_whois(&self, params: GetNetworkWhoisParams) -> Result<crate::models::GetNetworkWhois200Response> {
        let mut path = "/api/v1/network/whois".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 检查域名在微信中的访问状态
    #[instrument(skip(self, params))]
    pub async fn get_network_wxdomain(&self, params: GetNetworkWxdomainParams) -> Result<crate::models::GetNetworkWxdomain200Response> {
        let mut path = "/api/v1/network/wxdomain".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkDnsParams {
    pub domain_query: String,
    pub type_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkDnsParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
            type_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn type_query(mut self, value: impl Into<String>) -> Self {
        self.type_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkIcpParams {
    pub domain_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkIcpParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkIpinfoParams {
    pub ip_query: String,
    pub source_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkIpinfoParams {
    pub fn new(ip_query: impl Into<String>) -> Self {
        Self {
            ip_query: ip_query.into(),
            source_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn source_query(mut self, value: impl Into<String>) -> Self {
        self.source_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkMyipParams {
    pub source_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkMyipParams {
    pub fn new() -> Self {
        Self {
            source_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn source_query(mut self, value: impl Into<String>) -> Self {
        self.source_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkPingParams {
    pub host_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkPingParams {
    pub fn new(host_query: impl Into<String>) -> Self {
        Self {
            host_query: host_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}


#[derive(Debug, Clone)]
pub struct GetNetworkPortscanParams {
    pub host_query: String,
    pub port_query: String,
    pub protocol_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkPortscanParams {
    pub fn new(host_query: impl Into<String>, port_query: impl Into<String>) -> Self {
        Self {
            host_query: host_query.into(),
            port_query: port_query.into(),
            protocol_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn protocol_query(mut self, value: impl Into<String>) -> Self {
        self.protocol_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkUrlstatusParams {
    pub url_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkUrlstatusParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkWhoisParams {
    pub domain_query: String,
    pub format_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkWhoisParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
            format_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkWxdomainParams {
    pub domain_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetNetworkWxdomainParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct PoemService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> PoemService<'a> {
/// 一言
    #[instrument(skip(self))]
    pub async fn get_saying(&self) -> Result<crate::models::GetSaying200Response> {
        let mut path = "/api/v1/saying".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct RandomService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> RandomService<'a> {
/// 答案之书
    #[instrument(skip(self, params))]
    pub async fn get_answerbook_ask(&self, params: GetAnswerbookAskParams) -> Result<crate::models::GetAnswerbookAsk200Response> {
        let mut path = "/api/v1/answerbook/ask".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("question".to_string(), params.question_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 随机图片
    #[instrument(skip(self, params))]
    pub async fn get_random_image(&self, params: GetRandomImageParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/random/image".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.category_query {
            query.push(("category".to_string(), value.clone()));
        }
        if let Some(value) = &params.type_query {
            query.push(("type".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 随机字符串
    #[instrument(skip(self, params))]
    pub async fn get_random_string(&self, params: GetRandomStringParams) -> Result<crate::models::GetRandomString200Response> {
        let mut path = "/api/v1/random/string".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.length_query {
            query.push(("length".to_string(), value.clone()));
        }
        if let Some(value) = &params.type_query {
            query.push(("type".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 答案之书 (POST)
    #[instrument(skip(self, params))]
    pub async fn post_answerbook_ask(&self, params: PostAnswerbookAskParams) -> Result<crate::models::PostAnswerbookAsk200Response> {
        let mut path = "/api/v1/answerbook/ask".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetAnswerbookAskParams {
    pub question_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetAnswerbookAskParams {
    pub fn new(question_query: impl Into<String>) -> Self {
        Self {
            question_query: question_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetRandomImageParams {
    pub category_query: Option<String>,
    pub type_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetRandomImageParams {
    pub fn new() -> Self {
        Self {
            category_query: None,
            type_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn category_query(mut self, value: impl Into<String>) -> Self {
        self.category_query = Some(value.into());
        self
    }
    pub fn type_query(mut self, value: impl Into<String>) -> Self {
        self.type_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetRandomStringParams {
    pub length_query: Option<String>,
    pub type_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetRandomStringParams {
    pub fn new() -> Self {
        Self {
            length_query: None,
            type_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn length_query(mut self, value: impl Into<String>) -> Self {
        self.length_query = Some(value.into());
        self
    }
    pub fn type_query(mut self, value: impl Into<String>) -> Self {
        self.type_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostAnswerbookAskParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostAnswerbookAskParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SocialService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> SocialService<'a> {
/// 查询 GitHub 仓库
    #[instrument(skip(self, params))]
    pub async fn get_github_repo(&self, params: GetGithubRepoParams) -> Result<crate::models::GetGithubRepo200Response> {
        let mut path = "/api/v1/github/repo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("repo".to_string(), params.repo_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 GitHub 用户信息
    #[instrument(skip(self, params))]
    pub async fn get_github_user(&self, params: GetGithubUserParams) -> Result<crate::models::GetGithubUser200Response> {
        let mut path = "/api/v1/github/user".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("user".to_string(), params.user_query.clone()));
        if let Some(value) = &params.activity_query {
            query.push(("activity".to_string(), value.clone()));
        }
        if let Some(value) = &params.activity_scope_query {
            query.push(("activity_scope".to_string(), value.clone()));
        }
        if let Some(value) = &params.org_query {
            query.push(("org".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 B站投稿
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_archives(&self, params: GetSocialBilibiliArchivesParams) -> Result<crate::models::GetSocialBilibiliArchives200Response> {
        let mut path = "/api/v1/social/bilibili/archives".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("mid".to_string(), params.mid_query.clone()));
        if let Some(value) = &params.keywords_query {
            query.push(("keywords".to_string(), value.clone()));
        }
        if let Some(value) = &params.orderby_query {
            query.push(("orderby".to_string(), value.clone()));
        }
        if let Some(value) = &params.ps_query {
            query.push(("ps".to_string(), value.clone()));
        }
        if let Some(value) = &params.pn_query {
            query.push(("pn".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 B站直播间
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_liveroom(&self, params: GetSocialBilibiliLiveroomParams) -> Result<crate::models::GetSocialBilibiliLiveroom200Response> {
        let mut path = "/api/v1/social/bilibili/liveroom".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.mid_query {
            query.push(("mid".to_string(), value.clone()));
        }
        if let Some(value) = &params.room_id_query {
            query.push(("room_id".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 B站评论
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_replies(&self, params: GetSocialBilibiliRepliesParams) -> Result<crate::models::GetSocialBilibiliReplies200Response> {
        let mut path = "/api/v1/social/bilibili/replies".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("oid".to_string(), params.oid_query.clone()));
        if let Some(value) = &params.sort_query {
            query.push(("sort".to_string(), value.clone()));
        }
        if let Some(value) = &params.ps_query {
            query.push(("ps".to_string(), value.clone()));
        }
        if let Some(value) = &params.pn_query {
            query.push(("pn".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 B站用户
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_userinfo(&self, params: GetSocialBilibiliUserinfoParams) -> Result<crate::models::GetSocialBilibiliUserinfo200Response> {
        let mut path = "/api/v1/social/bilibili/userinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("uid".to_string(), params.uid_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 B站视频
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_videoinfo(&self, params: GetSocialBilibiliVideoinfoParams) -> Result<crate::models::GetSocialBilibiliVideoinfo200Response> {
        let mut path = "/api/v1/social/bilibili/videoinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.aid_query {
            query.push(("aid".to_string(), value.clone()));
        }
        if let Some(value) = &params.bvid_query {
            query.push(("bvid".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 QQ 群信息
    #[instrument(skip(self, params))]
    pub async fn get_social_qq_groupinfo(&self, params: GetSocialQqGroupinfoParams) -> Result<crate::models::GetSocialQqGroupinfo200Response> {
        let mut path = "/api/v1/social/qq/groupinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("group_id".to_string(), params.group_id_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 查询 QQ 信息
    #[instrument(skip(self, params))]
    pub async fn get_social_qq_userinfo(&self, params: GetSocialQqUserinfoParams) -> Result<crate::models::GetSocialQqUserinfo200Response> {
        let mut path = "/api/v1/social/qq/userinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("qq".to_string(), params.qq_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetGithubRepoParams {
    pub repo_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetGithubRepoParams {
    pub fn new(repo_query: impl Into<String>) -> Self {
        Self {
            repo_query: repo_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetGithubUserParams {
    pub user_query: String,
    pub activity_query: Option<String>,
    pub activity_scope_query: Option<String>,
    pub org_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetGithubUserParams {
    pub fn new(user_query: impl Into<String>) -> Self {
        Self {
            user_query: user_query.into(),
            activity_query: None,
            activity_scope_query: None,
            org_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn activity_query(mut self, value: impl Into<String>) -> Self {
        self.activity_query = Some(value.into());
        self
    }
    pub fn activity_scope_query(mut self, value: impl Into<String>) -> Self {
        self.activity_scope_query = Some(value.into());
        self
    }
    pub fn org_query(mut self, value: impl Into<String>) -> Self {
        self.org_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliArchivesParams {
    pub mid_query: String,
    pub keywords_query: Option<String>,
    pub orderby_query: Option<String>,
    pub ps_query: Option<String>,
    pub pn_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialBilibiliArchivesParams {
    pub fn new(mid_query: impl Into<String>) -> Self {
        Self {
            mid_query: mid_query.into(),
            keywords_query: None,
            orderby_query: None,
            ps_query: None,
            pn_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn keywords_query(mut self, value: impl Into<String>) -> Self {
        self.keywords_query = Some(value.into());
        self
    }
    pub fn orderby_query(mut self, value: impl Into<String>) -> Self {
        self.orderby_query = Some(value.into());
        self
    }
    pub fn ps_query(mut self, value: impl Into<String>) -> Self {
        self.ps_query = Some(value.into());
        self
    }
    pub fn pn_query(mut self, value: impl Into<String>) -> Self {
        self.pn_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliLiveroomParams {
    pub mid_query: Option<String>,
    pub room_id_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialBilibiliLiveroomParams {
    pub fn new() -> Self {
        Self {
            mid_query: None,
            room_id_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn mid_query(mut self, value: impl Into<String>) -> Self {
        self.mid_query = Some(value.into());
        self
    }
    pub fn room_id_query(mut self, value: impl Into<String>) -> Self {
        self.room_id_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliRepliesParams {
    pub oid_query: String,
    pub sort_query: Option<String>,
    pub ps_query: Option<String>,
    pub pn_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialBilibiliRepliesParams {
    pub fn new(oid_query: impl Into<String>) -> Self {
        Self {
            oid_query: oid_query.into(),
            sort_query: None,
            ps_query: None,
            pn_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn sort_query(mut self, value: impl Into<String>) -> Self {
        self.sort_query = Some(value.into());
        self
    }
    pub fn ps_query(mut self, value: impl Into<String>) -> Self {
        self.ps_query = Some(value.into());
        self
    }
    pub fn pn_query(mut self, value: impl Into<String>) -> Self {
        self.pn_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliUserinfoParams {
    pub uid_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialBilibiliUserinfoParams {
    pub fn new(uid_query: impl Into<String>) -> Self {
        Self {
            uid_query: uid_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliVideoinfoParams {
    pub aid_query: Option<String>,
    pub bvid_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialBilibiliVideoinfoParams {
    pub fn new() -> Self {
        Self {
            aid_query: None,
            bvid_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn aid_query(mut self, value: impl Into<String>) -> Self {
        self.aid_query = Some(value.into());
        self
    }
    pub fn bvid_query(mut self, value: impl Into<String>) -> Self {
        self.bvid_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialQqGroupinfoParams {
    pub group_id_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialQqGroupinfoParams {
    pub fn new(group_id_query: impl Into<String>) -> Self {
        Self {
            group_id_query: group_id_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialQqUserinfoParams {
    pub qq_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSocialQqUserinfoParams {
    pub fn new(qq_query: impl Into<String>) -> Self {
        Self {
            qq_query: qq_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct StatusService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> StatusService<'a> {
/// 限流状态
    #[instrument(skip(self, params))]
    pub async fn get_status_ratelimit(&self, params: GetStatusRatelimitParams) -> Result<crate::models::GetStatusRatelimit200Response> {
        let mut path = "/api/v1/status/ratelimit".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        extra_headers.insert(
            "Authorization",
            HeaderValue::from_str(&params.authorization_header).map_err(|_| Error::InvalidHeader { name: "Authorization".into() })?,
        );
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 获取API端点使用统计
    #[instrument(skip(self, params))]
    pub async fn get_status_usage(&self, params: GetStatusUsageParams) -> Result<crate::models::GetStatusUsage200Response> {
        let mut path = "/api/v1/status/usage".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.path_query {
            query.push(("path".to_string(), value.clone()));
        }
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetStatusRatelimitParams {
    pub authorization_header: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetStatusRatelimitParams {
    pub fn new(authorization_header: impl Into<String>) -> Self {
        Self {
            authorization_header: authorization_header.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetStatusUsageParams {
    pub path_query: Option<String>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetStatusUsageParams {
    pub fn new() -> Self {
        Self {
            path_query: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn path_query(mut self, value: impl Into<String>) -> Self {
        self.path_query = Some(value.into());
        self
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct TextService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> TextService<'a> {
/// MD5 哈希
    #[instrument(skip(self, params))]
    pub async fn get_text_md_5(&self, params: GetTextMd5Params) -> Result<crate::models::GetTextMd5200Response> {
        let mut path = "/api/v1/text/md5".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("text".to_string(), params.text_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// AES 解密
    #[instrument(skip(self, params))]
    pub async fn post_text_aes_decrypt(&self, params: PostTextAesDecryptParams) -> Result<crate::models::PostTextAesDecrypt200Response> {
        let mut path = "/api/v1/text/aes/decrypt".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// AES高级解密
    #[instrument(skip(self, params))]
    pub async fn post_text_aes_decrypt_advanced(&self, params: PostTextAesDecryptAdvancedParams) -> Result<crate::models::PostTextAesDecryptAdvanced200Response> {
        let mut path = "/api/v1/text/aes/decrypt-advanced".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// AES 加密
    #[instrument(skip(self, params))]
    pub async fn post_text_aes_encrypt(&self, params: PostTextAesEncryptParams) -> Result<crate::models::PostTextAesEncrypt200Response> {
        let mut path = "/api/v1/text/aes/encrypt".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// AES高级加密
    #[instrument(skip(self, params))]
    pub async fn post_text_aes_encrypt_advanced(&self, params: PostTextAesEncryptAdvancedParams) -> Result<crate::models::PostTextAesEncryptAdvanced200Response> {
        let mut path = "/api/v1/text/aes/encrypt-advanced".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 文本分析
    #[instrument(skip(self, params))]
    pub async fn post_text_analyze(&self, params: PostTextAnalyzeParams) -> Result<crate::models::PostTextAnalyze200Response> {
        let mut path = "/api/v1/text/analyze".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// Base64 解码
    #[instrument(skip(self, params))]
    pub async fn post_text_base_64_decode(&self, params: PostTextBase64DecodeParams) -> Result<crate::models::PostTextBase64Decode200Response> {
        let mut path = "/api/v1/text/base64/decode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// Base64 编码
    #[instrument(skip(self, params))]
    pub async fn post_text_base_64_encode(&self, params: PostTextBase64EncodeParams) -> Result<crate::models::PostTextBase64Encode200Response> {
        let mut path = "/api/v1/text/base64/encode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 格式转换
    #[instrument(skip(self, params))]
    pub async fn post_text_convert(&self, params: PostTextConvertParams) -> Result<crate::models::PostTextConvert200Response> {
        let mut path = "/api/v1/text/convert".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// Markdown 转 HTML
    #[instrument(skip(self, params))]
    pub async fn post_text_markdown_to_html(&self, params: PostTextMarkdownToHtmlParams) -> Result<crate::models::PostTextMarkdownToHtml200Response> {
        let mut path = "/api/v1/text/markdown-to-html".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// Markdown 转 PDF
    #[instrument(skip(self, params))]
    pub async fn post_text_markdown_to_pdf(&self, params: PostTextMarkdownToPdfParams) -> Result<Vec<u8>> {
        let mut path = "/api/v1/text/markdown-to-pdf".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// MD5 哈希 (POST)
    #[instrument(skip(self, params))]
    pub async fn post_text_md_5(&self, params: PostTextMd5Params) -> Result<crate::models::GetTextMd5200Response> {
        let mut path = "/api/v1/text/md5".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// MD5 校验
    #[instrument(skip(self, params))]
    pub async fn post_text_md_5_verify(&self, params: PostTextMd5VerifyParams) -> Result<crate::models::PostTextMd5Verify200Response> {
        let mut path = "/api/v1/text/md5/verify".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetTextMd5Params {
    pub text_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetTextMd5Params {
    pub fn new(text_query: impl Into<String>) -> Self {
        Self {
            text_query: text_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAesDecryptParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextAesDecryptParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAesDecryptAdvancedParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextAesDecryptAdvancedParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAesEncryptParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextAesEncryptParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAesEncryptAdvancedParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextAesEncryptAdvancedParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAnalyzeParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextAnalyzeParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextBase64DecodeParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextBase64DecodeParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextBase64EncodeParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextBase64EncodeParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextConvertParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextConvertParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextMarkdownToHtmlParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextMarkdownToHtmlParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextMarkdownToPdfParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextMarkdownToPdfParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextMd5Params {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextMd5Params {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextMd5VerifyParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTextMd5VerifyParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct TranslateService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> TranslateService<'a> {
/// AI翻译配置
    #[instrument(skip(self))]
    pub async fn get_ai_translate_languages(&self) -> Result<crate::models::GetAiTranslateLanguages200Response> {
        let mut path = "/api/v1/ai/translate/languages".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
/// AI智能翻译
    #[instrument(skip(self, params))]
    pub async fn post_ai_translate(&self, params: PostAiTranslateParams) -> Result<crate::models::PostAiTranslate200Response> {
        let mut path = "/api/v1/ai/translate".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("target_lang".to_string(), params.target_lang_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 流式翻译（中英互译）
    #[instrument(skip(self, params))]
    pub async fn post_translate_stream(&self, params: PostTranslateStreamParams) -> Result<String> {
        let mut path = "/api/v1/translate/stream".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 翻译
    #[instrument(skip(self, params))]
    pub async fn post_translate_text(&self, params: PostTranslateTextParams) -> Result<crate::models::PostTranslateText200Response> {
        let mut path = "/api/v1/translate/text".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("to_lang".to_string(), params.to_lang_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}


#[derive(Debug, Clone)]
pub struct PostAiTranslateParams {
    pub target_lang_query: String,
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostAiTranslateParams {
    pub fn new(target_lang_query: impl Into<String>) -> Self {
        Self {
            target_lang_query: target_lang_query.into(),
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTranslateStreamParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTranslateStreamParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTranslateTextParams {
    pub to_lang_query: String,
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostTranslateTextParams {
    pub fn new(to_lang_query: impl Into<String>) -> Self {
        Self {
            to_lang_query: to_lang_query.into(),
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct WebparseService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> WebparseService<'a> {
/// 转换任务状态
    #[instrument(skip(self, params))]
    pub async fn get_web_tomarkdown_async_status(&self, params: GetWebTomarkdownAsyncStatusParams) -> Result<crate::models::GetWebTomarkdownAsyncStatus200Response> {
        let mut path = "/api/v1/web/tomarkdown/async/{task_id}".to_string();
        {
            let encoded = encode(&params.task_id_path).into_owned();
            path = path.replace("{task_id}", &encoded);
        }

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 提取网页图片
    #[instrument(skip(self, params))]
    pub async fn get_webparse_extractimages(&self, params: GetWebparseExtractimagesParams) -> Result<crate::models::GetWebparseExtractimages200Response> {
        let mut path = "/api/v1/webparse/extractimages".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 提取网页元数据
    #[instrument(skip(self, params))]
    pub async fn get_webparse_metadata(&self, params: GetWebparseMetadataParams) -> Result<crate::models::GetWebparseMetadata200Response> {
        let mut path = "/api/v1/webparse/metadata".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 网页转 Markdown
    #[instrument(skip(self, params))]
    pub async fn post_web_tomarkdown_async(&self, params: PostWebTomarkdownAsyncParams) -> Result<crate::models::PostWebTomarkdownAsync202Response> {
        let mut path = "/api/v1/web/tomarkdown/async".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetWebTomarkdownAsyncStatusParams {
    pub task_id_path: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetWebTomarkdownAsyncStatusParams {
    pub fn new(task_id_path: impl Into<String>) -> Self {
        Self {
            task_id_path: task_id_path.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetWebparseExtractimagesParams {
    pub url_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetWebparseExtractimagesParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetWebparseMetadataParams {
    pub url_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetWebparseMetadataParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostWebTomarkdownAsyncParams {
    pub url_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostWebTomarkdownAsyncParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct MinGanCiShiBieService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> MinGanCiShiBieService<'a> {
/// 敏感词分析 (GET)
    #[instrument(skip(self, params))]
    pub async fn get_sensitive_word_analyze_query(&self, params: GetSensitiveWordAnalyzeQueryParams) -> Result<crate::models::PostSensitiveWordAnalyze200Response> {
        let mut path = "/api/v1/sensitive-word/analyze-query".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("keyword".to_string(), params.keyword_query.clone()));
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 分析敏感词
    #[instrument(skip(self, params))]
    pub async fn post_sensitive_word_analyze(&self, params: PostSensitiveWordAnalyzeParams) -> Result<crate::models::PostSensitiveWordAnalyze200Response> {
        let mut path = "/api/v1/sensitive-word/analyze".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
/// 敏感词检测（快速）
    #[instrument(skip(self, params))]
    pub async fn post_sensitive_word_quick_check(&self, params: PostSensitiveWordQuickCheckParams) -> Result<crate::models::PostSensitiveWordQuickCheck200Response> {
        let mut path = "/api/v1/text/profanitycheck".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetSensitiveWordAnalyzeQueryParams {
    pub keyword_query: String,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl GetSensitiveWordAnalyzeQueryParams {
    pub fn new(keyword_query: impl Into<String>) -> Self {
        Self {
            keyword_query: keyword_query.into(),
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostSensitiveWordAnalyzeParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostSensitiveWordAnalyzeParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostSensitiveWordQuickCheckParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostSensitiveWordQuickCheckParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ZhiNengSouSuoService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ZhiNengSouSuoService<'a> {
/// 搜索引擎配置
    #[instrument(skip(self))]
    pub async fn get_search_engines(&self) -> Result<crate::models::GetSearchEngines200Response> {
        let mut path = "/api/v1/search/engines".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = None;

        self.client
            .request_json(
                Method::GET,
                &path,
                headers,
                query,
                body,
                None,
            )
            .await
    }
/// 智能搜索
    #[instrument(skip(self, params))]
    pub async fn post_search_aggregate(&self, params: PostSearchAggregateParams) -> Result<crate::models::PostSearchAggregate200Response> {
        let mut path = "/api/v1/search/aggregate".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params._t {
            query.push(("_t".to_string(), value.clone()));
        }
        let query = if query.is_empty() { None } else { Some(query) };

        let mut extra_headers = HeaderMap::new();
        let headers = if extra_headers.is_empty() { None } else { Some(extra_headers) };
        let body = params.body.clone();

        self.client
            .request_json(
                Method::POST,
                &path,
                headers,
                query,
                body,
                params.disable_cache,
            )
            .await
    }
}


#[derive(Debug, Clone)]
pub struct PostSearchAggregateParams {
    pub body: Option<Value>,
    pub disable_cache: Option<bool>,
    pub _t: Option<String>,
}

impl PostSearchAggregateParams {
    pub fn new() -> Self {
        Self {
            body: None,
            disable_cache: None,
            _t: None,
        }
    }
    pub fn disable_cache(mut self, value: bool) -> Self {
        self.disable_cache = Some(value);
        self
    }
    pub fn _t(mut self, value: impl Into<String>) -> Self {
        self._t = Some(value.into());
        self
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
