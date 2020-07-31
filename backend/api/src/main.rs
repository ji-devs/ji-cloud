use core::settings::SETTINGS;

mod db;
mod extractor;
mod http;
mod jwt;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::from_path("../../../config/.env");
    let _ = dotenv::from_path(".env");
    logger::init_logger();

    core::settings::init().await;
    let db_pool = db::get_pool(&SETTINGS.get().expect("Settings aren't initialized?")).await?;

    http::run(db_pool).await?;
    log::info!("app started!");

    Ok(())
}
