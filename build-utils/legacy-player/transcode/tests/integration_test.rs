use ::transcode::src_manifest::*;
use legacy::Manifest;

#[allow(warnings)]

use std::{
    path::{Path, PathBuf},
    panic
};
use simplelog::*;

const SRC_GAME_PATH:&'static str = "D:\\Dropbox (Jewish Interactive)\\ji-cloud-media\\legacy\\examples\\web-stress-test";

const SRC_GAME_JSON:&'static str = "game.json";

#[test]
fn all_good() {
    run_test(|| {
        let src_json = PathBuf::from(SRC_GAME_PATH)
            .join(PathBuf::from(SRC_GAME_JSON));

        log::info!("source json: {:?}", src_json);

        let src_manifest = SrcManifest::load(src_json);

        log::info!("loaded!");

        let (manifest, slides) = src_manifest.convert();

        log::info!("converted!");

        //log::info!("{:#?}", manifest);
        //log::info!("{:#?}", slides);
    });
}

// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
fn run_test<T>(test: T) -> ()
    where T: FnOnce() -> () + panic::UnwindSafe
{
    //setup logger

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
    ])
    .unwrap();

    let result = panic::catch_unwind(|| {
        test()
    });

    assert!(result.is_ok())
}
