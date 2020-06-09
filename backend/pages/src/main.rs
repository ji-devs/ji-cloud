//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

// see https://github.com/emk/rust-musl-builder/issues/69
// Order matters!
extern crate openssl;
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
    settings::init().await;

    start_server().await;
}
