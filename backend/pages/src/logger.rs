use simplelog::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn init_logger() { 

            CombinedLogger::init(vec![
                TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
                //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
            ])
            .unwrap();

        }
    } else { 
        pub fn init_logger() { 
            CombinedLogger::init(vec![
                SimpleLogger::new(LevelFilter::Info, Config::default()),
                //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
            ])
            .unwrap();
        }
        
    } 
}
