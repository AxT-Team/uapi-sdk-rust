use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqUserInfo {
    pub qq: String,
    pub nickname: Option<String>,
    pub long_nick: Option<String>,
    pub avatar_url: Option<String>,
    pub age: Option<i32>,
    pub sex: Option<String>,
    pub qid: Option<String>,
    pub qq_level: Option<i32>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub is_vip: Option<bool>,
    pub vip_level: Option<i32>,
    pub reg_time: Option<String>,
    pub last_updated: Option<String>,
}
