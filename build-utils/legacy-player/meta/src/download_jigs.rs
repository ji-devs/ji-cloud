use super::context::Context;
use std::sync::Arc;
use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use shared::domain::module::ModuleKind;
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
use futures::stream::{FuturesUnordered, StreamExt};
use futures::lock::Mutex;
use std::collections::{HashMap, HashSet};
use shared::{
    api::{
        ApiEndpoint,
        PathParts,
        endpoints,
    },
    domain::jig::{
        JigResponse,
        JigBrowseResponse
    }
};

pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();
    _run(ctx.clone(), get_jig_futures(ctx.clone()).await).await;
    ctx.stats.jigs_set_completed();
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

async fn get_jig_futures(ctx: Arc<Context>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    let mut page_num = 1;
    async fn do_browse(ctx: &Context, page: usize) -> JigBrowseResponse {
        let url = format!(
            "{}{}?authorId=me&jigFocus=modules&page={}&draftOrLive=draft",
            ctx.opts.get_remote_target().api_url(),
            <endpoints::jig::Browse as ApiEndpoint>::Path::PATH,
            page
        );

        log::info!("{}", url);

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
            let has_downloaded = has_downloaded.clone();
            async move {
                log::info!("downloading jigs from page {}", page);
                for jig in do_browse(&ctx, page).await.jigs {
                    let jig_id = jig.id.0.to_string();
                    if has_downloaded.lock().await.contains(&jig_id) {
                        panic!("jig {} is a duplicate from pagination", jig_id);
                        //log::info!("jig {} is a duplicate from pagination", jig_id);
                    } else {
                        for module in jig.jig_data.modules.iter() {
                            if module.kind != ModuleKind::Legacy && module.kind != ModuleKind::Cover {
                                panic!("jig_id {:?}, module_id {:?} is not a legacy module!", jig.id, module.id);
                            }
                        }
                        if !ctx.opts.dry_run {
                            let dest_path = ctx.jigs_dir.join(&format!("{}.json", jig_id));
                            if dest_path.exists() {
                                log::warn!("{} already exists!", jig_id)
                            }
                            let mut file = File::create(&dest_path).unwrap();
                            serde_json::to_writer_pretty(&file, &jig).unwrap();
                        }
                        has_downloaded.lock().await.insert(jig_id);
                        ctx.stats.jigs_increase();
                        log::info!("written {} jigs", ctx.stats.jigs_count());
                    } 
                }
            }
        });

        if let Some(limit) = ctx.opts.download_jigs_page_stop_limit {
            if page_num as usize >= limit {
                break;
            }
        }
    }

    futures
}
