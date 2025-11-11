use uapi_sdk_rust::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api = Client::from_env().unwrap_or_else(|| Client::new("<TOKEN>"));
    let user = api.game().get_minecraft_user_info("Notch").await?;
    println!("{} -> {}", user.username, user.uuid);
    Ok(())
}
