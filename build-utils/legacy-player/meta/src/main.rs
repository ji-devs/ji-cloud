#![allow(warnings)]

use dotenv::dotenv;
use simplelog::*;

mod context;
mod download_albums;
mod download_jigs;
mod download_modules;
mod create_or_update_jigs;
mod src_manifest;
mod report;
mod stats;
mod record;
use context::*;

// url
// http://localhost:4104/module/legacy/play/debug/debug?game_id=ID&slide_index=0
// or maybe? http://localhost:4104/module/legacy/play/debug?slide_index=0&game_id=ID

#[tokio::main]
async fn main() {
    dotenv().ok();
    let ctx = Context::new();
    init_logger(&ctx);

    //download_albums::run(ctx.clone()).await;
    //download_jigs::run(ctx.clone()).await;
    //download_modules::run(ctx.clone()).await;
    //report::run(ctx.clone()).await;
    //create_or_update_jigs::run(ctx.clone()).await;
}

fn init_logger(ctx: &Context) {
    if ctx.opts.verbose {
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
