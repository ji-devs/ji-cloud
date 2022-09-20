pub mod analytics;
pub mod settings;
pub mod user;

pub async fn init() {
    analytics::init();
    settings::init().await;
    user::init().await;
}
