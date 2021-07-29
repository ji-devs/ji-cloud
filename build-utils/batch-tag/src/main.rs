#![allow(warnings)]

mod options;
mod report;
mod context;
mod data;
mod requests;

use context::*;
use data::*;

use std::future::Future;
use tokio_util::codec::{BytesCodec, FramedRead};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, RwLock};
use std::collections::HashMap;
use futures::stream::{FuturesUnordered, StreamExt};


#[tokio::main]
async fn main() {
    let ctx = Arc::new(Context::new());

    let meta = requests::get_meta(ctx.clone()).await.unwrap();
    let image_list = requests::get_image_list(ctx.clone(), &meta).await.unwrap();

    ctx.report.write().await.set_from_images(&image_list);

    let mut jobs = get_jobs(ctx.clone(), image_list, meta);
    let mut futures = FuturesUnordered::new();

    ctx.report.read().await.pre_log();

    //See: https://users.rust-lang.org/t/awaiting-futuresunordered/49295/5
    //Idea is we try to have a saturated queue of futures
    while let Some(next_job) = jobs.pop() {
        while futures.len() >= ctx.opts.batch_size {
            futures.next().await;
        }
        futures.push(next_job);
    }
    while let Some(_) = futures.next().await {}

    ctx.report.read().await.final_log();
}



fn get_jobs(ctx: Arc<Context>, image_list: Vec<ImageInfo>, meta: MetaInfo) -> Vec<impl Future> {
    let meta = Arc::new(meta);

    image_list
        .into_iter()
        .map(|image| {
            requests::fix_image(ctx.clone(), meta.clone(), image)
        })
        .collect()
}
