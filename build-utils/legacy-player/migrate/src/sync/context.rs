use std::{fs::{OpenOptions, File}, path::PathBuf, collections::HashMap};

use crate::stats::Stats;
use super::options::Opts;
use dotenv::dotenv;
use shared::domain::jig::{JigData, JigId};
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;
use std::io::{self, BufRead};
use futures::lock::Mutex;

pub struct Context {
    pub opts: Opts,
    pub client: Client, 
    pub warnings_log: File,
    pub errors_log: File,
    pub finished_log: File,
    pub albums_dir: PathBuf,
    pub games_dir: PathBuf,
    pub stats: Mutex<Stats>,
}


impl Context {
    pub fn new(opts: Opts) -> Self {
        let logs_dir = opts.output_dir.join("logs");
        let albums_dir = opts.output_dir.join("albums");
        let games_dir = opts.output_dir.join("games");

        if !opts.dry_run {
            std::fs::create_dir_all(&logs_dir);
            std::fs::create_dir_all(&albums_dir);
            std::fs::create_dir_all(&games_dir);
        }

        let finished_path = logs_dir.join("finished.txt");
        let warnings_path = logs_dir.join("warnings.txt");
        let errors_path = logs_dir.join("errors.txt");
        let albums_path = logs_dir.join("albums.txt");
        let stats_path = opts.output_dir.join("stats.txt");

        let mut finished_log = 
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&finished_path)
                .unwrap();

        let mut warnings_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&warnings_path).unwrap()
        };

        let mut errors_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&errors_path).unwrap()
        };

        let mut albums_log = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            if opts.clear_log_files {
                file.truncate(true)
            } else {
                file.append(true)
            }.open(&albums_path).unwrap()
        };

        let stats = Stats::load(stats_path, opts.clear_stats);

        Self {
            opts,
            client: Client::new(),
            warnings_log,
            errors_log,
            finished_log,
            albums_dir,
            games_dir,
            stats: Mutex::new(stats),
        }
    }
}

