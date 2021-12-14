#![allow(warnings)]

use std::{future::Future, path::PathBuf};
use std::sync::Arc;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use ::transcode::{
    src_manifest::*,
};
use serde::Deserialize;
use shared::domain::jig::module::ModuleBody;
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::process::Command;
use reqwest::Client; 
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{Duration};

mod context;
use context::*;
mod options;
use options::*;
mod convert;

// url
// http://localhost:4104/module/legacy/play/debug?slide_index=0&game_id=ID

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    let ctx = Arc::new(Context::new(opts));


    let batch_size = *&ctx.opts.batch_size;
    let mut jobs = get_futures(ctx.clone()).await;

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

    log::info!("done!");
}

async fn get_futures(ctx:Arc<Context>) -> Vec<impl Future> {
    let urls = game_json_urls(&ctx.opts).await;
    log::info!("parsing {} urls", urls.len());
    urls
        .into_iter()
        .map(|url| {
            transcode_game(ctx.clone(), url)
        })
        .collect()
}
async fn game_json_urls(opts:&Opts) -> Vec<String> {

    #[derive(Deserialize)]
    struct Data {
        album: Album,
    }
    #[derive(Deserialize)]
    struct Album {
        fields: AlbumFields,
    }
    #[derive(Deserialize)]
    struct AlbumFields {
        structure: String,
    }

    match opts.game_json_url.as_ref() {
        Some(url) => {
            vec![url.to_string()]
        },
        None => {
            if opts.game_json_from_albums {
                let paths = fs::read_dir(&opts.game_json_albums_dir).unwrap();
                let mut urls = Vec::new();

                for path in paths {
                    let file = File::open(path.unwrap().path()).unwrap();
                    let reader = BufReader::new(file);
                    let data:Data = serde_json::from_reader(reader).unwrap();

                    //log::info!("{}", data.album.fields.structure);
                    urls.push(data.album.fields.structure);
                }
                urls
            } else {
                let list:&[&str] = &[
                        // // David Test 002 (houdini) - 17736
                        // https://jitap.net/activities/gemy/play/david-test-002
                        "https://d24o39yp3ttic8.cloudfront.net/5D00A147-73B7-43FF-A215-A38CB84CEBCD/game.json",

                        // // // Corinne Houdini - 17762
                        // https://jitap.net/activities/geno/play/houdini-states
                        "https://d24o39yp3ttic8.cloudfront.net/58C85551-79A5-4E36-9794-3D3D8D3E0D31/game.json",

                        // // // Soundboard states - 17765
                        // // // https://jitap.net/activities/genr/play/soundboard-states 
                        "https://d24o39yp3ttic8.cloudfront.net/9F5AD80D-7D86-4AB9-AB11-C942B162923E/game.json",

                        // // // say something options - 17746
                        // // // https://jitap.net/activities/gen8/play/say-something-options
                        "https://d24o39yp3ttic8.cloudfront.net/86DCDC1D-64CB-4198-A866-257E213F0405/game.json",

                        // // // video for legacy - 17771 
                        // // // https://jitap.net/activities/genx/play/ 
                        "https://d24o39yp3ttic8.cloudfront.net/64498594-B340-4B5C-87E0-615C6ACC7676/game.json",

                        // // // ask a question legacy player - 17792
                        // // // https://jitap.net/activities/geoi/play/testing-ask-a-question-legacy-player
                        "https://d24o39yp3ttic8.cloudfront.net/236F4AC1-9B06-49EA-B580-4AE806B0A337/game.json",

                        // // puzzle - 17822
                        // // https://jitap.net/activities/gepc/play/test-puzzles-for-legacy-player
                        "https://d24o39yp3ttic8.cloudfront.net/D9BB6E6A-03FE-4B39-A3CD-289059E118E9/game.json",

                        // // talk or type - 17820
                        // // https://jitap.net/activities/gepa/play/test-talk-or-type-for-legacy-player
                        "https://d24o39yp3ttic8.cloudfront.net/2B7A33C0-BA81-4661-9202-4C0463AEC606/game.json"
                ];
                
                list.iter().map(|x| String::from(*x)).collect()
            }
        }
    }
    
}

async fn transcode_game(ctx: Arc<Context>, game_json_url: String) {
    let opts = &ctx.opts;
    let client = &ctx.client;

    log::info!("loading game data from {}", game_json_url);

    let loaded = convert::load_url(&ctx, &game_json_url).await;

    if loaded.is_none() {
        return;
    }

    let (src_manifest, raw_game_json) = loaded.unwrap();

    let slide_ids:Vec<String> = src_manifest.structure.slides.iter().map(|slide| slide.slide_id()).collect();

    log::info!("loaded manifest, game id: {} ({})", src_manifest.game_id(), game_json_url);

    let dest_dir = opts.dest_base_path.join(&src_manifest.game_id());
    if opts.skip_dir_exists && dest_dir.join(&opts.dest_json_dir).join("game.json").exists() {
        log::info!("skipping {} because dir already exists", &src_manifest.game_id());
        return;
    }
    std::fs::create_dir_all(&dest_dir);

    let (slides, medias) = convert::into_slides(&ctx, src_manifest, &game_json_url).await;

    if opts.write_json {
        let dest_path = dest_dir.join(&opts.dest_json_dir);
        std::fs::create_dir_all(&dest_path);

        {
            let mut file = File::create(&dest_path.join("game.json")).unwrap();
            let mut cursor = std::io::Cursor::new(raw_game_json);

            std::io::copy(&mut cursor, &mut file).unwrap();
        }

        {
            let dest_path = dest_path.join("slides");
            std::fs::create_dir_all(&dest_path);

            for (index, slide) in slides.into_iter().enumerate() {
                let id = &slide_ids[index];
                if let Ok(mut file) = File::create(&dest_path.join(format!("{}.json", id))) {
                    serde_json::to_writer_pretty(file, &slide).unwrap();
                } else {
                    panic!("unable to create file at {} (game json: {})", dest_path.join(format!("{}.json", id)).display().to_string(), game_json_url);
                }
            }
        }
    }

    if opts.download_media {
        let dest_path = dest_dir.join(&opts.dest_media_dir);
        std::fs::create_dir_all(&dest_path);

        for media in medias.iter() {

            let dest_dir = dest_path.join(&media.basepath);
            std::fs::create_dir_all(&dest_dir);
            let dest_path = dest_dir.join(&media.filename);


            if !opts.skip_download_exists || !dest_path.exists() {
                log::info!("downloading {} -> {}", media.url, dest_path.to_str().unwrap());
                match client.get(&media.url)
                    .send()
                    .await
                    .unwrap()
                    .error_for_status() {
                        Ok(resp) => {
                            let data = resp
                                .bytes()
                                .await
                                .unwrap();

                            let mut cursor = std::io::Cursor::new(data);

                            let mut dest_file = std::fs::File::create(&dest_path).unwrap();
                            std::io::copy(&mut cursor, &mut dest_file).unwrap();
                        },
                        Err(err) => {
                            if opts.allow_empty_media {
                                log::warn!("couldn't download {}", media.url)
                            } else {
                                panic!("couldn't download {}", media.url)
                            }
                        }
                    }

            }
        }


        if opts.transcode_media {

            for media in medias.iter() {
            
                let dest_dir = dest_path.join(&media.basepath);
                let src_file_path = dest_dir.join(&media.filename);
                let src_file_path_str = src_file_path.to_str().unwrap();

                if !src_file_path.exists() {
                    log::warn!("transcode src missing: {}", src_file_path_str);
                    continue;
                }

                if let Some((transcode, filename_dest)) = media.transcode.as_ref() {
                    match transcode {
                        MediaTranscode::Audio => {
 
                            let dest_file_path = dest_dir.join(filename_dest);
                            if !opts.skip_transcode_exists || !dest_file_path.exists() {
                                let dest_file_path = dest_file_path.to_str().unwrap();
                                log::info!("converting audio {} to {}...", media.file_stem(), dest_file_path);

                                Command::new("ffmpeg")
                                    .arg("-i")
                                    .arg(src_file_path_str)
                                    .arg("-acodec")
                                    .arg("libmp3lame")
                                    .arg(dest_file_path)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        },
                        MediaTranscode::Video => {
 
                            let dest_file_path = dest_dir.join(filename_dest);
                            if !opts.skip_transcode_exists || !dest_file_path.exists() {
                                let dest_file_path = dest_file_path.to_str().unwrap();
                                log::info!("converting video {} to {}...", media.file_stem(), dest_file_path);

                                Command::new("ffmpeg")
                                    .arg("-i")
                                    .arg(src_file_path_str)
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
                                    .arg(dest_file_path)
                                    .output()
                                    .expect("failed to execute process");
                            }
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
            }
        }
    }

}

fn init_logger(verbose:bool) {
    if verbose {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    }
}
