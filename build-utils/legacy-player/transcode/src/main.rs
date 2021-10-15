#![allow(warnings)]

mod binary;
pub use binary::*;

use std::future::Future;
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

    transcode_game(&opts, "web-stress-test").await;
}

async fn transcode_game(opts: &Opts, game_id: &str) {

    let src_path = opts.src_path.join(&game_id);
    let src_json = src_path.join(&opts.src_json);

    let src_manifest = SrcManifest::load(src_json);

    let jig_req = src_manifest.jig_req();
    let module_reqs = src_manifest.module_reqs(game_id);
    let slides = src_manifest.into_slides();

    let dest_path = src_path.join(&opts.dest_dir);
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
