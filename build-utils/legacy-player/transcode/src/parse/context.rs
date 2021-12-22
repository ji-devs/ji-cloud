use std::fs::{OpenOptions, File};

use super::options::Opts;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

pub struct Context {
    pub opts: Opts,
    pub client: ClientWithMiddleware, 
    pub warnings_log: File,
    pub errors_log: File,
}

impl Context {
    pub fn new(opts: Opts) -> Self {

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

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(10);
        let client = ClientBuilder::new(reqwest::Client::new())
            // Retry failed requests.
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Self {
            opts,
            client,
            warnings_log,
            errors_log,
        }
    }
}