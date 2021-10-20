#![allow(warnings)]

mod binary;
pub use binary::*;

use std::{future::Future, path::PathBuf};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs::File;
use options::*;
use ::transcode::src_manifest::*;
use shared::domain::jig::module::ModuleBody;

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

            let file = File::create(&dest_path.join("jig.json")).unwrap();
            serde_json::to_writer_pretty(file, &jig_req).unwrap();

            for (index, module_req) in module_reqs.iter().enumerate() {
                let file = File::create(&dest_path.join(format!("module-{}.json", index))).unwrap();
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

                let file = File::create(&dest_path.join(format!("{}.json", id))).unwrap();
                serde_json::to_writer_pretty(file, &slide).unwrap();
            }
        }
    }

    if opts.download_media {
        let dest_path = dest_dir.join(&opts.dest_media_dir);
        std::fs::create_dir_all(&dest_path);

        for media in medias {
            let data = reqwest::get(&media.url)
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();

            let mut cursor = std::io::Cursor::new(data);

            let dest_dir = dest_path.join(&media.basepath);
            std::fs::create_dir_all(&dest_dir);

            let mut dest_file = std::fs::File::create(dest_dir.join(&media.filename)).unwrap();
            std::io::copy(&mut cursor, &mut dest_file).unwrap();
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
