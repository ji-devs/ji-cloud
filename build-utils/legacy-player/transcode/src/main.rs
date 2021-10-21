#![allow(warnings)]

mod binary;
pub use binary::*;

use std::{future::Future, path::PathBuf};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs::File;
use std::io::Write;
use options::*;
use ::transcode::src_manifest::*;
use shared::domain::jig::module::ModuleBody;
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::process::Command;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    transcode_game(&opts).await;
}

async fn transcode_game(opts: &Opts) {
    let src_manifest = SrcManifest::load_url(&opts.game_json_url).await;


    let dest_dir = opts.dest_base_path.join(&src_manifest.game_id());
    std::fs::create_dir_all(&dest_dir);

    let jig_req = src_manifest.jig_req();
    let module_reqs = src_manifest.module_reqs();
    let (slides, medias) = src_manifest.into_slides();

    if opts.write_json {
        let dest_path = dest_dir.join(&opts.dest_json_dir);
        std::fs::create_dir_all(&dest_path);

        {
            let dest_path = dest_path.join("requests");
            std::fs::create_dir_all(&dest_path);

            let mut file = File::create(&dest_path.join("jig.json")).unwrap();
            serde_json::to_writer_pretty(file, &jig_req).unwrap();


            for (index, module_req) in module_reqs.iter().enumerate() {
                let mut file = File::create(&dest_path.join(format!("module-{}.json", index))).unwrap();
                serde_json::to_writer_pretty(file, &module_req).unwrap();
            }
        }

        {
            let dest_path = dest_path.join("slides");
            std::fs::create_dir_all(&dest_path);

            for (index, slide) in slides.into_iter().enumerate() {
                let id = match &module_reqs[index].body {
                    ModuleBody::Legacy(legacy) => &legacy.slide_id,
                    _ => panic!("not a legacy module?!")
                };

                let mut file = File::create(&dest_path.join(format!("{}.json", id))).unwrap();
                serde_json::to_writer_pretty(file, &slide).unwrap();
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
                let data = reqwest::get(&media.url)
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();

                let mut cursor = std::io::Cursor::new(data);

                let mut dest_file = std::fs::File::create(&dest_path).unwrap();
                std::io::copy(&mut cursor, &mut dest_file).unwrap();
            }
        }


        if opts.transcode_media {

            for media in medias.iter() {
            
                let dest_dir = dest_path.join(&media.basepath);
                let src_file_path = dest_dir.join(&media.filename);
                let src_file_path = src_file_path.to_str().unwrap();

                if let Some(transcode) = media.transcode.as_ref() {
                    match transcode {
                        MediaTranscode::Audio => {

                            let dest_file_path = dest_dir.join(&format!("{}.mp3", media.file_stem()));
                            if !opts.skip_transcode_exists || !dest_file_path.exists() {
                                let dest_file_path = dest_file_path.to_str().unwrap();
                                log::info!("converting audio {} to {}...", media.file_stem(), dest_file_path);

                                Command::new("ffmpeg")
                                    .arg("-i")
                                    .arg(src_file_path)
                                    .arg("-acodec")
                                    .arg("libmp3lame")
                                    .arg(dest_file_path)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        },
                        MediaTranscode::Animation => {

                            let dest_file_path = dest_dir.join(&format!("{}.webm", media.file_stem()));

                            if !opts.skip_transcode_exists || !dest_file_path.exists() {
                                let dest_file_path = dest_file_path.to_str().unwrap();
                                log::info!("converting animation {} to {}...", media.file_stem(), dest_file_path);

                                // ffmpeg -i 2_anim.gif -c vp9 -b:v 0 -crf 26 -pix_fmt yuva420p test001.webm
                                Command::new("ffmpeg")
                                    .arg("-i")
                                    .arg(src_file_path)
                                    .arg("-c")
                                    .arg("vp9")
                                    .arg("-b:v")
                                    .arg("0")
                                    .arg("-crf")
                                    .arg("26")
                                    .arg("-pix_fmt")
                                    .arg("-yuva420p")
                                    .arg(dest_file_path)
                                    .output()
                                    .expect("failed to execute process");
                            }

                            /// works for prores but huge file:
                            /// ffmpeg -i 2_anim.gif -c prores -pix_fmt yuva444p10le -profile:v 4 -vf pad="width=ceil(iw/2)*2:height=ceil(ih/2)*2" test001.mov
                            
                            // unfortunately, yuva420p isn't supported here
                            // let dest_file_path = dest_dir.join(&format!("{}.mp4", media.file_stem()));

                            // if !opts.skip_transcode_exists || !dest_file_path.exists() {
                            //     let dest_file_path = dest_file_path.to_str().unwrap();
                            //     log::info!("converting animation {} to {}...", media.file_stem(), dest_file_path);
                            //      ffmpeg -i 2_anim.gif -c libx265 -b:v 0 -crf 26 -pix_fmt yuva420p test006.mp4
                            //     Command::new("ffmpeg")
                            //         .arg("-i")
                            //         .arg(src_file_path)
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
                        }
                    }
                }
            }
        }
    }

}

fn init_logger(verbose:bool) {
    if verbose {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        ])
        .unwrap();
    }
}
