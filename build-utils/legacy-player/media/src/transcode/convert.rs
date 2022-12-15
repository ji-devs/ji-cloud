
use super::media::{MediaInfo, TranscodeCommand};

use crate::context::Context;
use crate::transcode::media::MediaTranscode;
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
use std::thread;
use glob::{glob_with, MatchOptions};

fn main() -> Result<()> {

    Ok(())
}
pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();

    if ctx.opts.transcode_media_parse_from_json_file {
        let file = File::open(&ctx.opts.transcode_infos_file_path).unwrap();
        let reader = BufReader::new(file);
        *ctx.transcodes.lock().unwrap() = serde_json::from_reader(reader).unwrap();
    } else {
        set_transcodes_from_disk(ctx.clone());
    }

    println!("loaded {} transcode entries", ctx.transcodes.lock().unwrap().len());
    do_convert(ctx.clone());
    println!("finished all {} transcode entries", ctx.transcodes.lock().unwrap().len());
}

fn set_transcodes_from_disk(ctx:Arc<Context>) {
    let mut output = Vec::new();
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for ext in vec!["mov", "avi", "aiff", "ac3"] {
        let path = ctx.opts.games_dir.join(format!("**/*.{ext}")).display().to_string();
        for entry in glob_with(&path, options).unwrap() {
            let src = entry.unwrap();
            let dest_name = format!("{}.mp4", Path::new(&src).file_stem().unwrap().to_str().unwrap().to_string());
            let dest = format!("{}/{}", Path::new(&src).parent().unwrap().display(), dest_name);

            let cmd = TranscodeCommand {
                src: src.display().to_string(), 
                dest,
                cmd: MediaTranscode::Video
            };

            println!("{:#?}", cmd);
            output.push(cmd);

        }
    }

    for ext in vec!["aac", "wav", "aiff", "ac3"] {
        let path = ctx.opts.games_dir.join(format!("**/*.{ext}")).display().to_string();
        for entry in glob_with(&path, options).unwrap() {
            let src = entry.unwrap();
            let dest_name = format!("{}.mp3", Path::new(&src).file_stem().unwrap().to_str().unwrap().to_string());
            let dest = format!("{}/{}", Path::new(&src).parent().unwrap().display(), dest_name);

            let cmd = TranscodeCommand {
                src: src.display().to_string(), 
                dest,
                cmd: MediaTranscode::Audio
            };

            println!("{:#?}", cmd);
            output.push(cmd);
        }
    }

    *ctx.transcodes.lock().unwrap() = output;
}

fn do_convert(ctx:Arc<Context>) {
    let mut handles = Vec::new();
    let transcode_commands:Arc<Vec<TranscodeCommand>> = Arc::new(ctx.transcodes.lock().unwrap().clone());
    let n_threads = ctx.opts.transcode_media_convert_thread_size;
    if n_threads > 0 {
        for i in 0..n_threads {
            handles.push({
                let transcode_commands = transcode_commands.clone();
                let ctx = ctx.clone();
                thread::spawn(move || {
                    if let Some(chunk) = transcode_commands.chunks(n_threads).nth(i) {
                        for item in chunk.into_iter() {
                            do_transcode(ctx.clone(), item);
                        }
                    }
                })
            });
        }
        for h in handles {
            h.join().unwrap();
        }
    } else {
        for item in transcode_commands.iter() {
            do_transcode(ctx.clone(), item);
        }
    }
}

fn do_transcode(ctx: Arc<Context>, transcode_command: &TranscodeCommand) {
    let TranscodeCommand { src, dest, cmd } = transcode_command;

    if !Path::new(src).exists() {

        writeln!(&ctx.missing_media_downloads_file, "{} does not exist for transcoding", src);
        return;
    }
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
