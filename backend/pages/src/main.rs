//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod logger;
mod server;
mod reject;
mod templates;
mod loader;
mod user;
#[macro_use]
mod utils;

use dotenv::dotenv;
use server::start_server;
use core::settings::SETTINGS;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    logger::init_logger();

    core::settings::init().await; 

    start_server().await;
}

