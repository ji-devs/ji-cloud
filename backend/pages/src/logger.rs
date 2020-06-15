use crate::settings::SETTINGS;
use simplelog::*;
pub fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
    ])
    .unwrap();
    //log::info!("{:?}", &*SETTINGS);
}
