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
    pub n_deleted: usize,
}

pub async fn run(ctx:Arc<Context>, jigs: JigsLookupArc) {
    let local_stats = Arc::new(Mutex::new(LocalStats::default()));

    let mut jobs = get_delete_jig_duplicates_futures(ctx.clone(), local_stats.clone(), jigs.clone()).await;
    _run(ctx.clone(), jobs).await;

    let local_stats = local_stats.lock().await;

    log::info!("deleted {} jigs", local_stats.n_deleted);
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

async fn get_delete_jig_duplicates_futures(ctx: Arc<Context>, local_stats:Arc<Mutex<LocalStats>>, jigs_lookup: JigsLookupArc) -> Vec<impl Future> {
    let mut futures = Vec::new();
    
    let game_to_jig = jigs_lookup.lock().await.game_to_jig.clone();

    for (game_id, jig_list) in game_to_jig.into_iter() {
        futures.push({
            let ctx = ctx.clone();
            let local_stats = local_stats.clone();
            let mut sorted = jig_list.clone();
            async move {
                if jig_list.len() > 1 {

                    sorted.sort_by(|a, b| a.published_at.partial_cmp(&b.published_at).unwrap());
                    sorted.remove(0);

                    log::info!("has multiple");
                    for jig in sorted.iter() {
                        log::info!("deleting {} {:?}", jig.id.0.to_string(), jig.published_at);
   
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
                }
            }
        });
    }

    futures
}
