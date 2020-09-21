use log::LevelFilter;
use simplelog::{Config, TermLogger, TerminalMode};

pub fn init() -> anyhow::Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;

    Ok(())
}
