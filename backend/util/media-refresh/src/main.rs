#![warn(
    clippy::pedantic,
    clippy::multiple_crate_versions,
    clippy::cognitive_complexity,
    clippy::future_not_send,
    clippy::missing_const_for_fn,
    clippy::needless_borrow,
    clippy::redundant_pub_crate,
    clippy::string_lit_as_bytes,
    clippy::use_self,
    clippy::useless_let_if_seq,
    rust_2018_idioms,
    future_incompatible
)]

use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use reqwest::header::{self, HeaderMap, HeaderValue};
use simplelog::Config;

mod download;
mod refresh;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
struct Opts {
    /// Endpoint to access the API from
    #[clap(long, env = "API_ENDPOINT")]
    endpoint: String,

    /// An admin API token
    #[clap(long, env = "API_TOKEN", hide_env_values = true)]
    token: String,

    // todo: just decode this from the above.
    /// CSRF key for the token
    #[clap(long, env = "API_CSRF", hide_env_values = true)]
    csrf: String,

    /// Minimum log level to use.
    #[clap(
            long,
            default_value = "WARN",
            parse(try_from_str),
            possible_values(&["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"]), case_insensitive = true
        )]
    log_level: simplelog::LevelFilter,

    /// Disable progress bar
    #[clap(long = "no-show-progress", parse(from_flag = std::ops::Not::not))]
    show_progress: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
#[clap(rename_all = "kebab-case")]
enum Command {
    // Note: This only supports filtering for images, and refreshing for images
    Download {
        output_file: PathBuf,
    },

    // RestoreBackup {
    //     #[clap(long, short)]
    //     input_file: PathBuf,
    //     #[clap(long, short)]
    //     output_file: PathBuf,

    //     #[clap(long, default_value = "record.csv")]
    //     record_file: PathBuf,
    // },
    Refresh {
        input_file: PathBuf,

        #[clap(long, default_value = "record.csv")]
        record_file: PathBuf,

        /// Controls the maximum amount of tasks that will be processed at once
        #[clap(long, default_value = "5")]
        max_tasks: usize,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().ok();
    let opts = Opts::parse();

    simplelog::WriteLogger::init(
        opts.log_level,
        Config::default(),
        File::create(concat!(env!("CARGO_PKG_NAME"), ".log")).expect("failed to open log file"),
    )
    .expect("failed to create logger");

    match opts.command {
        Command::Download { output_file } => {
            download::run(
                output_file,
                opts.endpoint,
                opts.token,
                opts.csrf,
                opts.show_progress,
            )
            .await
        }
        Command::Refresh {
            input_file,
            record_file,
            max_tasks,
        } => {
            refresh::run(
                input_file,
                record_file,
                max_tasks,
                opts.endpoint,
                opts.token,
                opts.csrf,
                opts.show_progress,
            )
            .await
        } // Command::RestoreBackup { .. } => todo!("restore backup"),
    }
}

fn create_http_client(token: &str, csrf: &str) -> anyhow::Result<reqwest::Client> {
    let mut default_headers = HeaderMap::new();
    let mut csrf = HeaderValue::from_str(csrf)?;
    csrf.set_sensitive(true);

    default_headers.append("X-CSRF", csrf);

    let mut cookie = HeaderValue::from_str(&format!("X-AUTH={}", token))?;
    cookie.set_sensitive(true);

    default_headers.append(header::COOKIE, cookie);

    let client = reqwest::Client::builder()
        .default_headers(default_headers)
        .build()?;

    Ok(client)
}
