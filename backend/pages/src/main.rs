use std::thread;

mod logger;
mod server;
mod templates;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    logger::init_logger()?;

    let settings = core::settings::init().await?.runtime;

    let handle = thread::spawn(|| server::run(settings));

    log::info!("app started!");

    handle.join().unwrap()?;

    Ok(())
}
