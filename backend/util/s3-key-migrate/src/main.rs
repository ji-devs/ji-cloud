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

use std::{fs::File, time::Instant};

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rusoto_core::{
    credential::{AwsCredentials, StaticProvider},
    HttpClient, Region,
};
use rusoto_s3::{
    CopyObjectRequest, DeleteObjectRequest, ListObjectsOutput, ListObjectsRequest, S3Client, S3,
};
use shared::media::{FileKind, MediaLibrary, PngImageFile};
use simplelog::Config;
use uuid::Uuid;

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
struct Opts {
    /// The endpoint to access S3 from
    #[clap(long, env = "S3_ENDPOINT")]
    endpoint: String,

    /// The s3 bucket to use
    #[clap(long, env = "S3_BUCKET")]
    bucket: String,

    #[clap(long, env = "S3_ACCESS_KEY", hide_env_values = true)]
    access_key_id: String,

    #[clap(long, env = "S3_ACCESS_SECRET", hide_env_values = true)]
    access_secret: String,

    /// Just list through the objects, calculating their new keys, but not updating them
    #[clap(long)]
    dry_run: bool,

    /// Controls the maximum amount of objects that will be processed at once
    #[clap(long, default_value = "50")]
    max_tasks: usize,

    /// The minimum log level to use.
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

    let region = Region::Custom {
        name: "auto".to_owned(),
        endpoint: opts.endpoint,
    };

    let creds = AwsCredentials::new(opts.access_key_id, opts.access_secret, None, None);

    let credentials_provider = StaticProvider::from(creds);

    let s3 = rusoto_s3::S3Client::new_with(HttpClient::new()?, credentials_provider, region);

    let Opts {
        bucket,
        dry_run,
        max_tasks,
        show_progress,
        ..
    } = opts;

    let mut last_key = None;

    let mut total = 0;

    let mut list_ops: u64 = 0;

    loop {
        let start = Instant::now();

        list_ops += 1;

        let resp: ListObjectsOutput = s3
            .list_objects(ListObjectsRequest {
                bucket: bucket.clone(),
                prefix: Some("image".to_owned()),
                marker: last_key,
                ..ListObjectsRequest::default()
            })
            .await?;

        let after_list_objects = Instant::now();
        log::trace!(target: "s3_key_migrate::timing", "s3 `list_objects` took: {:?}", after_list_objects - start);

        let objects = resp.contents.as_ref().map_or(0, Vec::len);

        let progress = if show_progress {
            let progress = ProgressBar::new(objects as u64);
            progress.set_style(
                ProgressStyle::default_bar().template("{prefix} {wide_bar} {pos}/{len}"),
            );
            progress.set_prefix(&format!("{}/?", list_ops));
            Some(progress)
        } else {
            None
        };

        log::info!("processing: {} objects", objects);

        last_key = None;

        let iter = resp
            .contents
            .into_iter()
            .flatten()
            .filter_map(|obj| obj.key)
            .inspect(|key| last_key = Some(key.clone()));

        futures::stream::iter(iter)
            .for_each_concurrent(max_tasks, |key| {
                let s3 = s3.clone();
                let bucket = bucket.clone();
                let progress = progress.clone();
                async move {
                    if let Err(e) = handle_item(key, &s3, &bucket, dry_run).await {
                        log::warn!("error processing object: {:?}", e);
                    }

                    progress.map(|it| it.inc(1));
                }
            })
            .await;

        progress.as_ref().map(ProgressBar::finish);

        log::trace!(target: "s3_key_migrate::timing", "process objects took {:?}", after_list_objects.elapsed());

        total += objects;

        // Not truuncated = no more items.
        if !resp.is_truncated.unwrap_or(false) {
            break;
        }
    }

    log::info!("finished processing: {} objects", total);

    Ok(())
}

async fn handle_item(
    old_key: String,
    s3: &S3Client,
    bucket: &str,
    dry_run: bool,
) -> anyhow::Result<()> {
    log::debug!("processing object: {:?}", old_key);

    let (library, id, file_kind) = parse_old_image_key(&old_key)?;

    let new_key = shared::media::media_key(library, id, file_kind);

    // hack: we should just skip operations instead of doing an early return.
    if dry_run {
        log::debug!("rename {:?} -> {:?}", old_key, new_key);
        log::debug!("delete: {:?}", old_key);
        return Ok(());
    }

    log::debug!("rename {:?} -> {:?}", old_key, new_key);

    s3.copy_object(CopyObjectRequest {
        bucket: bucket.to_owned(),
        copy_source: format!("{}/{}", &bucket, old_key),
        content_type: Some(file_kind.content_type().to_owned()),
        metadata_directive: Some("REPLACE".to_owned()),
        key: new_key,
        ..CopyObjectRequest::default()
    })
    .await?;

    log::debug!("delete: {:?}", old_key);

    s3.delete_object(DeleteObjectRequest {
        bucket: bucket.to_owned(),
        key: old_key,
        ..DeleteObjectRequest::default()
    })
    .await?;

    Ok(())
}

fn parse_old_image_key(key: &str) -> anyhow::Result<(MediaLibrary, Uuid, FileKind)> {
    static IMAGE_LIBRARY: phf::Map<&'static str, MediaLibrary> = phf::phf_map! {
        "image" => MediaLibrary::Global,
        "image-user" => MediaLibrary::User,
        "image-web" => MediaLibrary::Web,
    };

    static IMAGE_FILE: phf::Map<&'static str, PngImageFile> = phf::phf_map! {
        "raw" => PngImageFile::Original,
        "original" => PngImageFile::Original,
        "resized" => PngImageFile::Resized,
        "thumbnail" => PngImageFile::Thumbnail,
    };

    let parts = key.split('/').collect::<Vec<_>>();

    anyhow::ensure!(
        parts.len() == 2 || parts.len() == 3,
        "too few/many parts: {:?} ({})",
        key,
        parts.len()
    );

    let library = IMAGE_LIBRARY.get(parts[0]);

    let library = match library {
        Some(library) => *library,
        None => anyhow::bail!("Couldn't detect library prefix: {}", parts[0]),
    };

    let file_kind = match parts.len() {
        2 => PngImageFile::Original,
        3 => match IMAGE_FILE.get(parts[1]) {
            Some(it) => *it,
            None => anyhow::bail!("Couldn't detect image file kind: {}", parts[1]),
        },
        _ => unreachable!(),
    };

    let file_kind = FileKind::ImagePng(file_kind);

    let id = parts.last().expect("Parts can't be empty by this point");
    let id = Uuid::parse_str(id)?;

    Ok((library, id, file_kind))
}
