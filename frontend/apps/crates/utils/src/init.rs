pub mod settings;
pub mod user;

pub async fn init() {
    settings::init().await;
    user::init().await;
}
