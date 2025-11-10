use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftUserInfo {
    pub code: i32,
    pub username: String,
    pub uuid: String,
    #[serde(default)]
    pub skin_url: Option<String>,
}
