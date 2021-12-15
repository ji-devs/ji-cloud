#![allow(warnings)]

use std::{future::Future, path::PathBuf, vec};
use components::module::_common::prelude::Image;
use dotenv::dotenv;
use shared::{domain::{image::user::UserImageUploadResponse, jig::{module::{body::Background, ModuleUpdateRequest}, PrivacyLevel}, meta::{GoalId, AgeRangeId, AffiliationId}}, media::MediaLibrary, config::RemoteTarget};
use tokio_util::codec::{BytesCodec, FramedRead};
use reqwest::Body;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::convert::TryInto;
use uuid::Uuid;
use ::transcode::{
    src_manifest::*,
    jig_log::JigInfoLogLine
};
use futures::stream::{FuturesUnordered, StreamExt};
pub use shared::{
    api::{
        ApiEndpoint,
        endpoints
    },
    domain::{
        CreateResponse,
        image::{
            ImageId,
            ImageKind,
            user::{
                UserImageCreateRequest,
                UserImageUploadRequest,
            },
        },
        category::CategoryId,
        jig::{
            JigId,
            JigCreateRequest, 
            JigData, 
            JigPlayerSettings, 
            JigResponse,
            JigUpdateDraftDataRequest,
            module::{
                ModuleCreateRequest, 
                ModuleBody, 
                ModuleId,
                ModuleKind,
                ModuleResponse,
                StableOrUniqueId,
                body::{
                    Transform,
                    _groups::design::{PathCommand, TraceKind, TraceShape, YoutubeUrl},
                    legacy::{
                        ModuleData,
                        slide::*,
                        design::*,
                        activity::*
                    },
                }
            }
        }
    }
};
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::process::Command;
use reqwest::Client; 

mod context;
use context::*;
mod options;
use options::*;

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
    let mut info_lines = ctx.info_lines.lock().unwrap().take().unwrap();
    info_lines
        .into_iter()
        .map(|info_line| {
            parse(ctx.clone(), info_line)
        })
        .collect()
}
async fn parse(ctx: Arc<Context>, info_line: JigInfoLogLine) {
    let ctx = &ctx;

    log::info!("loading manfiest for game_id: {}, jig_id_string: {}, game_hash: {}", 
        info_line.game_id,
        info_line.jig_id,
        info_line.game_hash
    );

    let url = format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/games/{}/json/game.json", info_line.game_id);

    let text = ctx
        .client
        .get(&url)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .await
        .unwrap();

    let manifest:SrcManifest = serde_json::from_str(&text).unwrap();


    // not actually doing anything yet...
    // but should be trivial to update jig based on game data
    // log::info!("loaded! got {} slides..", manifest.structure.slides.len());
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
