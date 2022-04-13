#![allow(warnings)]

use std::collections::{HashMap, HashSet};
use std::{future::Future, path::PathBuf, sync::atomic::Ordering, collections::hash_map::Entry};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use std::process::Command;
use reqwest::Client; 
use serde::{Serialize, Deserialize};
use serde_json::{Result, value::RawValue};
use std::sync::Arc;
use crate::context::Context;
use futures::stream::{FuturesUnordered, StreamExt};
use futures::lock::Mutex;
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

use super::{JigsLookup, JigsLookupArc};

pub async fn run(ctx:Arc<Context>) -> JigsLookupArc {
    let jigs = Arc::new(Mutex::new(JigsLookup::default()));
    let jig_list = Arc::new(Mutex::new(Vec::new()));

    _run(ctx.clone(), get_page_futures(ctx.clone(), jig_list.clone()).await).await;
    _run(ctx.clone(), get_jig_futures(ctx.clone(), jigs.clone(), jig_list.clone()).await).await;

    let mut stats = ctx.stats.lock().await;
    stats.downloaded_jigs = true;
    stats.write();

    log::info!("downloaded {} jigs", stats.n_jigs);
    jigs
}

async fn _run(ctx: Arc<Context>, mut jobs: Vec<impl Future>) {
    let batch_size = ctx.opts.download_jigs_batch_size;

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

async fn get_page_futures(ctx: Arc<Context>, jig_list: Arc<Mutex<Vec<JigResponse>>>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    let mut page_num = 1;
    async fn do_browse(ctx: &Context, page: usize) -> JigBrowseResponse {
        let url = format!(
            "{}{}?authorId=me&jigFocus=modules&page={}&draftOrLive=live",
            ctx.opts.get_remote_target().api_url(),
            endpoints::jig::Browse::PATH,
            page
        );
        let res = ctx
            .client
            .get(&url)
            .header("Authorization", &format!("Bearer {}", ctx.opts.token))
            .send()
            .await;

        let res = res.unwrap();

        if !res.status().is_success() {
            panic!("error code: {}, details: {:?}", res.status().as_str(), res);
        }

        res.json().await.unwrap()
    }

    // just for getting the number of pages
    let JigBrowseResponse {
        pages,
        total_jig_count,
        ..
    } = do_browse(&ctx, 0).await;

    log::info!("Updating {} pages, {} jigs total ", pages, total_jig_count);

    let has_downloaded:Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    // now really iterate over each page
    for page in (0..=pages as usize) {
        page_num += 1;
        futures.push({
            let ctx = ctx.clone();
            let jig_list = jig_list.clone();
            let has_downloaded = has_downloaded.clone();
            async move {
                log::info!("downloading jigs from page {}", page);
                for jig in do_browse(&ctx, page).await.jigs {
                    let jig_id = jig.id.0.to_string();
                    if !has_downloaded.lock().await.contains(&jig_id) {
                        ctx.stats.lock().await.n_jigs += 1;
                        jig_list.lock().await.push(jig);
                        has_downloaded.lock().await.insert(jig_id);
                    } else {
                        log::info!("jig {} is a duplicate from pagination", jig_id);
                    }
                }
            }
        });
        if Some(page_num as usize) == ctx.opts.download_jigs_page_stop_limit {
            break;
        }
    }

    futures
}
async fn get_jig_futures(ctx: Arc<Context>, jigs: JigsLookupArc, jig_list: Arc<Mutex<Vec<JigResponse>>>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    for jig in jig_list.lock().await.iter() {
        if jig.jig_data.modules.len() > 0 {
            futures.push({
                let jig = jig.clone();
                let ctx = ctx.clone();
                let jigs = jigs.clone();
                async move {
                    let jig_id_str = jig.id.0.to_string();
                    let module = jig.jig_data.modules.get(0).unwrap();
                    let module_id_str = module.id.0.to_string();

                    log::info!("downloading jig {}", jig_id_str);

                    let url = format!("{}{}", 
                        ctx.opts.get_remote_target().api_url(), 
                        endpoints::jig::module::GetDraft::PATH
                            .replace("{id}", &jig_id_str)
                            .replace("{module_id}", &module_id_str)
                    );


                    let res = ctx 
                        .client
                        .get(url)
                        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
                        .send()
                        .await
                        .unwrap();
                    
                    if !res.status().is_success() {
                        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                        panic!("Failed to get module data");
                    }

                    let ModuleResponse { module } = res.json().await.unwrap();

                    match module.body {
                        ModuleBody::Legacy(body) => {
                            let game_id = body.game_id;
                            let mut lock = jigs.lock().await;
                            
                            match lock.jig_to_game.entry(jig_id_str) {
                                Entry::Vacant(entry) => {
                                    entry.insert(vec![game_id.clone()]);
                                },
                                Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(game_id.clone())
                                }
                            }


                            match lock.game_to_jig.entry(game_id) {
                                Entry::Vacant(entry) => {
                                    entry.insert(vec![jig]);
                                },
                                Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(jig)
                                }
                            }
                        },
                        _ => log::warn!("module is not a legacy!")
                    }
                }
            });
        } else {
            let jig_id_str = jig.id.0.to_string();
            writeln!(&ctx.warnings_log, "jig {} has no modules", jig_id_str);
            log::warn!("jig {} has no modules", jig_id_str)
        }
    }

    futures
}
