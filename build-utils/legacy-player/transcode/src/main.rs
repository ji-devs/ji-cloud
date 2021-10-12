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

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    let src_path = opts.src_path.join(&opts.base_id);
    let src_json = src_path.join(opts.src_json);

    let src_manifest = SrcManifest::load(src_json);
    let (dest_manifest, dest_modules) = src_manifest.convert(&opts.base_id);

    let dest_path = src_path.join(opts.dest_dir);
    std::fs::create_dir_all(&dest_path);
    let file = File::create(&dest_path.join("manifest.json")).unwrap();

    serde_json::to_writer_pretty(file, &dest_manifest).unwrap();

    for (index, module) in dest_modules.into_iter().enumerate() {
        let file = File::create(&dest_path.join(format!("module-{}.json", index+1))).unwrap();
        serde_json::to_writer_pretty(file, &module).unwrap();
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
