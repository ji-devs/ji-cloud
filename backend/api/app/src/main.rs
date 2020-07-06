//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod logger;
mod db;
mod settings;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logger::init_logger();
    let settings = settings::init().await;
    server::settings::init().await;
    handler::settings::init().await;

    let db_pool = db::get_pool(&settings).await;

    server::start(db_pool).await;
    log::info!("app started!");
}
