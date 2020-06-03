use crate::settings::SETTINGS;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "simplelog")] {
        use simplelog::*;

        pub fn init_logger() {
            CombinedLogger::init(vec![
                TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
                //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
            ])
            .unwrap();
            //log::info!("{:?}", &*SETTINGS);
        }

    } else {
        pub fn init_logger() {

        }
    }
}