pub mod mixpanel;
pub mod settings;
pub mod user;

pub async fn init() {
    mixpanel::init();
    settings::init().await;
    user::init().await;
}
