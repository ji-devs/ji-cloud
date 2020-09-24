mod options;
mod data;

use dotenv::dotenv;
use simplelog::*;
use options::Opts;
use structopt::StructOpt;
use std::{
    fs,
    path::PathBuf,
};
use tokio_util::codec::{BytesCodec, FramedRead};
use data::*;
use shared::{
    api::{ApiEndpoint, endpoints::image::*},
    domain::image::*
};
use reqwest::{Body, StatusCode};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use futures::stream::{FuturesUnordered, StreamExt, Stream};

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
    let opts = Opts::from_args();
    init_logger(&opts);

    let albums = load_albums(&opts);

    //upload_image(&opts, &albums[0], &albums[0].list[0]).await;

    let credentials = Arc::new(Credentials::new());
    let opts = Arc::new(opts);

    let mut count = Arc::new(AtomicUsize::new(0));

    {
        let count = count.clone();
        let mut futures:FuturesUnordered<_> = 
            albums
            .into_iter()
            .flat_map(move |album| {
                let album_id = album.id.clone();
                let album_name = album.name.clone();
                let opts = opts.clone();
                let credentials = credentials.clone();
                let count = count.clone();
                album.list
                    .into_iter()
                    .map({
                        move |item| {
                            upload_image(count.clone(), opts.clone(), credentials.clone(), album_id.clone(), album_name.clone(), item.sprite)
                        }
                    })
            })
            .collect();

        while futures.next().await.is_some() { }
    }

    log::info!("finished: {} items uploaded!", count.load(Ordering::SeqCst));
}

async fn upload_image(count:Arc<AtomicUsize>, opts:Arc<Opts>, credentials:Arc<Credentials>, album_id:String, album_name:String, file_name:String) {
    let path = opts.get_image_path(&album_id, &file_name);

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
        log::info!("Skipping due to dry run: {}/{}", album_name, file_name);
        count.fetch_add(1, Ordering::SeqCst);
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

    log::info!("uploaded {}/{} - (id: {})", album_name, file_name, id.0);

    count.fetch_add(1, Ordering::SeqCst);
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

fn init_logger(opts:&Opts) {
    if opts.verbose {
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
