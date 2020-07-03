//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]


mod models;
mod settings;
mod logger;
mod server;
mod user;
mod reject;
mod reply;
mod db;
#[macro_use]
mod utils;

use dotenv::dotenv;
use server::start_server;

#[tokio::main]
async fn main() {
    dotenv().ok();

    logger::init_logger();

    settings::init().await;

    start_server().await;
}
