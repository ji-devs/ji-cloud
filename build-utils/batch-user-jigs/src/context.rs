use super::options::Opts;
use dotenv::dotenv;
use legacy_transcode::jig_log::JigInfoLogLine;
use reqwest::Client;
use shared::domain::jig::JigId;
use shared::{
    api::{endpoints, ApiEndpoint},
    config::RemoteTarget,
    domain::jig::JigBrowseResponse,
};
use simplelog::*;
use std::sync::Mutex;
use std::{collections::HashMap, io::BufRead};
use structopt::StructOpt;

pub struct Context {
    pub token: String,
    pub opts: Opts,
    pub client: Client,
    // jig id to game id
    pub legacy_lookup: HashMap<String, String>,
}

impl Context {
    pub async fn new(mut opts: Opts) -> Self {
        let token = {
            if !opts.token.is_empty() {
                log::info!("TOKEN: {}", opts.token);
                opts.token.clone()
            } else {
                log::info!("no token set in opts, using env");
                std::env::var("LOCAL_API_AUTH_OVERRIDE")
                    .expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
            }
        };

        let client = Client::new();

        let legacy_lookup = if opts.update_background_music {
            log::info!("loading legacy lookup");
            let url = match opts.get_remote_target() {
                RemoteTarget::Release => "https://storage.googleapis.com/ji-cloud-legacy-eu-001/17000-report/create.txt",
                _ => "https://storage.googleapis.com/ji-cloud-legacy-eu-001/17000-report/create-sandbox.txt",
            };

            let res = client.get(url).send().await.unwrap();

            res.text()
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
            legacy_lookup,
        }
    }
}
