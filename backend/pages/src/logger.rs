use simplelog::*;

#[cfg(feature = "local")]
pub fn init_logger() -> anyhow::Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;

    Ok(())
}

#[cfg(not(feature = "local"))]
pub fn init_logger() -> anyhow::Result<()> {
    SimpleLogger::init(LevelFilter::Info, Config::default())?;

    Ok(())
}
