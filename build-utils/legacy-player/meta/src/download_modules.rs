use super::context::Context;
use std::sync::Arc;
use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use shared::domain::module::ModuleKind;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
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
    domain::{
        jig::JigResponse,
        module::{ModuleResponse, ModuleBody}
    }
};

pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();
    _run(ctx.clone(), get_module_futures(ctx.clone()).await).await;
    ctx.stats.modules_set_completed();
    println!("wrote {} jigs and {} modules", ctx.stats.jigs_count(), ctx.stats.modules_count());
}

async fn _run(ctx: Arc<Context>, mut jobs: Vec<impl Future>) {
    let batch_size = ctx.opts.download_modules_batch_size;

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

async fn get_module_futures(ctx: Arc<Context>) -> Vec<impl Future> {

    let mut futures = Vec::new();

    let paths = fs::read_dir(&ctx.jigs_dir).unwrap();

    for path in paths {
        let jig = read_jig_to_file(path.unwrap().path());
        let jig_id_str = jig.id.0.to_string();
        ctx.stats.jigs_increase();

        let module_jig_dir = ctx.modules_dir.join(&jig_id_str);
        if !ctx.opts.dry_run {
            std::fs::create_dir_all(&module_jig_dir);
        }
        for module in jig.jig_data.modules {
            futures.push({
                let jig_id_str = jig_id_str.clone();
                let module_id_str = module.id.0.to_string();
                let ctx = ctx.clone();
                let module_jig_dir = module_jig_dir.clone();
                async move {
                    let url = format!("{}{}", 
                        ctx.opts.get_remote_target().api_url(), 
                        <endpoints::module::GetDraft as ApiEndpoint>::Path::PATH
                            .replace("{asset_type}", "jig")
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

                    match &module.body {
                        ModuleBody::Legacy(body) => {
                            if !ctx.opts.dry_run {
                                let dest_path = module_jig_dir.join(&format!("{}.json", module_id_str));
                                if dest_path.exists() {
                                    log::warn!("{}/{}.json already exists!", jig_id_str, module_id_str)
                                }
                                let mut file = File::create(&dest_path).unwrap();
                                serde_json::to_writer_pretty(&file, &module).unwrap();
                            }
                            println!("wrote game id {} (module #{})", body.game_id, ctx.stats.modules_count() +1);
                            ctx.stats.modules_increase();
                        },
                        _ => panic!("module {} of jig {} is not a legacy!", module_id_str, jig_id_str)
                    }
                }
            });
        }

        if let Some(limit) = ctx.opts.download_modules_jig_stop_limit {
            if ctx.stats.jigs_count() >= limit {
                break;
            }
        }
    }

    futures
}

fn read_jig_to_file(path: PathBuf) -> JigResponse {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}
