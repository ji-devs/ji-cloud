use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};

use flume::Sender;
use futures::{stream::FuturesUnordered, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use reqwest::{header, StatusCode, Url};
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
    #[serde(alias = "file_etag")]
    etag: Option<Box<str>>,
    library: MediaLibrary,
}

#[derive(serde::Deserialize)]
struct MediaItems {
    media: Vec<MediaItem>,
}

async fn load(input_file: PathBuf, show_progress: bool) -> anyhow::Result<Vec<MediaItem>> {
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

    Ok(data.media)
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
    let endpoint = Arc::new(Url::parse(&endpoint)?);

    let mut data = load(input_file, show_progress).await?;
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

    let client = crate::create_http_client(&token, &csrf)?;
    let mut tasks = FuturesUnordered::new();

    while let Some(item) = data.pop() {
        while tasks.len() >= max_tasks {
            tasks.next().await;
        }

        let pb = main_pb.clone();
        let mp = Arc::clone(&mp);

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

    while let Some(_) = tasks.next().await {
        // do nothing
    }

    main_pb.finish();

    mp_task.await??;

    Ok(())
}

async fn refresh_item(
    pb: ProgressBar,
    item: MediaItem,
    client: reqwest::Client,
    endpoint: &Url,
    tx: Sender<MediaRecord>,
) {
    let id = item.id;
    if let Err(e) = refresh_item_inner(item, client, endpoint, tx).await {
        log::error!("Failed to refresh item `{}`: {}", id, e);
        pb.println(format!("Failed to refresh item: {}", id));
    }
}

async fn refresh_item_inner(
    item: MediaItem,
    client: reqwest::Client,
    endpoint: &Url,
    tx: Sender<MediaRecord>,
) -> anyhow::Result<()> {
    let path = media_refresh_path(item.library, item.id);

    let request = client.post(endpoint.join(&path)?);

    let request = match item.etag {
        None => request.header(header::IF_NONE_MATCH, "*"),
        Some(etag) => request.header(header::IF_MATCH, &*etag),
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
