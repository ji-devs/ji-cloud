//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

extern crate openssl;

mod logger;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();
    logger::init_logger();
    server::start().await;
    log::info!("app started!");
}
