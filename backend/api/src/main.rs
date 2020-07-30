use core::settings::SETTINGS;

mod db;
mod extractor;
mod http;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::from_path("../../../config/.env");
    logger::init_logger();

    core::settings::init().await;
    let db_pool = db::get_pool(&SETTINGS.get().unwrap()).await?;

    http::run(db_pool).await?;
    log::info!("app started!");

    Ok(())
}
