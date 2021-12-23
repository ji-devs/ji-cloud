use std::fs::{OpenOptions, File};

use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;
use std::io::{self, BufRead};

pub struct Context {
    pub opts: Opts,
    pub client: Client, 
    pub warnings_log: File,
    pub errors_log: File,
    pub finished_log: File,
    pub skip_finished_list: Vec<String>
}

impl Context {
    pub fn new(opts: Opts) -> Self {

        let skip_finished_list: Vec<String> = {
            match File::open(&opts.finished_log) {
                Ok(file) => {
                    io::BufReader::new(file)
                        .lines()
                        .map(|line| line.unwrap())
                        .collect()
                },
                Err(_) => Vec::new()
            }
        };

        let mut finished_log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&opts.finished_log)
            .unwrap();

        let mut warnings_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&opts.warnings_log).unwrap()
        };

        let mut errors_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&opts.errors_log).unwrap()
        };

        Self {
            opts,
            client: Client::new(),
            warnings_log,
            errors_log,
            finished_log,
            skip_finished_list
        }
    }
}