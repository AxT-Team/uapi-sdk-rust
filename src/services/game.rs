use crate::client::Client;
use crate::models::game::MinecraftUserInfo;
use crate::Result;
use reqwest::Method;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct GameService<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> GameService<'a> {
    /// 对应 GET /game/minecraft/userinfo
    #[instrument(skip(self))]
    pub async fn get_minecraft_user_info(&self, username: &str) -> Result<MinecraftUserInfo> {
        let query = &[("username", username)];
        self.client
            .request_json::<MinecraftUserInfo>(Method::GET, "/game/minecraft/userinfo", None, Some(query), None)
            .await
    }
}
