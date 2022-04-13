#![allow(warnings)]

use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use std::process::Command;
use reqwest::Client; 
use serde::Deserialize;
use serde_json::{Result, value::RawValue};
use std::sync::Arc;
use crate::context::Context;
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn run(ctx:Arc<Context>) {
    let batch_size = *&ctx.opts.albums_batch_size;
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

    let mut stats = ctx.stats.lock().await;
    stats.downloaded_tt_albums = true;
    stats.write();

}

async fn get_futures(ctx:Arc<Context>) -> Vec<impl Future> {
    let mut futures = Vec::new();
    let mut page_num = 1;

    loop {
        log::info!("querying page {}...", page_num);

        if let Some(albums) = load_page(&ctx, page_num).await {
            page_num += 1;
            for (pk, album) in albums.into_iter() {
                futures.push({
                    let ctx = ctx.clone();
                    async move {
                        log::info!("writing album {}", pk);

                        if !ctx.opts.dry_run {
                            let dest_path = ctx.albums_dir.join(&format!("{}.json", pk));
                            if ctx.opts.albums_warn_exists && dest_path.exists() {
                                log::warn!("{} already exists!", pk)
                            }
                            let mut file = File::create(&dest_path).unwrap();
                            let mut cursor = std::io::Cursor::new(album);

                            std::io::copy(&mut cursor, &mut file).unwrap();
                        }
                        ctx.stats.lock().await.n_tt_albums += 1;
                    }
                });
            }
        } else {
            break;
        }

        if Some(page_num as usize) == ctx.opts.albums_page_stop_limit {
            break;
        }
    }

    futures
}

#[derive(Deserialize)]
struct WithPk {
    pk: u64,
}

#[derive(Deserialize)]
struct AlbumList<'a> {
    #[serde(borrow)]
    data: Vec<&'a RawValue>,
}

pub async fn load_page(ctx: &Context, page_num: u32) -> Option<Vec<(u64, String)>> {
    let url = format!("https://jitap.net/community/api/albums/recent/?page_num={}&per_page={}&language=0&category=0&ageGroup=0 ", page_num, ctx.opts.albums_per_page);

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

    match serde_json::from_str::<AlbumList<'_>>(&text) {
        Ok(list) => {
            if list.data.len() > 0 {
                Some(
                    list.data.into_iter().map(|value| {
                        let raw = serde_json::to_string(&value).unwrap();
                        let with_pk = serde_json::from_str::<WithPk>(&raw).unwrap();
                        (with_pk.pk, raw)
                    }).collect()
                )
            } else {
                println!("no more entries!");
                None
            }
        },
        Err(_) => {
            println!("error parsing json! raw text:");
            println!("{}", text);
            None
        }
    }

}
