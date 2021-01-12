pub fn init() -> anyhow::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,sqlx::query=warn"),
    );

    Ok(())
}
