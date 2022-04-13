#![allow(warnings)]

use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use ::migrate::{
    src_manifest::*,
};
use std::process::Command;
use reqwest::Client; 
use serde::Deserialize;
use serde_json::{Result, value::RawValue};
use std::sync::Arc;
use std::collections::HashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use futures::lock::Mutex;
use crate::context::Context;
use crate::process::game_urls::GameJsonUrl;
use std::thread;

type TranscodeCommands = Arc<Mutex<Vec<TranscodeCommand>>>;

#[derive(Debug, Clone)]
struct TranscodeCommand {
    pub src: String, 
    pub dest: String,
    pub cmd: MediaTranscode
}

pub async fn run(ctx:Arc<Context>, medias: Arc<Mutex<Vec<Media>>>) {
    let transcode_commands = Arc::new(Mutex::new(Vec::new()));

    if ctx.opts.transcode_download_media {
        _run(ctx.opts.transcode_media_download_batch_size, get_download_futures(ctx.clone(), medias.lock().await.clone(), transcode_commands.clone()).await).await;
    }

    if ctx.opts.transcode_convert_media {
        let mut handles = Vec::new();
        let transcode_commands:Arc<Vec<TranscodeCommand>> = Arc::new(transcode_commands.lock().await.clone());
        let n_threads = ctx.opts.transcode_media_convert_thread_size;
        for i in 0..n_threads {
            handles.push({
                let transcode_commands = transcode_commands.clone();
                thread::spawn(move || {
                    if let Some(chunk) = transcode_commands.chunks(n_threads).nth(i) {
                        for item in chunk.into_iter() {
                            do_transcode(item);
                        }
                    }
                })
            });
        }


        for h in handles {
            h.join().unwrap();
        }
    }

}

async fn _run(batch_size: usize, mut jobs: Vec<impl Future>) {

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

async fn get_download_futures(ctx:Arc<Context>, medias: Vec<Media>, transcode_commands: TranscodeCommands) -> Vec<impl Future> {

    let mut futures = Vec::new();

    for media in medias.into_iter() {
        futures.push({
            let ctx = ctx.clone();
            let transcode_commands = transcode_commands.clone();
            async move {
                let games_dir = ctx.games_dir.join(&media.game_id);
                let media_dir = games_dir.join("media");
                let dest_dir = media_dir.join(&media.basepath);
                let dest_path = dest_dir.join(&media.filename);

                if !ctx.opts.transcode_media_skip_download_exists || !dest_path.exists() {
                    log::info!("downloading {} -> {}", media.url, dest_path.to_str().unwrap());
                    match ctx.client.get(&media.url).send().await {
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

                                    let src_path_str = dest_path.to_str().unwrap();
                                    if let Some((transcode, filename_dest)) = media.transcode.as_ref() {
                                        let dest_file_path = dest_dir.join(filename_dest);
                                        if !ctx.opts.transcode_media_skip_convert_exists || !dest_file_path.exists() {
                                            let dest_file_path_str = dest_file_path.to_str().unwrap();

                                            transcode_commands.lock().await.push(TranscodeCommand {
                                                src: src_path_str.to_string(),
                                                dest: dest_file_path_str.to_string(),
                                                cmd: *transcode
                                            });
                                        }
                                    }

                                },
                                Err(err) => {
                                    if ctx.opts.transcode_allow_empty_media {
                                        log::warn!("couldn't download {}", media.url)
                                    } else {
                                        panic!("couldn't download {}", media.url)
                                    }
                                }
                            }
                        },
                        Err(err) => {
                            panic!("unable to download: {}", media.url);
                        }
                    }
                }
            }
        });
    }
    futures
}

fn do_transcode(transcode_command: &TranscodeCommand) {
    let TranscodeCommand { src, dest, cmd } = transcode_command;

    log::info!("converting {} to {}...", src, dest);
    match cmd{
        MediaTranscode::Audio => {

            Command::new("ffmpeg")
                .arg("-i")
                .arg(src)
                .arg("-acodec")
                .arg("libmp3lame")
                .arg(dest)
                .output()
                .expect("failed to execute process");
        },
        MediaTranscode::Video => {
            Command::new("ffmpeg")
                .arg("-i")
                .arg(src)
                .arg("-vcodec")
                .arg("libx264")
                .arg("-acodec")
                .arg("aac")
                .arg("-pix_fmt")
                .arg("yuv420p")
                .arg("-crf")
                .arg("20")
                .arg("-maxrate")
                .arg("1M")
                .arg(dest)
                .output()
                .expect("failed to execute process");
        },

        // ffmpeg -an -i input.mov -vcodec libx264 -pix_fmt yuv420p -profile:v baseline -level 3 output.mp4

        // no longer transcoding animation
        // MediaTranscode::Animation => {

        //     let dest_file_path = dest_dir.join(&format!("{}.webm", media.file_stem()));

        //     if !opts.skip_transcode_exists || !dest_file_path.exists() {
        //         let dest_file_path = dest_file_path.to_str().unwrap();
        //         log::info!("converting animation {} to {}...", media.file_stem(), dest_file_path);

        //         // ffmpeg -i 2_anim.gif -c vp9 -b:v 0 -crf 26 -pix_fmt yuva420p test001.webm
        //         Command::new("ffmpeg")
        //             .arg("-i")
        //             .arg(src_file_path_str)
        //             .arg("-c")
        //             .arg("vp9")
        //             .arg("-b:v")
        //             .arg("0")
        //             .arg("-crf")
        //             .arg("26")
        //             .arg("-pix_fmt")
        //             .arg("-yuva420p")
        //             .arg(dest_file_path)
        //             .output()
        //             .expect("failed to execute process");
        //     }

            // works for prores but huge file:
            // ffmpeg -i 2_anim.gif -c prores -pix_fmt yuva444p10le -profile:v 4 -vf pad="width=ceil(iw/2)*2:height=ceil(ih/2)*2" test001.mov
            
            // unfortunately, yuva420p isn't supported here
            // let dest_file_path = dest_dir.join(&format!("{}.mp4", media.file_stem()));

            // if !opts.skip_transcode_exists || !dest_file_path.exists() {
            //     let dest_file_path = dest_file_path.to_str().unwrap();
            //     log::info!("converting animation {} to {}...", media.file_stem(), dest_file_path);
            //      ffmpeg -i 2_anim.gif -c libx265 -b:v 0 -crf 26 -pix_fmt yuva420p test006.mp4
            //     Command::new("ffmpeg")
            //         .arg("-i")
            //         .arg(src_file_path_str)
            //         .arg("-c")
            //         .arg("libx265")
            //         .arg("-vtag")
            //         .arg("hvc1")
            //         .arg("-preset")
            //         .arg("slow")
            //         .arg("-b:v")
            //         .arg("0")
            //         .arg("-crf")
            //         .arg("26")
            //         .arg("-pix_fmt")
            //         .arg("-yuva420p")
            //         .arg(dest_file_path)
            //         .output()
            //         .expect("failed to execute process");
            // }
        // }
    }
}
