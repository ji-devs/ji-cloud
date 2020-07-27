mod logger;

#[tokio::main]
async fn main() {
    dotenv::from_path("../../../config/.env").ok();
    logger::init_logger();
    server::start().await;
    log::info!("app started!");
}
