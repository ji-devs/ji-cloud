#![allow(warnings)]

use std::{future::Future, path::PathBuf};
use std::sync::Arc;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs::{self, File};
use std::io::{BufReader, Write};
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
use futures::lock::Mutex;
use std::thread;

mod context;
use context::*;
mod options;
use options::*;
mod process;
pub mod stats;

// url
// http://localhost:4104/module/legacy/play/debug/debug?game_id=ID&slide_index=0
// or maybe? http://localhost:4104/module/legacy/play/debug?slide_index=0&game_id=ID

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    let ctx = Arc::new(Context::new(opts));


    if ctx.opts.transcode_only_game_id.is_none() {
        if ctx.opts.process_download_albums && (!ctx.opts.albums_skip_if_stats_completed || !ctx.stats.lock().await.downloaded_tt_albums) {
            log::info!("downloading albums..");
            process::albums::run(ctx.clone()).await;
        }
    }

    let medias = Arc::new(Mutex::new(Vec::new()));

    if ctx.opts.process_transcode_game_json {
        // by this point we have all the albums and jigs, 
        // or they are written to disk from a previous run.
        // now get the list of urls and modules and transcode them

        log::info!("getting game urls..");
        let game_urls = process::game_urls::load(&ctx);

        log::info!("transcoding json for {} urls..", game_urls.len());
        process::transcode::json::run(ctx.clone(), game_urls, medias.clone()).await;
    }

    if ctx.opts.process_transcode_game_media {
        log::info!("transcoding media..");
        process::transcode::media::run(ctx.clone(), medias.clone()).await;
    }

    if ctx.opts.process_update_jigs {
        log::info!("downloading jigs..");
        let jigs = process::jigs::download::run(ctx.clone()).await;
        log::info!("updating jigs..");
        process::jigs::update::run(ctx.clone(), jigs.clone()).await;
    }
    log::info!("done!");
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
