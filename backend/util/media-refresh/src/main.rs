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

use std::{
    fs::File,
    io::{BufReader, BufWriter},
    panic,
    path::PathBuf,
    sync::Arc,
};

use clap::Clap;
use flume::Sender;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use reqwest::{header, StatusCode, Url};
use shared::{error::EmptyError, media::MediaLibrary};
use simplelog::Config;
use tokio::task;
use uuid::Uuid;

#[derive(Clap)]
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
            download(
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
            refresh(
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

#[derive(serde::Serialize, Debug)]
struct MediaRecord {
    id: Uuid,
    resolution: MediaResolution,
    library: MediaLibrary,
}

#[derive(serde::Serialize, Debug)]
enum MediaResolution {
    Success,
    AlreadyUpdated,
    NotFound,
}

#[derive(serde::Deserialize)]
struct MediaItem {
    id: Uuid,
    etag: Option<Box<str>>,
    library: MediaLibrary,
}

#[derive(serde::Deserialize)]
struct MediaItems {
    media: Vec<MediaItem>,
}

async fn refresh(
    input_file: PathBuf,
    record_file: PathBuf,
    max_tasks: usize,
    endpoint: String,
    token: String,
    csrf: String,
    show_progress: bool,
) -> anyhow::Result<()> {
    let reader = File::open(input_file)?;
    let len = reader.metadata()?.len();
    let reader = BufReader::new(reader);
    let res = task::spawn_blocking(move || -> anyhow::Result<MediaItems> {
        let pb = if show_progress {
            ProgressBar::new(len)
        } else {
            ProgressBar::hidden()
        };

        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed}] {wide_bar} {bytes}/{total_bytes} {msg}"),
        );

        pb.set_message("loading input file");
        let reader = pb.wrap_read(reader);

        let records = serde_json::from_reader(reader)?;

        Ok(records)
    })
    .await;

    let data = match res {
        Ok(res) => res?,
        Err(e) => panic::resume_unwind(e.into_panic()),
    };

    let mut data = data.media;

    let (tx, rx) = flume::bounded(max_tasks);

    task::spawn(async move {
        let mut writer = match csv::Writer::from_path(record_file) {
            Ok(writer) => writer,
            Err(e) => {
                log::error!("Failed to initialize record writer: {}", e);
                return;
            }
        };

        while let Ok(data) = rx.recv_async().await {
            if let Err(e) = writer.serialize(data) {
                log::error!("failed to serialize record: {}", e);
            }
        }
    });

    let mp = if show_progress {
        Arc::new(MultiProgress::new())
    } else {
        Arc::new(MultiProgress::with_draw_target(ProgressDrawTarget::hidden()))
    };

    let mp_task = {
        let mp = Arc::downgrade(&mp);
        task::spawn_blocking(move || -> std::io::Result<()> {
            while let Some(mp) = mp.upgrade() {
                mp.join()?;
            }

            Ok(())
        })
    };

    let main_pb = mp.add(ProgressBar::new(data.len() as u64));

    main_pb.set_style(
        ProgressStyle::default_bar().template("[{elapsed}] {wide_bar} {pos}/{len} {msg}"),
    );

    let mut tasks = FuturesUnordered::new();

    while let Some(item) = data.pop() {
        while tasks.len() >= max_tasks {
            tasks.next().await;
        }

        let pb = main_pb.clone();
        let mp = Arc::clone(&mp);

        // todo: don't clone all this stuff, just use 1 client + base urls
        let endpoint = endpoint.clone();
        let token = token.clone();
        let csrf = csrf.clone();
        let tx = tx.clone();
        tasks.push(tokio::spawn(async move {
            {
                let pb = mp.add(ProgressBar::new_spinner());
                pb.set_message(&format!("handling item: {}", item.id));
                refresh_item(pb.clone(), item, &endpoint, &token, &csrf, tx).await;
                pb.finish_and_clear();
            }
            pb.inc(1);
        }))
    }

    drop(mp);

    tasks.for_each(|_| async {}).await;
    main_pb.finish();

    mp_task.await??;

    Ok(())
}

async fn refresh_item(
    pb: ProgressBar,
    item: MediaItem,
    endpoint: &str,
    token: &str,
    csrf: &str,
    tx: Sender<MediaRecord>,
) {
    let id = item.id;
    if let Err(e) = refresh_item_inner(item, endpoint, token, csrf, tx).await {
        log::error!("Failed to refresh item `{}`: {}", id, e);
        pb.println(format!("Failed to refresh item: {}", id));
    }
}

async fn refresh_item_inner(
    item: MediaItem,
    endpoint: &str,
    token: &str,
    csrf: &str,
    tx: Sender<MediaRecord>,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let endpoint = Url::parse(endpoint)?;

    let path = media_refresh_path(item.library, item.id);

    let url = endpoint.join(&path)?;

    let request = client
        .post(url)
        .header("Cookie", &format!("X-JWT={}", token))
        .header("X-CSRF", csrf);

    let request = match item.etag {
        None => request.header(header::IF_NONE_MATCH, "*"),
        Some(etag) => request.header(header::IF_MATCH, format!("\"{}\"", etag)),
    };

    let response = request.send().await?;

    let resolution = match response.status() {
        StatusCode::NO_CONTENT => Some(MediaResolution::Success),
        StatusCode::PRECONDITION_FAILED => Some(MediaResolution::AlreadyUpdated),
        StatusCode::NOT_FOUND => Some(MediaResolution::NotFound),
        _ => None,
    };

    if let Some(resolution) = resolution {
        tx.send_async(MediaRecord {
            id: item.id,
            resolution,
            library: item.library,
        })
        .await?;
    }

    if response.status().is_server_error() {
        log::warn!("Failed to refresh item due to server error: {}", item.id);
    }

    if response.status() == StatusCode::UNAUTHORIZED || response.status() == StatusCode::FORBIDDEN {
        log::warn!("Failed to refresh item due to bad auth: {}", item.id);
    }

    Ok(())
}

fn media_refresh_path(library: MediaLibrary, id: Uuid) -> String {
    let library = match library {
        MediaLibrary::Web => "Web",
        MediaLibrary::User => "User",
        MediaLibrary::Global => "Global",
    };

    format!("v0/admin/media/refresh/{}/image/{}", library, id)
}

async fn download(
    output_file: PathBuf,
    endpoint: String,
    token: String,
    csrf: String,
    show_progress: bool,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/v0/admin/media", endpoint))
        .header("Cookie", &format!("X-JWT={}", token))
        .header("X-CSRF", csrf)
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_) => {}
        Err(_) => {
            let error_json = response
                .json::<shared::error::ApiError<EmptyError>>()
                .await?;

            anyhow::bail!(
                "request failed ({}): {}",
                error_json.code,
                error_json.message
            )
        }
    }

    let data = response
        .json::<shared::domain::admin::AdminListMediaResponse>()
        .await?;

    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
        let writer = File::create(&output_file)?;
        let writer = BufWriter::new(writer);

        let pb = if show_progress {
            ProgressBar::new_spinner()
        } else {
            ProgressBar::hidden()
        };

        pb.set_message(&format!("writing output file: {:?}", output_file));
        serde_json::to_writer(pb.wrap_write(writer), &data)?;
        pb.finish();

        Ok(())
    })
    .await
    .unwrap()?;

    Ok(())
}
