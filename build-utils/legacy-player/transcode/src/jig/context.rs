use std::fs::{OpenOptions, File};
use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;

pub struct Context {
    pub token:String,
    pub opts: Opts,
    pub client: Client,
    pub info_log: File,
}

impl Context {
    pub fn new(mut opts: Opts) -> Self {
        let token = {
            if !opts.token.is_empty() {
                log::info!("TOKEN: {}", opts.token);
                opts.token.clone()
            } else {
                log::info!("no token set in opts, using env");
                std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
            }
        };

        let mut info_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&opts.info_log).unwrap()
        };

        Self {
            token,
            opts,
            client: Client::new(),
            info_log
        }
    }
}