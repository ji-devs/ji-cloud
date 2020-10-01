mod options;
mod data;

use std::future::Future;
use dotenv::dotenv;
use simplelog::*;
use options::Opts;
use structopt::StructOpt;
use tokio_util::codec::{BytesCodec, FramedRead};
use data::*;
use shared::{
    api::{ApiEndpoint, endpoints::image::*},
    domain::image::*
};
use reqwest::Body;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{delay_for, Duration};

pub struct Credentials {
    pub token:String,
    pub csrf:String
}

impl Credentials {
    pub fn new() -> Self {
        let token = std::env::var("UTILS_TOKEN").expect("Need UTILS_TOKEN in .env");
        let csrf = std::env::var("UTILS_CSRF").expect("Need UTILS_CSRF in .env");

        Self {
            token,
            csrf
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    let credentials = Arc::new(Credentials::new());
    let opts = Arc::new(opts);
    //just used for logging/debugging - makes it easier to see the process order
    let count = Arc::new(AtomicUsize::new(0));
    let mut jobs = get_futures(count.clone(), opts.clone(), credentials.clone());
    let mut futures = FuturesUnordered::new();

    let batch_size = *&opts.batch_size;

    //See: https://users.rust-lang.org/t/awaiting-futuresunordered/49295/5
    //Idea is we try to have a saturated queue of futures
    while let Some(next_job) = jobs.pop() {
        while futures.len() >= batch_size {
            futures.next().await;
        }
        futures.push(next_job);
    }
    while let Some(_) = futures.next().await {}

    log::info!("finished: {} items uploaded!", count.load(Ordering::SeqCst));
}

fn get_futures(count:Arc<AtomicUsize>, opts:Arc<Opts>, credentials:Arc<Credentials>) -> Vec<impl Future> {

    let is_debug = *&opts.debug;
    let limit_debug = *&opts.limit_debug;

    let iter = 
        load_albums(&opts) 
        .into_iter()
        .flat_map(move |album| {
            let opts = opts.clone();
            let credentials = credentials.clone();
            let count = count.clone();
            let album_name = Arc::new(album.name);
            let album_id = Arc::new(album.id);
            album.list
                .into_iter()
                .map({
                    move |item| {
                        upload_image(count.clone(), opts.clone(), credentials.clone(), album_name.clone(), album_id.clone(), item.sprite)
                    }
                })
        })
        .into_iter();

    if is_debug {
        iter.take(limit_debug).collect()
    } else {
        iter.collect()
    }

}

async fn upload_image(count:Arc<AtomicUsize>, opts:Arc<Opts>, credentials:Arc<Credentials>, album_name:Arc<String>, album_id: Arc<String>, file_name:String) {
    let album_id = &album_id;
    let album_name = &album_name;

    let path = opts.get_image_path(album_id, &file_name);

    let count_num = count.fetch_add(1, Ordering::SeqCst) +1;

    log::info!("uploading #{}: {}/{}", count_num, album_name, file_name);

    let content_type = {
        if file_name.contains(".png") {
            "image/png"
        } else if file_name.contains(".jpg") {
            "image/jpeg"
        } else if file_name.contains(".gif") {
            "image/gif"
        } else if file_name.contains(".svg") {
            "image/svg+xml"
        } else {
            panic!("unknown content type!");
        }
    };

    let file = tokio::fs::File::open(path).await.unwrap();

    let file_size = file.metadata().await.unwrap().len();

    if opts.dry_run {
        log::info!("Skipping due to dry run: #{} {}/{}", count_num, album_name, file_name);
        if opts.debug && opts.sleep_debug != 0 {
            delay_for(Duration::from_millis(opts.sleep_debug)).await;
        }
        return;
    }

    let url = format!("{}{}", opts.get_remote_target().api_url(), Create::PATH);

    let req_data = CreateRequest {
        name: file_name.to_string(),
        description: format!("from {} pack", album_name), 
        is_premium: false,
        publish_at: None,
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new()
    };

    let request = reqwest::Client::new()
        .post(&url)
        .header("X-CSRF", &credentials.csrf)
        .header("Cookie", &format!("X-JWT={}", credentials.token))
        .json(&req_data);
   

    let res = request
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("{:?}", res);
        panic!("Failed to get CreateResponse!");
    }

    let res = res
        .json::<CreateResponse>()
        .await
        .unwrap();

    let CreateResponse { id, upload_url} = res;

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    let request = reqwest::Client::new()
        .put(&upload_url.to_string())
        .header("Content-Type", content_type)
        .header("Content-Length", file_size) 
        .body(body);

    let res = request
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("{:?}", res);
        panic!("Failed to upload image!");
    }

    log::info!("uploaded #{} {}/{}. id {}", count_num, album_name, file_name, id.0);
}

fn load_albums(opts:&Opts) -> Vec<Album> {
    let manifest = Manifest::load(&opts);

    manifest.list
        .into_iter()
        .map(|item| {
            Album::load(&opts, item.id, item.name)
        })
        .collect()
}

fn init_logger(verbose:bool) {
    if verbose {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        ])
        .unwrap();
    }
}
