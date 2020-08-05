use std::thread;

mod db;
mod extractor;
mod http;
mod jwt;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();
    logger::init_logger();

    let settings = core::settings::init().await?;
    let db_pool = db::get_pool(&settings).await?;

    let handle = thread::spawn(|| http::run(db_pool, settings));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
