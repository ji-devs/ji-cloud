#![allow(warnings)]

use std::collections::HashSet;

use dotenv::dotenv;

mod record;
mod stats;
mod context;
mod json;
mod transcode;
mod src_manifest;
use context::*;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let ctx = Context::new();

    //json::run(ctx.clone()).await;
    //transcode::download::run(ctx.clone()).await;
    transcode::convert::run(ctx.clone()).await;
}
