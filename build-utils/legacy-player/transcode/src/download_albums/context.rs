use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;

pub struct Context {
    pub opts: Opts,
    pub client: Client,
}

impl Context {
    pub fn new(opts: Opts) -> Self {
        log::info!("dry run: {}", opts.dry_run);

        Self {
            opts,
            client: Client::new()
        }
    }
}