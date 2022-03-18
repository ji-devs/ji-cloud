#![allow(warnings)]
mod context;
use context::*;
mod options;
use futures::lock::Mutex;
use legacy_transcode::src_manifest::{SrcManifest, SrcManifestData};
use options::*;

use dotenv::dotenv;
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::StatusCode;
use serde::Deserialize;
use shared::domain::jig::{AudioBackground, JigUpdateDraftDataRequest};
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::jig::{
        module::{ModuleId, ModuleKind},
        JigBrowseResponse, JigId,
    },
};
use simplelog::*;
use std::sync::Arc;
use std::{future::Future, process::exit};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    let ctx = Arc::new(Context::new(opts).await);

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

async fn get_futures(ctx: Arc<Context>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    async fn do_browse(ctx: &Context, page: usize) -> Result<JigBrowseResponse, reqwest::Error> {
        let url = format!(
            "{}{}?authorId=c3666fb2-827b-11eb-979f-0f06b8a150e4&jigFocus=modules&page={}&draftOrLive=live",
            ctx.opts.get_remote_target().api_url(),
            endpoints::jig::Browse::PATH,
            page
        );
        let res = ctx
            .client
            .get(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .send()
            .await;

        let res = res.unwrap();

        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        }

        res.json().await
    }
    let JigBrowseResponse {
        jigs,
        pages,
        total_jig_count,
        ..
    } = do_browse(&ctx, 0).await.unwrap();

    log::info!("Updating {} pages, {} jigs total ", pages, total_jig_count);

    for page in (0..=pages as usize) {
        futures.push({
            let ctx = ctx.clone();
            let mem = mem.clone();
            async move {
                match do_browse(&ctx, page).await {
                    Ok(res) => {
                        // log::info!("loading jigs for page {}", page);

                        let JigBrowseResponse { jigs, .. } = res;

                        for jig in jigs {

                            if ctx.opts.log_duplicate_jig {
                                if mem.lock().await.contains(&jig.id.0) {
                                    println!("Duplicate: {:?}", jig.id.0);
                                }
                                mem.lock().await.insert(jig.id.0.clone());
                            }

                            if ctx.opts.update_background_music {
                                update_background_music(ctx.clone(), jig.id).await;
                            }

                            if ctx.opts.update_screenshots {
                                for module in jig.jig_data.modules {
                                    let url = format!(
                                        "{}/screenshot/{}/{}/thumb.jpg",
                                        ctx.opts.get_remote_target().uploads_url(),
                                        jig.id.0.to_string(),
                                        module.id.0.to_string()
                                    );
                                    let res = ctx.client.get(&url).send().await.unwrap();

                                    match res.status() {
                                        StatusCode::NOT_FOUND => {
                                            log::info!(
                                                "updating {}/screenshot/{}/{}/thumb.jpg",
                                                ctx.opts.get_remote_target().uploads_url(),
                                                jig.id.0.to_string(),
                                                module.id.0.to_string()
                                            );
                                            // currently update is just to queue screenshot for each module in the jig
                                            call_screenshot_service(
                                                ctx.clone(),
                                                jig.id,
                                                module.id,
                                                module.kind,
                                            )
                                            .await;
                                        }
                                        StatusCode::OK => {
                                            log::info!(
                                                "skipping {}/screenshot/{}/{}/thumb.jpg",
                                                ctx.opts.get_remote_target().uploads_url(),
                                                jig.id.0.to_string(),
                                                module.id.0.to_string()
                                            );
                                        }
                                        e => {
                                            log::info!(
                                                "error {:?} {}/screenshot/{}/{}/thumb.jpg",
                                                e,
                                                ctx.opts.get_remote_target().uploads_url(),
                                                jig.id.0.to_string(),
                                                module.id.0.to_string()
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        });
    }

    futures
}

pub async fn update_background_music(ctx: Arc<Context>, jig_id: JigId) {
    let jig_id_str = jig_id.0.to_string();
    let game_id = ctx.legacy_lookup.get(&jig_id_str).unwrap();
    log::info!("loading manifest at {}", SrcManifestData::url(game_id));
    let manifest = SrcManifestData::load_game_id(&ctx.client, game_id)
        .await
        .data;

    if let Some(music_file) = manifest.structure.music_file {
        let target_music_file = music_file.split('/').last().unwrap_or(&music_file);
        let target_music_file = target_music_file
            .split('.')
            .next()
            .unwrap_or(&target_music_file);
        let target_music_file = target_music_file.to_lowercase();
        let target_music_file = target_music_file.trim();
        if !target_music_file.is_empty() {
            let target_enum: Option<AudioBackground> = if target_music_file.contains("silence") {
                None
            } else if target_music_file.contains("hanerothalalu") {
                Some(AudioBackground::LegacyHanerotHalalu)
            } else if target_music_file.contains("jitap") {
                Some(AudioBackground::LegacyJiTap)
            } else if target_music_file.contains("maoztzur") {
                Some(AudioBackground::LegacyMaozTzur)
            } else if target_music_file.contains("modehani") {
                Some(AudioBackground::LegacyModehAni)
            } else if target_music_file.contains("shehechiyanu") {
                Some(AudioBackground::LegacyShehechiyanu)
            } else if target_music_file.contains("cuckoo") {
                Some(AudioBackground::LegacyCuckooToYou)
            } else if target_music_file.contains("morning-zoo") {
                Some(AudioBackground::LegacyMorningZoo)
            } else if target_music_file.contains("playland-march") {
                Some(AudioBackground::LegacyPlaylandMarch)
            } else if target_music_file.contains("wandering-walrus") {
                Some(AudioBackground::LegacyWanderingWalrus)
            } else if target_music_file.contains("island-romp") {
                Some(AudioBackground::LegacyIslandRomp)
            } else if target_music_file.contains("monkey-bars") {
                Some(AudioBackground::LegacyMonkeyBars)
            } else if target_music_file.contains("sun-and-no-clouds") {
                Some(AudioBackground::LegacySunAndNoClouds)
            } else if target_music_file.contains("first-etude") {
                Some(AudioBackground::LegacyFirstEtude)
            } else if target_music_file.contains("teddys-bear") {
                Some(AudioBackground::LegacyTeddysBear)
            } else if target_music_file.contains("nap-time") {
                Some(AudioBackground::LegacyNapTime)
            } else if target_music_file.contains("windup-lullaby") {
                Some(AudioBackground::LegacyWindupLullaby)
            } else if target_music_file.contains("silence") {
                Some(AudioBackground::LegacyWindupLullaby)
            } else {
                panic!("unsupported bg music file: {}", music_file)
            };

            // no need to set in the case of "silence"
            if let Some(target_enum) = target_enum {
                log::info!(
                    "updating background music in jig {} (game id: {}), music file: {}",
                    jig_id_str,
                    game_id,
                    music_file
                );

                let req = JigUpdateDraftDataRequest {
                    audio_background: Some(Some(target_enum)),
                    ..Default::default()
                };

                if !ctx.opts.dry_run {
                    let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id_str);
                    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);
                    let res = ctx
                        .client
                        .patch(&url)
                        .header("Authorization", &format!("Bearer {}", ctx.token))
                        .json(&req)
                        .send()
                        .await
                        .unwrap();

                    if !res.status().is_success() {
                        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                        panic!("unable to update jig!");
                    }

                    let path = endpoints::jig::Publish::PATH.replace("{id}", &jig_id_str);
                    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);
                    let res = ctx
                        .client
                        .put(&url)
                        .header("Authorization", &format!("Bearer {}", ctx.token))
                        .header("content-length", 0)
                        .send()
                        .await
                        .unwrap();

                    if !res.status().is_success() {
                        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                        panic!("unable to publish jig!");
                    }
                }
            }
        }
    }
}
pub async fn call_screenshot_service(
    ctx: Arc<Context>,
    jig_id: JigId,
    module_id: ModuleId,
    kind: ModuleKind,
) {
    #[derive(Deserialize)]
    struct ScreenshotResponse {
        jpg: String,
        #[serde(rename = "taskName")]
        task_name: String,
        #[serde(rename = "taskUrl")]
        task_url: String,
    }

    let url = format!(
        "{}?jig={}&module={}&kind={}",
        ctx.opts.get_remote_target().screenshot_url(),
        jig_id.0.to_string(),
        module_id.0.to_string(),
        kind.as_str()
    );

    let res = ctx.client.get(&url).send().await.unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!(
            "unable to call screenshot service for jig {} module {}!",
            jig_id.0.to_string(),
            module_id.0.to_string()
        );
    }

    let resp: ScreenshotResponse = res.json().await.unwrap();
}

fn init_logger(verbose: bool) {
    if verbose {
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    } else {
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }
}
