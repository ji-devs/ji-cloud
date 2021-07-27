use super::report::Report;
use super::options::Opts;
use dotenv::dotenv;
use tokio::sync::RwLock;
use simplelog::*;
use structopt::StructOpt;

pub struct Context {
    pub token:String,
    pub opts: Opts,
    pub report: RwLock<Report>
}

impl Context {
    pub fn new() -> Self {
        let _ = dotenv().ok();
        
        let mut opts = Opts::from_args();

        init_logger(&opts);

        log::info!("dry run: {}", opts.dry_run);

        let token = {
            if !opts.token.is_empty() {
                log::info!("TOKEN: {}", opts.token);
                opts.token.clone()
            } else {
                log::info!("no token set in opts, using env");
                std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
            }
        };

        let report = RwLock::new(Report::new());

        Self {
            token,
            opts,
            report
        }
    }
}

fn init_logger(opts:&Opts) {
    if opts.verbose {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    }
}
