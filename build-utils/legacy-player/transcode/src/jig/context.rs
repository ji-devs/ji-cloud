use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;

pub struct Context {
    pub token:String,
    pub opts: Opts,
    pub client: Client,
}

impl Context {
    pub fn new(mut opts: Opts) -> Self {
        opts.game_id = Some("17820".to_string());

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

        Self {
            token,
            opts,
            client: Client::new()
        }
    }
}