use std::{fs::{OpenOptions, File}, hash::Hash};
use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;
use transcode::jig_log::JigInfoLogLine;
use std::io::BufRead;
pub use scan_fmt::scan_fmt;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{image::*, meta::*},
    error::*,
    media::MediaLibrary,
};
use std::collections::HashSet;

pub struct Context {
    pub token:String,
    pub opts: Opts,
    pub client: Client,
    pub info_log: File,
    pub skip_game_ids: Vec<String>,
    pub affiliations: Vec<AffiliationId>,
    pub age_ranges: Vec<AgeRangeId>,
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

        let url = format!("{}{}", 
            opts.get_remote_target().api_url(), 
            endpoints::meta::Get::PATH
        );
        let res = reqwest::Client::new()
            .get(url)
            .header("AUTHORIZATION", &format!("Bearer {}", &token))
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("Failed to get jig data");
        }

        let MetadataResponse { affiliations, age_ranges, .. } = res.json().await.unwrap();

        let affiliations = affiliations
            .iter()
            .map(|x| x.id)
            .collect();

        let age_ranges = age_ranges
            .iter()
            .map(|x| x.id)
            .collect();

        let mut info_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&opts.info_log).unwrap()
        };

        let mut skip_game_ids = HashSet::new();

        if opts.skip_info_log {
            if let Ok(mut file) = OpenOptions::new()
                .read(true)
                .open(&opts.skip_info_log_file)
                {

                    for line in std::io::BufReader::new(file).lines() {
                        let line = JigInfoLogLine::read_line(&line.unwrap());
                        log::info!("skipping {} due to skip_info", line.game_id);
                        skip_game_ids.insert(line.game_id);
                    }
            }
        } 
        if opts.skip_errors_log {

            let res = reqwest::Client::new()
                .get(&opts.skip_errors_log_url)
                .send()
                .await
                .unwrap();

            if !res.status().is_success() {
                log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                panic!("Failed to get errors log for skipping");
            }


            for line in res.text().await.unwrap().lines() {
                let game_id = scan_fmt!(&line, "{} ", String).unwrap();
                log::info!("skipping {} due to error", game_id);
                skip_game_ids.insert(game_id);
            }
        }

        Self {
            token,
            opts,
            client: Client::new(),
            info_log,
            skip_game_ids: skip_game_ids.into_iter().collect(),
            affiliations,
            age_ranges
        }
    }
}