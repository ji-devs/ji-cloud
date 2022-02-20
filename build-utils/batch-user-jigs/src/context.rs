use super::options::Opts;
use dotenv::dotenv;
use shared::domain::jig::JigId;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;
use std::{io::BufRead, collections::HashMap};
use std::sync::Mutex;
use shared::{
    config::RemoteTarget,
    api::{ApiEndpoint, endpoints},
    domain::jig::JigBrowseResponse
};
use legacy_transcode::jig_log::JigInfoLogLine;

pub struct Context {
    pub token:String,
    pub opts: Opts,
    pub client: Client,
    // jig id to game id
    pub legacy_lookup: HashMap<String, String>
}

impl Context {
    pub async fn new(mut opts: Opts) -> Self {
        let token = {
            if !opts.token.is_empty() {
                log::info!("TOKEN: {}", opts.token);
                opts.token.clone()
            } else {
                log::info!("no token set in opts, using env");
                std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
            }
        };

        let client = Client::new();

        let legacy_lookup = if opts.update_background_music {
            log::info!("loading legacy lookup");
            let url = match opts.get_remote_target() {
                RemoteTarget::Release => "https://storage.googleapis.com/ji-cloud-legacy-eu-001/17000-report/create.txt",
                _ => "https://storage.googleapis.com/ji-cloud-legacy-eu-001/17000-report/create-sandbox.txt",
            };

            let res = client 
                .get(url)
                .send()
                .await
                .unwrap();

            res
                .text()
                .await
                .unwrap()
                .lines()
                .map(|line| {
                    let line = JigInfoLogLine::read_line(&line);
                    (line.jig_id, line.game_id)
                })
                .collect()
        } else {
            HashMap::new()
        };

        Self {
            token,
            opts,
            client,
            legacy_lookup
        }
    }
}
