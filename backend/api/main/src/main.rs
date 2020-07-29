mod logger;

#[tokio::main]
async fn main() {
    let _ = dotenv::from_path("../../../config/.env");
    logger::init_logger();
    server::start().await;
    log::info!("app started!");
}
