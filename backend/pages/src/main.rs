//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[macro_use]
extern crate diesel;

mod settings;
mod logger;
mod server;
mod reject;
mod templates;
mod loader;
mod db;
mod user;
#[macro_use]
mod utils;

use dotenv::dotenv;
use server::start_server;

#[tokio::main]
async fn main() {
    dotenv().ok();

    logger::init_logger();

    start_server().await;
}
