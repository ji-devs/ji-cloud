use super::context::Context;
use std::sync::Arc;
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
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();
    let mut page_num = 1;

    loop {
        log::info!("querying page {}...", page_num);

        if let Some(albums) = load_page(&ctx, page_num).await {
            page_num += 1;
            for (pk, album) in albums.into_iter() {
                log::info!("writing album {}", pk);

                if !ctx.opts.dry_run {
                    let dest_path = ctx.albums_dir.join(&format!("{}.json", pk));
                    if dest_path.exists() {
                        log::warn!("{} already exists!", pk)
                    }
                    let mut file = File::create(&dest_path).unwrap();
                    let mut cursor = std::io::Cursor::new(album);

                    std::io::copy(&mut cursor, &mut file).unwrap();
                }
                ctx.stats.tt_albums_increase()
            }

            log::info!("written {} albums", ctx.stats.tt_albums_count());
        } else {
            break;
        }

        if let Some(limit) = ctx.opts.download_albums_page_stop_limit {
            if page_num as usize >= limit {
                break;
            }
        }
    }

    ctx.stats.tt_albums_set_completed();
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
    let url = format!("https://jitap.net/community/api/albums/recent/?page_num={}&per_page={}&language=0&category=0&ageGroup=0 ", page_num, ctx.opts.download_albums_per_page);

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
            log::info!("found {} entries", list.data.len());
            if list.data.len() > 0 {
                Some(
                    list.data.into_iter().map(|value| {
                        let raw = serde_json::to_string(&value).unwrap();
                        let with_pk = serde_json::from_str::<WithPk>(&raw).unwrap();
                        (with_pk.pk, raw)
                    }).collect()
                )
            } else {
                None
            }
        },
        Err(_) => {
            log::error!("error parsing json! raw text:");
            log::error!("{}", text);
            None
        }
    }
}


