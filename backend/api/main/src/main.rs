//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod logger;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv::from_path("../../../config/.env").ok();
    logger::init_logger();
    server::start().await;
    log::info!("app started!");
}
