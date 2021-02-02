use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};

use flume::Sender;
use futures::{stream::FuturesUnordered, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    StatusCode, Url,
};
use shared::media::MediaLibrary;
use tokio::task;
use uuid::Uuid;

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

pub async fn run(
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
        Err(e) => std::panic::resume_unwind(e.into_panic()),
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

    let client = create_client(&token, &csrf)?;
    let mut tasks = FuturesUnordered::new();

    while let Some(item) = data.pop() {
        while tasks.len() >= max_tasks {
            tasks.next().await;
        }

        let pb = main_pb.clone();
        let mp = Arc::clone(&mp);

        // todo: don't clone all this stuff, ~~just use 1 client~~ + base urls
        let endpoint = endpoint.clone();
        let tx = tx.clone();
        let client = client.clone();

        tasks.push(tokio::spawn(async move {
            {
                let pb = mp.add(ProgressBar::new_spinner());
                pb.set_message(&format!("handling item: {}", item.id));
                refresh_item(pb.clone(), item, client, &endpoint, tx).await;
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
    client: reqwest::Client,
    endpoint: &str,
    tx: Sender<MediaRecord>,
) {
    let id = item.id;
    if let Err(e) = refresh_item_inner(item, client, endpoint, tx).await {
        log::error!("Failed to refresh item `{}`: {}", id, e);
        pb.println(format!("Failed to refresh item: {}", id));
    }
}

fn create_client(token: &str, csrf: &str) -> anyhow::Result<reqwest::Client> {
    let mut default_headers = HeaderMap::new();
    let mut csrf = HeaderValue::from_str(csrf)?;
    csrf.set_sensitive(true);

    default_headers.append("X-CSRF", csrf);

    let mut cookie = HeaderValue::from_str(&format!("X-JWT={}", token))?;
    cookie.set_sensitive(true);

    default_headers.append(header::COOKIE, cookie);

    let client = reqwest::Client::builder()
        .default_headers(default_headers)
        .build()?;

    Ok(client)
}

async fn refresh_item_inner(
    item: MediaItem,
    client: reqwest::Client,
    endpoint: &str,
    tx: Sender<MediaRecord>,
) -> anyhow::Result<()> {
    let endpoint = Url::parse(endpoint)?;

    let path = media_refresh_path(item.library, item.id);

    let url = endpoint.join(&path)?;

    let request = client.post(url);

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
