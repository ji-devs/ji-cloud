#![allow(warnings)]

use std::{future::Future, path::PathBuf};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use ::transcode::{
    src_manifest::*,
};
pub use shared::{
    api::{
        ApiEndpoint,
        endpoints
    },
    domain::{
        CreateResponse,
        jig::{
            JigId,
            JigCreateRequest, 
            JigData, 
            JigPlayerSettings, 
            module::{
                ModuleCreateRequest, 
                ModuleBody, 
                ModuleId,
                body::{
                    Transform,
                    _groups::design::{PathCommand, TraceKind, TraceShape, YoutubeUrl},
                    legacy::{
                        ModuleData,
                        slide::*,
                        design::*,
                        activity::*
                    }
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

    let ctx = Context::new(opts);

    match &ctx.opts.game_id {
        None => {
            let paths = fs::read_dir(&ctx.opts.src_base_path).unwrap();
            for path in paths {
                let path = path.unwrap().path();
                let game_id = path.file_stem().unwrap().to_str().unwrap(); 
                parse(&ctx, game_id).await;
            }
        },
        Some(game_id) => {
            parse(&ctx, &game_id).await;
        }
    }
}

async fn parse(ctx: &Context, game_id: &str) {
    let manifest:SrcManifest = {
        let path = ctx.opts.src_base_path.join(&format!("{}/json/game.json", game_id));
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    };

    log::info!("{}: {} slides", game_id, manifest.structure.slides.len());

    let jig_id = make_jig(ctx, &manifest).await;
    log::info!("got jig id: {}", jig_id.0.to_string());
    assign_modules(ctx, game_id, &jig_id, &manifest).await;

    if ctx.opts.publish {
        publish_jig(ctx, &jig_id).await;
    }
}

async fn make_jig(ctx:&Context, manifest: &SrcManifest) -> JigId {

    // TODO- populate
    let req = JigCreateRequest { 
        display_name: manifest.album_store.album.fields.name.clone().unwrap_or_default(),
        goals: Vec::new(), 
        age_ranges: Vec::new(), 
        affiliations: Vec::new(), 
        language: None, 
        categories: Vec::new(), 
        description: manifest.album_store.album.fields.description.clone().unwrap_or_default(),
        default_player_settings: JigPlayerSettings::default()
    };

    let path = endpoints::jig::Create::PATH;
    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    log::info!("calling {}", url);

    if(ctx.opts.dry_run) {
        log::info!("{:#?}", req);

        JigId(Uuid::nil())
    } else {

        let resp = ctx.client
            .post(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .json(&req)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        if !resp.status().is_success() {
            panic!("error!"); 
        }


        let body: serde_json::Value = resp.json().await.unwrap();
        let body:CreateResponse<JigId> = serde_json::from_value(body).unwrap();

        body.id
    }
}

async fn assign_modules(ctx:&Context, game_id: &str, jig_id: &JigId, manifest: &SrcManifest) {

    for slide in manifest.structure.slides.iter() {
        let req = ModuleCreateRequest {
            body: ModuleBody::Legacy(
                ModuleData {
                    game_id: game_id.to_string(),
                    slide_id: slide.slide_id()
                },
            )
        };

        let path = endpoints::jig::module::Create::PATH.replace("{id}", &jig_id.0.to_string());
        let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

        log::info!("calling {}", url);

        let module_id = {
            if(ctx.opts.dry_run) {
                log::info!("{:#?}", req);

                ModuleId(Uuid::nil())
            } else {

                let resp = ctx.client
                    .post(&url)
                    .header("Authorization", &format!("Bearer {}", ctx.token))
                    .json(&req)
                    .send()
                    .await
                    .unwrap()
                    .error_for_status()
                    .unwrap();

                if !resp.status().is_success() {
                    panic!("error!"); 
                }

                let body: serde_json::Value = resp.json().await.unwrap();
                let body:CreateResponse<ModuleId> = serde_json::from_value(body).unwrap();

                body.id
            }
        };
    }
}

async fn publish_jig(ctx:&Context, jig_id: &JigId) {

    let path = endpoints::jig::Publish::PATH.replace("{id}", &jig_id.0.to_string());
    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    log::info!("calling {}", url);

    if !ctx.opts.dry_run {
        let resp = ctx.client
            .post(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        if !resp.status().is_success() {
            panic!("error!"); 
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
