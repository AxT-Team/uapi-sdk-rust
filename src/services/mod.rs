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
    pub async fn get_clipzy_get(&self, params: GetClipzyGetParams) -> Result<Value> {
        let mut path = "/api/get".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("id".to_string(), params.id_query.clone()));
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
            )
            .await
    }
/// 步骤2 (方法二): 获取原始文本
    #[instrument(skip(self, params))]
    pub async fn get_clipzy_raw(&self, params: GetClipzyRawParams) -> Result<Value> {
        let mut path = "/api/raw/{id}".to_string();
        {
            let encoded = encode(&params.id_path).into_owned();
            path = path.replace("{id}", &encoded);
        }

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("key".to_string(), params.key_query.clone()));
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
            )
            .await
    }
/// 步骤1：上传加密数据
    #[instrument(skip(self, params))]
    pub async fn post_clipzy_store(&self, params: PostClipzyStoreParams) -> Result<Value> {
        let mut path = "/api/store".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetClipzyGetParams {
    pub id_query: String,
}

impl GetClipzyGetParams {
    pub fn new(id_query: impl Into<String>) -> Self {
        Self {
            id_query: id_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetClipzyRawParams {
    pub id_path: String,
    pub key_query: String,
}

impl GetClipzyRawParams {
    pub fn new(id_path: impl Into<String>, key_query: impl Into<String>) -> Self {
        Self {
            id_path: id_path.into(),
            key_query: key_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostClipzyStoreParams {
    pub body: Option<Value>,
}

impl PostClipzyStoreParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
/// Unix时间戳与日期字符串双向转换
    #[instrument(skip(self, params))]
    pub async fn get_convert_unixtime(&self, params: GetConvertUnixtimeParams) -> Result<Value> {
        let mut path = "/convert/unixtime".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("time".to_string(), params.time_query.clone()));
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
            )
            .await
    }
/// 美化并格式化JSON字符串
    #[instrument(skip(self, params))]
    pub async fn post_convert_json(&self, params: PostConvertJsonParams) -> Result<Value> {
        let mut path = "/convert/json".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetConvertUnixtimeParams {
    pub time_query: String,
}

impl GetConvertUnixtimeParams {
    pub fn new(time_query: impl Into<String>) -> Self {
        Self {
            time_query: time_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostConvertJsonParams {
    pub body: Option<Value>,
}

impl PostConvertJsonParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
/// 生成每日新闻摘要图片
    #[instrument(skip(self))]
    pub async fn get_daily_news_image(&self) -> Result<Value> {
        let mut path = "/daily/news-image".to_string();

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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GameService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> GameService<'a> {
/// 获取Epic Games免费游戏
    #[instrument(skip(self))]
    pub async fn get_game_epic_free(&self) -> Result<Value> {
        let mut path = "/game/epic-free".to_string();

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
            )
            .await
    }
/// 查询Minecraft玩家历史用户名
    #[instrument(skip(self, params))]
    pub async fn get_game_minecraft_historyid(&self, params: GetGameMinecraftHistoryidParams) -> Result<Value> {
        let mut path = "/game/minecraft/historyid".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("uuid".to_string(), params.uuid_query.clone()));
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
            )
            .await
    }
/// 查询Minecraft服务器状态
    #[instrument(skip(self, params))]
    pub async fn get_game_minecraft_serverstatus(&self, params: GetGameMinecraftServerstatusParams) -> Result<Value> {
        let mut path = "/game/minecraft/serverstatus".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("server".to_string(), params.server_query.clone()));
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
            )
            .await
    }
/// 查询Minecraft玩家信息
    #[instrument(skip(self, params))]
    pub async fn get_game_minecraft_userinfo(&self, params: GetGameMinecraftUserinfoParams) -> Result<Value> {
        let mut path = "/game/minecraft/userinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("username".to_string(), params.username_query.clone()));
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
            )
            .await
    }
/// 获取Steam用户公开摘要
    #[instrument(skip(self, params))]
    pub async fn get_game_steam_summary(&self, params: GetGameSteamSummaryParams) -> Result<Value> {
        let mut path = "/game/steam/summary".to_string();

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
            )
            .await
    }
}


#[derive(Debug, Clone)]
pub struct GetGameMinecraftHistoryidParams {
    pub uuid_query: String,
}

impl GetGameMinecraftHistoryidParams {
    pub fn new(uuid_query: impl Into<String>) -> Self {
        Self {
            uuid_query: uuid_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetGameMinecraftServerstatusParams {
    pub server_query: String,
}

impl GetGameMinecraftServerstatusParams {
    pub fn new(server_query: impl Into<String>) -> Self {
        Self {
            server_query: server_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetGameMinecraftUserinfoParams {
    pub username_query: String,
}

impl GetGameMinecraftUserinfoParams {
    pub fn new(username_query: impl Into<String>) -> Self {
        Self {
            username_query: username_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetGameSteamSummaryParams {
    pub steamid_query: Option<String>,
    pub id_query: Option<String>,
    pub id_3_query: Option<String>,
    pub key_query: Option<String>,
}

impl GetGameSteamSummaryParams {
    pub fn new() -> Self {
        Self {
            steamid_query: None,
            id_query: None,
            id_3_query: None,
            key_query: None,
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
}
#[derive(Debug, Clone)]
pub struct ImageService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ImageService<'a> {
/// 获取Gravatar头像
    #[instrument(skip(self, params))]
    pub async fn get_avatar_gravatar(&self, params: GetAvatarGravatarParams) -> Result<Value> {
        let mut path = "/avatar/gravatar".to_string();

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
            )
            .await
    }
/// 获取必应每日壁纸
    #[instrument(skip(self))]
    pub async fn get_image_bing_daily(&self) -> Result<Value> {
        let mut path = "/image/bing-daily".to_string();

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
            )
            .await
    }
/// 生成摸摸头GIF (QQ号方式)
    #[instrument(skip(self, params))]
    pub async fn get_image_motou(&self, params: GetImageMotouParams) -> Result<Value> {
        let mut path = "/image/motou".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("qq".to_string(), params.qq_query.clone()));
        if let Some(value) = &params.bg_color_query {
            query.push(("bg_color".to_string(), value.clone()));
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
            )
            .await
    }
/// 动态生成二维码
    #[instrument(skip(self, params))]
    pub async fn get_image_qrcode(&self, params: GetImageQrcodeParams) -> Result<Value> {
        let mut path = "/image/qrcode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("text".to_string(), params.text_query.clone()));
        if let Some(value) = &params.size_query {
            query.push(("size".to_string(), value.clone()));
        }
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
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
            )
            .await
    }
/// 将在线图片转换为Base64
    #[instrument(skip(self, params))]
    pub async fn get_image_tobase_64(&self, params: GetImageTobase64Params) -> Result<Value> {
        let mut path = "/image/tobase64".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
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
            )
            .await
    }
/// 无损压缩图片
    #[instrument(skip(self, params))]
    pub async fn post_image_compress(&self, params: PostImageCompressParams) -> Result<Value> {
        let mut path = "/image/compress".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.level_query {
            query.push(("level".to_string(), value.clone()));
        }
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
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
            )
            .await
    }
/// 通过Base64编码上传图片
    #[instrument(skip(self, params))]
    pub async fn post_image_frombase_64(&self, params: PostImageFrombase64Params) -> Result<Value> {
        let mut path = "/image/frombase64".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 生成摸摸头GIF (图片上传或URL方式)
    #[instrument(skip(self, params))]
    pub async fn post_image_motou(&self, params: PostImageMotouParams) -> Result<Value> {
        let mut path = "/image/motou".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 生成你们怎么不说话了表情包
    #[instrument(skip(self, params))]
    pub async fn post_image_speechless(&self, params: PostImageSpeechlessParams) -> Result<Value> {
        let mut path = "/image/speechless".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// SVG转图片
    #[instrument(skip(self, params))]
    pub async fn post_image_svg(&self, params: PostImageSvgParams) -> Result<Value> {
        let mut path = "/image/svg".to_string();

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
}

impl GetAvatarGravatarParams {
    pub fn new() -> Self {
        Self {
            email_query: None,
            hash_query: None,
            s_query: None,
            d_query: None,
            r_query: None,
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
}


#[derive(Debug, Clone)]
pub struct GetImageMotouParams {
    pub qq_query: String,
    pub bg_color_query: Option<String>,
}

impl GetImageMotouParams {
    pub fn new(qq_query: impl Into<String>) -> Self {
        Self {
            qq_query: qq_query.into(),
            bg_color_query: None,
        }
    }
    pub fn bg_color_query(mut self, value: impl Into<String>) -> Self {
        self.bg_color_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetImageQrcodeParams {
    pub text_query: String,
    pub size_query: Option<String>,
    pub format_query: Option<String>,
}

impl GetImageQrcodeParams {
    pub fn new(text_query: impl Into<String>) -> Self {
        Self {
            text_query: text_query.into(),
            size_query: None,
            format_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetImageTobase64Params {
    pub url_query: String,
}

impl GetImageTobase64Params {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostImageCompressParams {
    pub level_query: Option<String>,
    pub format_query: Option<String>,
    pub body: Option<Value>,
}

impl PostImageCompressParams {
    pub fn new() -> Self {
        Self {
            level_query: None,
            format_query: None,
            body: None,
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
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageFrombase64Params {
    pub body: Option<Value>,
}

impl PostImageFrombase64Params {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageMotouParams {
    pub body: Option<Value>,
}

impl PostImageMotouParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostImageSpeechlessParams {
    pub body: Option<Value>,
}

impl PostImageSpeechlessParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
}

impl PostImageSvgParams {
    pub fn new() -> Self {
        Self {
            format_query: None,
            width_query: None,
            height_query: None,
            quality_query: None,
            body: None,
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
/// 获取指定日期的程序员历史事件
    #[instrument(skip(self, params))]
    pub async fn get_history_programmer(&self, params: GetHistoryProgrammerParams) -> Result<Value> {
        let mut path = "/history/programmer".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("month".to_string(), params.month_query.clone()));
        query.push(("day".to_string(), params.day_query.clone()));
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
            )
            .await
    }
/// 获取今天的程序员历史事件
    #[instrument(skip(self))]
    pub async fn get_history_programmer_today(&self) -> Result<Value> {
        let mut path = "/history/programmer/today".to_string();

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
            )
            .await
    }
/// 获取多平台实时热榜
    #[instrument(skip(self, params))]
    pub async fn get_misc_hotboard(&self, params: GetMiscHotboardParams) -> Result<Value> {
        let mut path = "/misc/hotboard".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("type".to_string(), params.type_query.clone()));
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
            )
            .await
    }
/// 查询手机号码归属地信息
    #[instrument(skip(self, params))]
    pub async fn get_misc_phoneinfo(&self, params: GetMiscPhoneinfoParams) -> Result<Value> {
        let mut path = "/misc/phoneinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("phone".to_string(), params.phone_query.clone()));
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
            )
            .await
    }
/// 生成高度可定制的随机数
    #[instrument(skip(self, params))]
    pub async fn get_misc_randomnumber(&self, params: GetMiscRandomnumberParams) -> Result<Value> {
        let mut path = "/misc/randomnumber".to_string();

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
            )
            .await
    }
/// 转换时间戳 (旧版，推荐使用/convert/unixtime)
    #[instrument(skip(self, params))]
    pub async fn get_misc_timestamp(&self, params: GetMiscTimestampParams) -> Result<Value> {
        let mut path = "/misc/timestamp".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("ts".to_string(), params.ts_query.clone()));
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
            )
            .await
    }
/// 获取支持的快递公司列表
    #[instrument(skip(self))]
    pub async fn get_misc_tracking_carriers(&self) -> Result<Value> {
        let mut path = "/misc/tracking/carriers".to_string();

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
            )
            .await
    }
/// 识别快递公司
    #[instrument(skip(self, params))]
    pub async fn get_misc_tracking_detect(&self, params: GetMiscTrackingDetectParams) -> Result<Value> {
        let mut path = "/misc/tracking/detect".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("tracking_number".to_string(), params.tracking_number_query.clone()));
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
            )
            .await
    }
/// 查询快递物流信息
    #[instrument(skip(self, params))]
    pub async fn get_misc_tracking_query(&self, params: GetMiscTrackingQueryParams) -> Result<Value> {
        let mut path = "/misc/tracking/query".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("tracking_number".to_string(), params.tracking_number_query.clone()));
        if let Some(value) = &params.carrier_code_query {
            query.push(("carrier_code".to_string(), value.clone()));
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
            )
            .await
    }
/// 查询实时天气信息
    #[instrument(skip(self, params))]
    pub async fn get_misc_weather(&self, params: GetMiscWeatherParams) -> Result<Value> {
        let mut path = "/misc/weather".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.city_query {
            query.push(("city".to_string(), value.clone()));
        }
        if let Some(value) = &params.adcode_query {
            query.push(("adcode".to_string(), value.clone()));
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
            )
            .await
    }
/// 查询全球任意时区的时间
    #[instrument(skip(self, params))]
    pub async fn get_misc_worldtime(&self, params: GetMiscWorldtimeParams) -> Result<Value> {
        let mut path = "/misc/worldtime".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("city".to_string(), params.city_query.clone()));
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetHistoryProgrammerParams {
    pub month_query: String,
    pub day_query: String,
}

impl GetHistoryProgrammerParams {
    pub fn new(month_query: impl Into<String>, day_query: impl Into<String>) -> Self {
        Self {
            month_query: month_query.into(),
            day_query: day_query.into(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct GetMiscHotboardParams {
    pub type_query: String,
}

impl GetMiscHotboardParams {
    pub fn new(type_query: impl Into<String>) -> Self {
        Self {
            type_query: type_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscPhoneinfoParams {
    pub phone_query: String,
}

impl GetMiscPhoneinfoParams {
    pub fn new(phone_query: impl Into<String>) -> Self {
        Self {
            phone_query: phone_query.into(),
        }
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
}

#[derive(Debug, Clone)]
pub struct GetMiscTimestampParams {
    pub ts_query: String,
}

impl GetMiscTimestampParams {
    pub fn new(ts_query: impl Into<String>) -> Self {
        Self {
            ts_query: ts_query.into(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct GetMiscTrackingDetectParams {
    pub tracking_number_query: String,
}

impl GetMiscTrackingDetectParams {
    pub fn new(tracking_number_query: impl Into<String>) -> Self {
        Self {
            tracking_number_query: tracking_number_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscTrackingQueryParams {
    pub tracking_number_query: String,
    pub carrier_code_query: Option<String>,
}

impl GetMiscTrackingQueryParams {
    pub fn new(tracking_number_query: impl Into<String>) -> Self {
        Self {
            tracking_number_query: tracking_number_query.into(),
            carrier_code_query: None,
        }
    }
    pub fn carrier_code_query(mut self, value: impl Into<String>) -> Self {
        self.carrier_code_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMiscWeatherParams {
    pub city_query: Option<String>,
    pub adcode_query: Option<String>,
}

impl GetMiscWeatherParams {
    pub fn new() -> Self {
        Self {
            city_query: None,
            adcode_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetMiscWorldtimeParams {
    pub city_query: String,
}

impl GetMiscWorldtimeParams {
    pub fn new(city_query: impl Into<String>) -> Self {
        Self {
            city_query: city_query.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NetworkService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> NetworkService<'a> {
/// 执行DNS解析查询
    #[instrument(skip(self, params))]
    pub async fn get_network_dns(&self, params: GetNetworkDnsParams) -> Result<Value> {
        let mut path = "/network/dns".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
        if let Some(value) = &params.type_query {
            query.push(("type".to_string(), value.clone()));
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
            )
            .await
    }
/// 查询域名ICP备案信息
    #[instrument(skip(self, params))]
    pub async fn get_network_icp(&self, params: GetNetworkIcpParams) -> Result<Value> {
        let mut path = "/network/icp".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
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
            )
            .await
    }
/// 查询指定IP或域名的归属信息
    #[instrument(skip(self, params))]
    pub async fn get_network_ipinfo(&self, params: GetNetworkIpinfoParams) -> Result<Value> {
        let mut path = "/network/ipinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("ip".to_string(), params.ip_query.clone()));
        if let Some(value) = &params.source_query {
            query.push(("source".to_string(), value.clone()));
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
            )
            .await
    }
/// 获取你的公网IP及归属信息
    #[instrument(skip(self, params))]
    pub async fn get_network_myip(&self, params: GetNetworkMyipParams) -> Result<Value> {
        let mut path = "/network/myip".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.source_query {
            query.push(("source".to_string(), value.clone()));
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
            )
            .await
    }
/// 从服务器Ping指定主机
    #[instrument(skip(self, params))]
    pub async fn get_network_ping(&self, params: GetNetworkPingParams) -> Result<Value> {
        let mut path = "/network/ping".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("host".to_string(), params.host_query.clone()));
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
            )
            .await
    }
/// 从服务器Ping你的客户端IP
    #[instrument(skip(self))]
    pub async fn get_network_pingmyip(&self) -> Result<Value> {
        let mut path = "/network/pingmyip".to_string();

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
            )
            .await
    }
/// 扫描远程主机的指定端口
    #[instrument(skip(self, params))]
    pub async fn get_network_portscan(&self, params: GetNetworkPortscanParams) -> Result<Value> {
        let mut path = "/network/portscan".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("host".to_string(), params.host_query.clone()));
        query.push(("port".to_string(), params.port_query.clone()));
        if let Some(value) = &params.protocol_query {
            query.push(("protocol".to_string(), value.clone()));
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
            )
            .await
    }
/// 检查URL的可访问性状态
    #[instrument(skip(self, params))]
    pub async fn get_network_urlstatus(&self, params: GetNetworkUrlstatusParams) -> Result<Value> {
        let mut path = "/network/urlstatus".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
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
            )
            .await
    }
/// 查询域名的WHOIS注册信息
    #[instrument(skip(self, params))]
    pub async fn get_network_whois(&self, params: GetNetworkWhoisParams) -> Result<Value> {
        let mut path = "/network/whois".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
        if let Some(value) = &params.format_query {
            query.push(("format".to_string(), value.clone()));
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
            )
            .await
    }
/// 检查域名在微信中的访问状态
    #[instrument(skip(self, params))]
    pub async fn get_network_wxdomain(&self, params: GetNetworkWxdomainParams) -> Result<Value> {
        let mut path = "/network/wxdomain".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("domain".to_string(), params.domain_query.clone()));
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkDnsParams {
    pub domain_query: String,
    pub type_query: Option<String>,
}

impl GetNetworkDnsParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
            type_query: None,
        }
    }
    pub fn type_query(mut self, value: impl Into<String>) -> Self {
        self.type_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkIcpParams {
    pub domain_query: String,
}

impl GetNetworkIcpParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkIpinfoParams {
    pub ip_query: String,
    pub source_query: Option<String>,
}

impl GetNetworkIpinfoParams {
    pub fn new(ip_query: impl Into<String>) -> Self {
        Self {
            ip_query: ip_query.into(),
            source_query: None,
        }
    }
    pub fn source_query(mut self, value: impl Into<String>) -> Self {
        self.source_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkMyipParams {
    pub source_query: Option<String>,
}

impl GetNetworkMyipParams {
    pub fn new() -> Self {
        Self {
            source_query: None,
        }
    }
    pub fn source_query(mut self, value: impl Into<String>) -> Self {
        self.source_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkPingParams {
    pub host_query: String,
}

impl GetNetworkPingParams {
    pub fn new(host_query: impl Into<String>) -> Self {
        Self {
            host_query: host_query.into(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct GetNetworkPortscanParams {
    pub host_query: String,
    pub port_query: String,
    pub protocol_query: Option<String>,
}

impl GetNetworkPortscanParams {
    pub fn new(host_query: impl Into<String>, port_query: impl Into<String>) -> Self {
        Self {
            host_query: host_query.into(),
            port_query: port_query.into(),
            protocol_query: None,
        }
    }
    pub fn protocol_query(mut self, value: impl Into<String>) -> Self {
        self.protocol_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkUrlstatusParams {
    pub url_query: String,
}

impl GetNetworkUrlstatusParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkWhoisParams {
    pub domain_query: String,
    pub format_query: Option<String>,
}

impl GetNetworkWhoisParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
            format_query: None,
        }
    }
    pub fn format_query(mut self, value: impl Into<String>) -> Self {
        self.format_query = Some(value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetNetworkWxdomainParams {
    pub domain_query: String,
}

impl GetNetworkWxdomainParams {
    pub fn new(domain_query: impl Into<String>) -> Self {
        Self {
            domain_query: domain_query.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PoemService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> PoemService<'a> {
/// 随机获取一句诗词或名言
    #[instrument(skip(self))]
    pub async fn get_saying(&self) -> Result<Value> {
        let mut path = "/saying".to_string();

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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct RandomService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> RandomService<'a> {
/// 获取答案之书的神秘答案 (GET)
    #[instrument(skip(self, params))]
    pub async fn get_answerbook_ask(&self, params: GetAnswerbookAskParams) -> Result<Value> {
        let mut path = "/answerbook/ask".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("question".to_string(), params.question_query.clone()));
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
            )
            .await
    }
/// 随机二次元、风景、动漫图片壁纸
    #[instrument(skip(self, params))]
    pub async fn get_random_image(&self, params: GetRandomImageParams) -> Result<Value> {
        let mut path = "/random/image".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.category_query {
            query.push(("category".to_string(), value.clone()));
        }
        if let Some(value) = &params.type_query {
            query.push(("type".to_string(), value.clone()));
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
            )
            .await
    }
/// 生成高度可定制的随机字符串
    #[instrument(skip(self, params))]
    pub async fn get_random_string(&self, params: GetRandomStringParams) -> Result<Value> {
        let mut path = "/random/string".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.length_query {
            query.push(("length".to_string(), value.clone()));
        }
        if let Some(value) = &params.type_query {
            query.push(("type".to_string(), value.clone()));
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
            )
            .await
    }
/// 获取答案之书的神秘答案 (POST)
    #[instrument(skip(self, params))]
    pub async fn post_answerbook_ask(&self, params: PostAnswerbookAskParams) -> Result<Value> {
        let mut path = "/answerbook/ask".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetAnswerbookAskParams {
    pub question_query: String,
}

impl GetAnswerbookAskParams {
    pub fn new(question_query: impl Into<String>) -> Self {
        Self {
            question_query: question_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetRandomImageParams {
    pub category_query: Option<String>,
    pub type_query: Option<String>,
}

impl GetRandomImageParams {
    pub fn new() -> Self {
        Self {
            category_query: None,
            type_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetRandomStringParams {
    pub length_query: Option<String>,
    pub type_query: Option<String>,
}

impl GetRandomStringParams {
    pub fn new() -> Self {
        Self {
            length_query: None,
            type_query: None,
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
}

#[derive(Debug, Clone)]
pub struct PostAnswerbookAskParams {
    pub body: Option<Value>,
}

impl PostAnswerbookAskParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
/// 获取GitHub仓库信息
    #[instrument(skip(self, params))]
    pub async fn get_github_repo(&self, params: GetGithubRepoParams) -> Result<Value> {
        let mut path = "/github/repo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("repo".to_string(), params.repo_query.clone()));
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
            )
            .await
    }
/// 获取Bilibili用户投稿列表
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_archives(&self, params: GetSocialBilibiliArchivesParams) -> Result<Value> {
        let mut path = "/social/bilibili/archives".to_string();

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
            )
            .await
    }
/// 获取Bilibili直播间信息
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_liveroom(&self, params: GetSocialBilibiliLiveroomParams) -> Result<Value> {
        let mut path = "/social/bilibili/liveroom".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.mid_query {
            query.push(("mid".to_string(), value.clone()));
        }
        if let Some(value) = &params.room_id_query {
            query.push(("room_id".to_string(), value.clone()));
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
            )
            .await
    }
/// 获取Bilibili视频评论
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_replies(&self, params: GetSocialBilibiliRepliesParams) -> Result<Value> {
        let mut path = "/social/bilibili/replies".to_string();

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
            )
            .await
    }
/// 查询Bilibili用户信息
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_userinfo(&self, params: GetSocialBilibiliUserinfoParams) -> Result<Value> {
        let mut path = "/social/bilibili/userinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("uid".to_string(), params.uid_query.clone()));
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
            )
            .await
    }
/// 获取Bilibili视频详细信息
    #[instrument(skip(self, params))]
    pub async fn get_social_bilibili_videoinfo(&self, params: GetSocialBilibiliVideoinfoParams) -> Result<Value> {
        let mut path = "/social/bilibili/videoinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.aid_query {
            query.push(("aid".to_string(), value.clone()));
        }
        if let Some(value) = &params.bvid_query {
            query.push(("bvid".to_string(), value.clone()));
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
            )
            .await
    }
/// 获取QQ群名称、头像、简介
    #[instrument(skip(self, params))]
    pub async fn get_social_qq_groupinfo(&self, params: GetSocialQqGroupinfoParams) -> Result<Value> {
        let mut path = "/social/qq/groupinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("group_id".to_string(), params.group_id_query.clone()));
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
            )
            .await
    }
/// 独家获取QQ号头像、昵称
    #[instrument(skip(self, params))]
    pub async fn get_social_qq_userinfo(&self, params: GetSocialQqUserinfoParams) -> Result<Value> {
        let mut path = "/social/qq/userinfo".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("qq".to_string(), params.qq_query.clone()));
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetGithubRepoParams {
    pub repo_query: String,
}

impl GetGithubRepoParams {
    pub fn new(repo_query: impl Into<String>) -> Self {
        Self {
            repo_query: repo_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliArchivesParams {
    pub mid_query: String,
    pub keywords_query: Option<String>,
    pub orderby_query: Option<String>,
    pub ps_query: Option<String>,
    pub pn_query: Option<String>,
}

impl GetSocialBilibiliArchivesParams {
    pub fn new(mid_query: impl Into<String>) -> Self {
        Self {
            mid_query: mid_query.into(),
            keywords_query: None,
            orderby_query: None,
            ps_query: None,
            pn_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliLiveroomParams {
    pub mid_query: Option<String>,
    pub room_id_query: Option<String>,
}

impl GetSocialBilibiliLiveroomParams {
    pub fn new() -> Self {
        Self {
            mid_query: None,
            room_id_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliRepliesParams {
    pub oid_query: String,
    pub sort_query: Option<String>,
    pub ps_query: Option<String>,
    pub pn_query: Option<String>,
}

impl GetSocialBilibiliRepliesParams {
    pub fn new(oid_query: impl Into<String>) -> Self {
        Self {
            oid_query: oid_query.into(),
            sort_query: None,
            ps_query: None,
            pn_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliUserinfoParams {
    pub uid_query: String,
}

impl GetSocialBilibiliUserinfoParams {
    pub fn new(uid_query: impl Into<String>) -> Self {
        Self {
            uid_query: uid_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialBilibiliVideoinfoParams {
    pub aid_query: Option<String>,
    pub bvid_query: Option<String>,
}

impl GetSocialBilibiliVideoinfoParams {
    pub fn new() -> Self {
        Self {
            aid_query: None,
            bvid_query: None,
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
}

#[derive(Debug, Clone)]
pub struct GetSocialQqGroupinfoParams {
    pub group_id_query: String,
}

impl GetSocialQqGroupinfoParams {
    pub fn new(group_id_query: impl Into<String>) -> Self {
        Self {
            group_id_query: group_id_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetSocialQqUserinfoParams {
    pub qq_query: String,
}

impl GetSocialQqUserinfoParams {
    pub fn new(qq_query: impl Into<String>) -> Self {
        Self {
            qq_query: qq_query.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StatusService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> StatusService<'a> {
/// 获取API限流器实时状态
    #[instrument(skip(self, params))]
    pub async fn get_status_ratelimit(&self, params: GetStatusRatelimitParams) -> Result<Value> {
        let mut path = "/status/ratelimit".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 获取API端点使用统计
    #[instrument(skip(self, params))]
    pub async fn get_status_usage(&self, params: GetStatusUsageParams) -> Result<Value> {
        let mut path = "/status/usage".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(value) = &params.path_query {
            query.push(("path".to_string(), value.clone()));
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetStatusRatelimitParams {
    pub authorization_header: String,
}

impl GetStatusRatelimitParams {
    pub fn new(authorization_header: impl Into<String>) -> Self {
        Self {
            authorization_header: authorization_header.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetStatusUsageParams {
    pub path_query: Option<String>,
}

impl GetStatusUsageParams {
    pub fn new() -> Self {
        Self {
            path_query: None,
        }
    }
    pub fn path_query(mut self, value: impl Into<String>) -> Self {
        self.path_query = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct TextService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> TextService<'a> {
/// 计算文本的MD5哈希值(GET)
    #[instrument(skip(self, params))]
    pub async fn get_text_md_5(&self, params: GetTextMd5Params) -> Result<Value> {
        let mut path = "/text/md5".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("text".to_string(), params.text_query.clone()));
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
            )
            .await
    }
/// 使用AES算法解密文本
    #[instrument(skip(self, params))]
    pub async fn post_text_aes_decrypt(&self, params: PostTextAesDecryptParams) -> Result<Value> {
        let mut path = "/text/aes/decrypt".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 使用AES算法加密文本
    #[instrument(skip(self, params))]
    pub async fn post_text_aes_encrypt(&self, params: PostTextAesEncryptParams) -> Result<Value> {
        let mut path = "/text/aes/encrypt".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 多维度分析文本内容
    #[instrument(skip(self, params))]
    pub async fn post_text_analyze(&self, params: PostTextAnalyzeParams) -> Result<Value> {
        let mut path = "/text/analyze".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 解码Base64编码的文本
    #[instrument(skip(self, params))]
    pub async fn post_text_base_64_decode(&self, params: PostTextBase64DecodeParams) -> Result<Value> {
        let mut path = "/text/base64/decode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 将文本进行Base64编码
    #[instrument(skip(self, params))]
    pub async fn post_text_base_64_encode(&self, params: PostTextBase64EncodeParams) -> Result<Value> {
        let mut path = "/text/base64/encode".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 计算文本的MD5哈希值 (POST)
    #[instrument(skip(self, params))]
    pub async fn post_text_md_5(&self, params: PostTextMd5Params) -> Result<Value> {
        let mut path = "/text/md5".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 校验MD5哈希值
    #[instrument(skip(self, params))]
    pub async fn post_text_md_5_verify(&self, params: PostTextMd5VerifyParams) -> Result<Value> {
        let mut path = "/text/md5/verify".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetTextMd5Params {
    pub text_query: String,
}

impl GetTextMd5Params {
    pub fn new(text_query: impl Into<String>) -> Self {
        Self {
            text_query: text_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAesDecryptParams {
    pub body: Option<Value>,
}

impl PostTextAesDecryptParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAesEncryptParams {
    pub body: Option<Value>,
}

impl PostTextAesEncryptParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextAnalyzeParams {
    pub body: Option<Value>,
}

impl PostTextAnalyzeParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextBase64DecodeParams {
    pub body: Option<Value>,
}

impl PostTextBase64DecodeParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextBase64EncodeParams {
    pub body: Option<Value>,
}

impl PostTextBase64EncodeParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextMd5Params {
    pub body: Option<Value>,
}

impl PostTextMd5Params {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTextMd5VerifyParams {
    pub body: Option<Value>,
}

impl PostTextMd5VerifyParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
/// 获取AI翻译支持的语言和配置
    #[instrument(skip(self))]
    pub async fn get_ai_translate_languages(&self) -> Result<Value> {
        let mut path = "/ai/translate/languages".to_string();

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
            )
            .await
    }
/// AI智能翻译
    #[instrument(skip(self, params))]
    pub async fn post_ai_translate(&self, params: PostAiTranslateParams) -> Result<Value> {
        let mut path = "/ai/translate".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("target_lang".to_string(), params.target_lang_query.clone()));
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
            )
            .await
    }
/// 流式翻译（中英互译）
    #[instrument(skip(self, params))]
    pub async fn post_translate_stream(&self, params: PostTranslateStreamParams) -> Result<Value> {
        let mut path = "/translate/stream".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 多语言文本翻译
    #[instrument(skip(self, params))]
    pub async fn post_translate_text(&self, params: PostTranslateTextParams) -> Result<Value> {
        let mut path = "/translate/text".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("to_lang".to_string(), params.to_lang_query.clone()));
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
            )
            .await
    }
}


#[derive(Debug, Clone)]
pub struct PostAiTranslateParams {
    pub target_lang_query: String,
    pub body: Option<Value>,
}

impl PostAiTranslateParams {
    pub fn new(target_lang_query: impl Into<String>) -> Self {
        Self {
            target_lang_query: target_lang_query.into(),
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostTranslateStreamParams {
    pub body: Option<Value>,
}

impl PostTranslateStreamParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
}

impl PostTranslateTextParams {
    pub fn new(to_lang_query: impl Into<String>) -> Self {
        Self {
            to_lang_query: to_lang_query.into(),
            body: None,
        }
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
/// 查询网页转换任务状态和结果
    #[instrument(skip(self, params))]
    pub async fn get_web_tomarkdown_async_status(&self, params: GetWebTomarkdownAsyncStatusParams) -> Result<Value> {
        let mut path = "/web/tomarkdown/async/{task_id}".to_string();
        {
            let encoded = encode(&params.task_id_path).into_owned();
            path = path.replace("{task_id}", &encoded);
        }

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
            )
            .await
    }
/// 提取网页中的所有图片
    #[instrument(skip(self, params))]
    pub async fn get_webparse_extractimages(&self, params: GetWebparseExtractimagesParams) -> Result<Value> {
        let mut path = "/webparse/extractimages".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
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
            )
            .await
    }
/// 抓取并解析网页的元数据
    #[instrument(skip(self, params))]
    pub async fn get_webparse_metadata(&self, params: GetWebparseMetadataParams) -> Result<Value> {
        let mut path = "/webparse/metadata".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
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
            )
            .await
    }
/// 深度抓取网页转Markdown
    #[instrument(skip(self, params))]
    pub async fn post_web_tomarkdown_async(&self, params: PostWebTomarkdownAsyncParams) -> Result<Value> {
        let mut path = "/web/tomarkdown/async".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("url".to_string(), params.url_query.clone()));
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetWebTomarkdownAsyncStatusParams {
    pub task_id_path: String,
}

impl GetWebTomarkdownAsyncStatusParams {
    pub fn new(task_id_path: impl Into<String>) -> Self {
        Self {
            task_id_path: task_id_path.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetWebparseExtractimagesParams {
    pub url_query: String,
}

impl GetWebparseExtractimagesParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetWebparseMetadataParams {
    pub url_query: String,
}

impl GetWebparseMetadataParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostWebTomarkdownAsyncParams {
    pub url_query: String,
}

impl PostWebTomarkdownAsyncParams {
    pub fn new(url_query: impl Into<String>) -> Self {
        Self {
            url_query: url_query.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MinGanCiShiBieService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> MinGanCiShiBieService<'a> {
/// 查询参数分析
    #[instrument(skip(self, params))]
    pub async fn get_sensitive_word_analyze_query(&self, params: GetSensitiveWordAnalyzeQueryParams) -> Result<Value> {
        let mut path = "/sensitive-word/analyze-query".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("keyword".to_string(), params.keyword_query.clone()));
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
            )
            .await
    }
/// 分析敏感词
    #[instrument(skip(self, params))]
    pub async fn post_sensitive_word_analyze(&self, params: PostSensitiveWordAnalyzeParams) -> Result<Value> {
        let mut path = "/sensitive-word/analyze".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
/// 敏感词检测（快速）
    #[instrument(skip(self, params))]
    pub async fn post_sensitive_word_quick_check(&self, params: PostSensitiveWordQuickCheckParams) -> Result<Value> {
        let mut path = "/text/profanitycheck".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct GetSensitiveWordAnalyzeQueryParams {
    pub keyword_query: String,
}

impl GetSensitiveWordAnalyzeQueryParams {
    pub fn new(keyword_query: impl Into<String>) -> Self {
        Self {
            keyword_query: keyword_query.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostSensitiveWordAnalyzeParams {
    pub body: Option<Value>,
}

impl PostSensitiveWordAnalyzeParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PostSensitiveWordQuickCheckParams {
    pub body: Option<Value>,
}

impl PostSensitiveWordQuickCheckParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
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
/// 获取搜索引擎信息
    #[instrument(skip(self))]
    pub async fn get_search_engines(&self) -> Result<Value> {
        let mut path = "/search/engines".to_string();

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
            )
            .await
    }
/// 智能搜索
    #[instrument(skip(self, params))]
    pub async fn post_search_aggregate(&self, params: PostSearchAggregateParams) -> Result<Value> {
        let mut path = "/search/aggregate".to_string();

        let mut query: Vec<(String, String)> = Vec::new();
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
            )
            .await
    }
}


#[derive(Debug, Clone)]
pub struct PostSearchAggregateParams {
    pub body: Option<Value>,
}

impl PostSearchAggregateParams {
    pub fn new() -> Self {
        Self {
            body: None,
        }
    }
    pub fn body(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }
}
