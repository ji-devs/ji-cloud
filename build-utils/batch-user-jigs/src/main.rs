#![allow(warnings)]
mod context;
use context::*;
mod options;
use options::*;

use simplelog::*;
use std::future::Future;
use std::sync::Arc;
use futures::stream::{FuturesUnordered, StreamExt};
use dotenv::dotenv;
use structopt::StructOpt;
use shared::{
    api::{ApiEndpoint, endpoints},
    domain::jig::{
        JigId,
        JigBrowseResponse,
        module::{
            ModuleId,
            ModuleKind
        }
    }
};
use serde::Deserialize;

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

async fn get_futures(ctx:Arc<Context>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    let url = format!("{}{}?authorId=me&jigFocus=modules", ctx.opts.get_remote_target().api_url(), endpoints::jig::Browse::PATH);

    let res = ctx.client
        .get(&url)
        .header("Authorization", &format!("Bearer {}", ctx.token))
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("unable to get jigs pages!"); 
    }

    let JigBrowseResponse { pages, total_jig_count, ..} = res.json().await.unwrap();

    log::info!("Updating {} pages, {} jigs total ", pages, total_jig_count);

    for page in (0..pages) {
        futures.push({
            let ctx = ctx.clone();
            async move {
                let url = format!("{}{}?authorId=me&jigFocus=modules&page={}", ctx.opts.get_remote_target().api_url(), endpoints::jig::Browse::PATH, page);
                log::info!("loading jigs for page {}", page);

                let res = ctx.client
                    .get(&url)
                    .header("Authorization", &format!("Bearer {}", ctx.token))
                    .send()
                    .await
                    .unwrap();

                if !res.status().is_success() {
                    log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                    panic!("unable to browse jigs at page {}!", page); 
                }

                let JigBrowseResponse { jigs, total_jig_count, ..} = res.json().await.unwrap();

                for jig in jigs {
                    for module in jig.jig_data.modules {
                        log::info!("updating jig {} module {}!", jig.id.0.to_string(), module.id.0.to_string()); 
                        // currently update is just to queue screenshot for each module in the jig
                        call_screenshot_service(ctx.clone(), jig.id, module.id, module.kind).await;
                    }
                }
            }
        });
    }

    futures
}


pub async fn call_screenshot_service(ctx:Arc<Context>, jig_id: JigId, module_id: ModuleId, kind: ModuleKind) {
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

    let res = ctx.client
        .get(&url)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("unable to call screenshot service for jig {} module {}!", jig_id.0.to_string(), module_id.0.to_string()); 
    }

    let resp:ScreenshotResponse = res.json().await.unwrap();
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
