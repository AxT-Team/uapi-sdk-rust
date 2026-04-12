use uapi_sdk_rust::{Client, Result};
use uapi_sdk_rust::services::GetGameMinecraftUserinfoParams;

#[tokio::main]
async fn main() -> Result<()> {
    let api = Client::from_env().unwrap_or_else(|| Client::new("YOUR_API_KEY"));
    let user = api.game().get_game_minecraft_userinfo(
        GetGameMinecraftUserinfoParams::new("Notch")
    ).await?;
    println!("{:?} -> {:?}", user.username, user.uuid);
    Ok(())
}
