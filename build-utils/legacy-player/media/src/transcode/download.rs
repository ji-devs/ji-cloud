
use super::media::{MediaInfo, TranscodeCommand};

use crate::context::Context;
use std::sync::Arc;
use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader};
use std::path::Path;
use uuid::Uuid;
use std::process::Command;
use reqwest::Client; 
use serde::Deserialize;
use serde_json::{Result, value::RawValue};
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();

    let file = File::open(&ctx.opts.media_infos_file_path).unwrap();
    let reader = BufReader::new(file);
    *ctx.medias.lock().unwrap() = serde_json::from_reader(reader).unwrap();

    println!("loaded {} media entries", ctx.medias.lock().unwrap().len());
    _run(ctx.clone(), get_download_futures(ctx.clone()).await).await;
    if !ctx.opts.dry_run {
        let file = File::create(&ctx.opts.transcode_infos_file_path).unwrap();
        serde_json::to_writer_pretty(file, &*ctx.transcodes.lock().unwrap()).unwrap();
    }
    println!("downloaded {} game entries", ctx.stats.media_count());
    ctx.stats.json_set_completed();
}

async fn _run(ctx: Arc<Context>, mut jobs: Vec<impl Future>) {
    let batch_size = ctx.opts.transcode_media_batch_size;

    if batch_size == 0 {
        for job in jobs {
            job.await;
        }
    } else {
        //See: https://users.rust-lang.org/t/awaiting-futuresunordered/49295/5
        //Idea is we try to have a saturated queue of futures

        let mut futures = FuturesUnordered::new();

        while let Some(next_job) = jobs.pop() {
            while futures.len() >= batch_size {
                futures.next().await;
            }
            futures.push(next_job);
        }
        while let Some(_) = futures.next().await {}
    }

}

async fn get_download_futures(ctx:Arc<Context>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    for media in ctx.medias.lock().unwrap().iter() {
        futures.push({
            let ctx = ctx.clone();
            let media = media.clone();
            async move {
                let media_dir = ctx.media_game_dir(&media.game_id);
                let dest_dir = media_dir.join(&media.basepath);
                let dest_path = dest_dir.join(&media.filename);

                if !ctx.opts.transcode_media_skip_download_exists || !dest_path.exists() {
                    log::info!("(game id: {}) downloading {} -> {}", media.game_id, media.url, dest_path.to_str().unwrap());
                    let success = match ctx.client.get(&media.url).send().await {
                        Ok(resp) => {
                            match resp.error_for_status() {
                                Ok(resp) => {
                                    let data = resp
                                        .bytes()
                                        .await
                                        .unwrap();

                                    let mut cursor = std::io::Cursor::new(data);

                                    let mut dir = dest_path.clone();
                                    dir.pop();
                                    std::fs::create_dir_all(&dir);

                                    let mut dest_file = std::fs::File::create(&dest_path).unwrap();
                                    std::io::copy(&mut cursor, &mut dest_file).unwrap();
                                    true
                                },
                                Err(err) => {
                                    false
                                }
                            }
                        },
                        Err(err) => {
                            false
                        }
                    };

                    if !success {
                        writeln!(&ctx.missing_media_downloads_file, "(game id: {}) failure to download {} -> {}", media.game_id, media.url, dest_path.to_str().unwrap());
                    }
                }

                let src_path_str = dest_path.to_str().unwrap();
                if let Some((transcode, filename_dest)) = media.transcode.as_ref() {
                    let dest_file_path = dest_dir.join(filename_dest);
                    if !ctx.opts.transcode_media_skip_convert_exists || !dest_file_path.exists() {
                        let dest_file_path_str = dest_file_path.to_str().unwrap();

                        ctx.transcodes.lock().unwrap().push(TranscodeCommand {
                            src: src_path_str.to_string(),
                            dest: dest_file_path_str.to_string(),
                            cmd: *transcode
                        });
                    }
                }

                ctx.stats.media_increase();
                log::info!("downloaded media #{}", ctx.stats.media_count());

            }
        });
    }
    futures
}