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
use std::process::Command;
use reqwest::Client; 
use serde::Deserialize;
use serde_json::{Result, value::RawValue};

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

    if !ctx.opts.dry_run {
        std::fs::create_dir_all(&ctx.opts.dest_dir);
    }

    let mut page_num = 1;
    let mut total = 0;

    loop {

        log::info!("querying page {}...", page_num);
        if let Some(albums) = load_page(&ctx, page_num).await {
            total += albums.len();
            log::info!("writing {} albums from page {}...", albums.len(), page_num);
            for (pk, album) in albums.into_iter() {
                if !ctx.opts.dry_run {
                    let dest_path = ctx.opts.dest_dir.join(&format!("{}.json", pk));
                    if dest_path.exists() {
                        panic!("{} already exists!", pk)
                    }
                    let mut file = File::create(&dest_path).unwrap();
                    let mut cursor = std::io::Cursor::new(album);

                    std::io::copy(&mut cursor, &mut file).unwrap();
                }
            }
            page_num += 1;
        } else {
            break;
        }
    }

    log::info!("wrote {} albums!", total);
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
    let url = format!("https://jitap.net/community/api/albums/recent/?page_num={}&per_page={}&language=0&category=0&ageGroup=0 ", page_num, ctx.opts.per_page);

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
