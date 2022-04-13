#![allow(warnings)]

use std::{future::Future, path::PathBuf, sync::atomic::Ordering, collections::hash_map::Entry};
use dotenv::dotenv;
use migrate::src_manifest::SrcManifest;
use shared::domain::jig::AudioBackground;
use simplelog::*;
use structopt::StructOpt;
use tokio::sync::Mutex;
use std::fs;
use std::fs::File;
use uuid::Uuid;
use std::process::Command;
use reqwest::Client; 
use serde::{Serialize, Deserialize};
use serde_json::{Result, value::RawValue};
use std::sync::Arc;
use crate::context::Context;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Write};
pub use shared::{
    api::{
        ApiEndpoint,
        endpoints
    },
    domain::{
        CreateResponse,
        meta::MetadataResponse,
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
            JigBrowseResponse,
            JigUpdateDraftDataRequest,
            Module,
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

use super::JigsLookupArc;
use super::create::LocalMeta;

#[derive(Default)]
struct LocalStats {
    pub n_updated: usize,
    pub n_created: usize,
    pub n_deleted: usize,
    pub unknown_audio: HashSet<String>,
    pub games_unaccounted: HashSet<String>,
    pub jigs_unaccounted: HashSet<String>,
}

pub async fn run(ctx:Arc<Context>, jigs: JigsLookupArc) {
    let local_meta = Arc::new(get_local_meta(&ctx).await);

    let local_stats = Arc::new(Mutex::new(LocalStats::default()));

    let mut jobs = get_sanitize_jigs_futures(ctx.clone(), local_stats.clone(), local_meta.clone(), jigs.clone()).await;
    _run(ctx.clone(), jobs).await;

    let mut jobs = get_update_jigs_futures(ctx.clone(), local_stats.clone(), local_meta.clone(), jigs.clone()).await;
    _run(ctx.clone(), jobs).await;

    let local_stats = local_stats.lock().await;

    if local_stats.unknown_audio.len() == 0 {
        log::info!("all audio accounted for!");
    } else {
        log::info!("unknown audio:");
        for audio in local_stats.unknown_audio.iter() {
            log::info!("{}", audio);
        }
    }

    log::info!("created {} new jigs", local_stats.n_created);
    log::info!("updated {} existing jigs", local_stats.n_updated);
    log::info!("unaccounted for {} games (existing jigs which have no tt manifest)", local_stats.games_unaccounted.len());

}

async fn _run(ctx: Arc<Context>, mut jobs: Vec<impl Future>) {
    let batch_size = ctx.opts.update_jigs_batch_size;

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

}

async fn get_sanitize_jigs_futures(ctx: Arc<Context>, local_stats:Arc<Mutex<LocalStats>>, local_meta: Arc<LocalMeta>, jigs_lookup: JigsLookupArc) -> Vec<impl Future> {
    let mut futures = Vec::new();
    
    let game_to_jig = jigs_lookup.lock().await.game_to_jig.clone();

    for (game_id, jig_list) in game_to_jig.into_iter() {
        local_stats.lock().await.games_unaccounted.insert(game_id.clone());
        futures.push({
            let jigs_lookup = jigs_lookup.clone();
            let ctx = ctx.clone();
            let local_stats = local_stats.clone();
            async move {
                if jig_list.len() > 1 {
                    let mut sorted = jig_list.clone();

                    sorted.sort_by(|a, b| a.published_at.partial_cmp(&b.published_at).unwrap());
                    sorted.remove(0);

                    log::info!("has multiple");
                    for jig in sorted.iter() {
                        log::info!("deleting {} {:?}", jig.id.0.to_string(), jig.published_at);
                        jigs_lookup.lock().await.jig_to_game.remove(&jig.id.0.to_string());
   
                        if !ctx.opts.dry_run {
                            let url = format!("{}{}", 
                                ctx.opts.get_remote_target().api_url(), 
                                endpoints::jig::Delete::PATH.replace("{id}", &jig.id.0.to_string())
                            );

                            let res = ctx
                                .client 
                                .delete(&url)
                                .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
                                .header("content-length", 0)
                                .send()
                                .await
                                .unwrap();

                            if !res.status().is_success() {
                                log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                                log::error!("Failed to delete all jig {}", jig.id.0.to_string());
                            }
                        }

                        local_stats.lock().await.n_deleted += 1;
                    }

                    jigs_lookup.lock().await.game_to_jig.insert(game_id, sorted);
                }
            }
        });
    }

    futures
}
async fn get_update_jigs_futures(ctx: Arc<Context>, local_stats:Arc<Mutex<LocalStats>>, local_meta: Arc<LocalMeta>, jigs: JigsLookupArc) -> Vec<impl Future> {
    let mut futures = Vec::new();

    let paths = fs::read_dir(&ctx.games_dir).unwrap();

    for path in paths {
        let (src_manifest, _) = crate::process::transcode::json::load_file(&ctx, path.unwrap().path().join("json").join("game.json")).unwrap();
        futures.push({
            let jigs = jigs.clone();
            let ctx = ctx.clone();
            let local_meta = local_meta.clone();
            let local_stats = local_stats.clone();
            async move {
                let game_id = src_manifest.game_id();
                local_stats.lock().await.games_unaccounted.remove(&game_id);

                log::info!("{}", game_id);

                match jigs.lock().await.game_to_jig.get(&game_id) {
                    Some(jig_list) => {
                        for jig in jig_list {
                            let jig_id = jig.id.0.to_string();
                            update_jig(&ctx, local_stats.clone(), &jig_id, &src_manifest).await;
                            writeln!(&ctx.finished_log, "{} {} {} (updated)", game_id, jig_id, src_manifest.album_store.album.fields.hash.as_ref().unwrap_or(&"[unknown]".to_string()));
                            local_stats.lock().await.n_updated += 1;
                        }
                    },
                    None => {
                        if ctx.opts.update_jigs_create_if_not_exist {
                            let jig_id = crate::process::jigs::create::run(&ctx, &local_meta, &src_manifest).await;
                            update_jig(&ctx, local_stats.clone(), &jig_id, &src_manifest).await;
                            writeln!(&ctx.finished_log, "{} {} {} (created)", game_id, jig_id, src_manifest.album_store.album.fields.hash.as_ref().unwrap_or(&"[unknown]".to_string()));
                            local_stats.lock().await.n_created += 1;
                        } else {
                            log::info!("SHOULD CREATE JIG FOR {}", game_id);
                        }
                    }
                }
            }
        });

    }

    futures
}

async fn update_jig(ctx: &Context, local_stats:Arc<Mutex<LocalStats>>, jig_id: &str, manifest: &SrcManifest) {
    log::info!("updating jig jig_id: {}, game_id: {}", jig_id, manifest.game_id());
    let lang = manifest.lang_str();
    let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", jig_id);
    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    let mut req = JigUpdateDraftDataRequest {
        language: Some(lang.to_string()),
        ..Default::default()
    };

    let bg_audio = {
        if let Some(music_file) = manifest.structure.music_file.as_ref() {
            let name = music_file.to_lowercase();
            if name.contains("halalu") {
                Some(AudioBackground::LegacyHanerotHalalu)
            } else if name.contains("jitap") {
                Some(AudioBackground::LegacyJiTap)
            } else if name.contains("maoz") {
                Some(AudioBackground::LegacyMaozTzur)
            } else if name.contains("modeh") {
                Some(AudioBackground::LegacyModehAni)
            } else if name.contains("hechiyanu") || name.contains("cheyanu") {
                Some(AudioBackground::LegacyShehechiyanu)
            } else if name.contains("zoo") {
                Some(AudioBackground::LegacyMorningZoo)
            } else if name.contains("march") {
                Some(AudioBackground::LegacyPlaylandMarch)
            } else if name.contains("walrus") {
                Some(AudioBackground::LegacyWanderingWalrus)
            } else if name.contains("romp") {
                Some(AudioBackground::LegacyIslandRomp)
            } else if name.contains("bars") {
                Some(AudioBackground::LegacyMonkeyBars)
            } else if name.contains("clouds") {
                Some(AudioBackground::LegacySunAndNoClouds)
            } else if name.contains("etude") {
                Some(AudioBackground::LegacyFirstEtude)
            } else if name.contains("bear") {
                Some(AudioBackground::LegacyTeddysBear)
            } else if name.contains("time") {
                Some(AudioBackground::LegacyNapTime)
            } else if name.contains("lullaby") {
                Some(AudioBackground::LegacyWindupLullaby)
            } else if name.contains("cuckoo") || name.contains("show") {
                Some(AudioBackground::LegacyCuckooToYou)
            } else if name.is_empty() || name.contains("silence") {
                None
            } else {
                local_stats.lock().await.unknown_audio.insert(music_file.clone());
                None
            }
        } else {
            None
        }
    };

    if let Some(bg_audio) = bg_audio {
        req.audio_background = Some(Some(bg_audio));
    }

    if !ctx.opts.dry_run {
        let res = ctx.client
            .patch(&url)
            .header("Authorization", &format!("Bearer {}", ctx.opts.token))
            .json(&req)
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            log::error!("unable to update jig: {:#?}", req);
            //panic!("unable to update jig!"); 
        }
    }
}

async fn get_local_meta(ctx:&Context) -> LocalMeta {
    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::meta::Get::PATH
    );
    let res = reqwest::Client::new()
        .get(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Failed to get jig data");
    }

    let MetadataResponse { affiliations, age_ranges, .. } = res.json().await.unwrap();

    let affiliations = affiliations
        .iter()
        .map(|x| x.id)
        .collect();

    let age_ranges = age_ranges
        .iter()
        .map(|x| x.id)
        .collect();

    LocalMeta { age_ranges, affiliations }
}
