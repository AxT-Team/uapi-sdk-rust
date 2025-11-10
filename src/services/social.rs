use crate::client::Client;
use crate::models::social::QqUserInfo;
use crate::Result;
use reqwest::Method;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct SocialService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> SocialService<'a> {
    /// 对应 GET /social/qq/userinfo
    #[instrument(skip(self))]
    pub async fn get_social_qq_userinfo(&self, qq: &str) -> Result<QqUserInfo> {
        let query = &[("qq", qq)];
        self.client
            .request_json::<QqUserInfo>(Method::GET, "/social/qq/userinfo", None, Some(query), None)
            .await
    }
}
