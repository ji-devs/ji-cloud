mod logger;
mod server;
mod templates;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    logger::init_logger()?;

    let settings = core::settings::init().await?;

    server::run(settings).await
}
