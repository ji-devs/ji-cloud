use std::fs::{OpenOptions, File};
use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;
use transcode::jig_log::JigInfoLogLine;
use std::io::BufRead;
use scan_fmt::scan_fmt;
use std::sync::Mutex;

pub struct Context {
    pub token:String,
    pub opts: Opts,
    pub client: Client,
    pub info_lines: Mutex<Option<Vec<JigInfoLogLine>>>
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

        let res = reqwest::Client::new()
            .get(&opts.info_file_url)
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("Failed to get info file");
        }

        let mut info_lines:Vec<JigInfoLogLine> = res
            .text()
            .await
            .unwrap()
            .lines()
            .map(|line| {
                JigInfoLogLine::read_line(&line)
            })
            .collect();

        if let Some(game_id) = opts.game_id.as_ref() {
            info_lines.retain(|line| &line.game_id == game_id);
        }
        

        Self {
            token,
            opts,
            client: Client::new(),
            info_lines: Mutex::new(Some(info_lines))
        }
    }
}