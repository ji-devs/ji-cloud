mod options;
mod data;
mod report;

use std::future::Future;
use dotenv::dotenv;
use simplelog::*;
use options::Opts;
use structopt::StructOpt;
use tokio_util::codec::{BytesCodec, FramedRead};
use data::*;
use report::*;
use shared::{
    api::{ApiEndpoint, endpoints::image::*},
    domain::image::*
};
use reqwest::Body;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, RwLock};
use std::collections::HashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{delay_for, Duration};

pub struct Credentials {
    pub token:String,
}

impl Credentials {
    pub fn new() -> Self {
        let token = std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env");

        Self {
            token,
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
    let report = Arc::new(RwLock::new(Report::default()));
    let mut jobs = get_futures(opts.clone(), credentials.clone(), report.clone());
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

    {
        let report = report.read().unwrap();
        log::info!("writing report...");
        report.write_csv(&opts);
        log::info!("finished: {}/{} items uploaded ({} skipped, {} total)!", report.n_uploaded, report.n_to_upload, report.n_skipped, report.n_skipped + report.n_to_upload);
    }
}

fn get_futures(opts:Arc<Opts>, credentials:Arc<Credentials>, report:Arc<RwLock<Report>>) -> Vec<impl Future> {

    let is_debug = *&opts.debug;
    let limit_debug = *&opts.limit_debug;

    let albums = load_albums(&opts);

    let mut skip_albums:Vec<Album> = Vec::new();
    let mut upload_albums:Vec<UploadAlbum> = Vec::new();

    {
        let mut total_index:usize = 0;
        let mut skip_count:usize = 0;
        let mut upload_count:usize = 0;

        let mut report = report.write().unwrap();
        for (album_index, album) in albums.into_iter().enumerate() {
            let mut report_album:ReportAlbum = (&album).into(); 
            let mut skip_album = Album::new(album.id.clone(), album.name.clone());
            let mut upload_album = UploadAlbum::new(album.id, album.name);
             

            for (item_index, item) in album.list.into_iter().enumerate() {
                //only upload sticker types
                if item.item_type == 0 {
                    upload_album.list.push(UploadAlbumItem::new(total_index, album_index, item_index, item));
                    upload_count += 1;
                } else {
                    {
                        report_album.list[item_index].is_skipped = true;
                    }
                    skip_album.list.push(item);
                    skip_count += 1;
                }
                total_index += 1;
            }

            report.albums.push(report_album);

            skip_albums.push(skip_album);
            upload_albums.push(upload_album);
        }
        report.n_skipped = skip_count; 
        report.n_to_upload = upload_count; 
    }
    
    
    let upload_futures = upload_albums
        .into_iter()
        .flat_map(move |album| {
            let opts = opts.clone();
            let credentials = credentials.clone();
            let album_name = Arc::new(album.name);
            let album_id = Arc::new(album.id);
            let report = report.clone();
            album.list
                .into_iter()
                .map({
                    move |item| {
                        upload_image(opts.clone(), credentials.clone(), album_name.clone(), album_id.clone(), item, report.clone())
                    }
                })
        })
        .into_iter();

    if is_debug {
        upload_futures.take(limit_debug).collect()
    } else {
        upload_futures.collect()
    }

}

async fn upload_image(opts:Arc<Opts>, credentials:Arc<Credentials>, album_name:Arc<String>, album_id: Arc<String>, item: UploadAlbumItem, report: Arc<RwLock<Report>>) {
    let album_id = &album_id;
    let album_name = &album_name;

    let path = opts.get_image_path(album_id, &item.name);

    let item_count = item.total_index+1;
    log::info!("uploading #{}: {}/{}", item_count, album_name, item.name);

    let content_type = {
        if item.name.contains(".png") {
            "image/png"
        } else if item.name.contains(".jpg") {
            "image/jpeg"
        } else if item.name.contains(".gif") {
            "image/gif"
        } else if item.name.contains(".svg") {
            "image/svg+xml"
        } else {
            panic!("unknown content type!");
        }
    };

    let file = tokio::fs::File::open(path).await.unwrap();

    let file_size = file.metadata().await.unwrap().len();

    if opts.dry_run {
        log::info!("Skipping due to dry run: #{} {}/{}", item_count, album_name, item.name);
        if opts.debug && opts.sleep_debug != 0 {
            delay_for(Duration::from_millis(opts.sleep_debug)).await;
        }
        return;
    }

    let url = format!("{}{}", opts.get_remote_target().api_url(), Create::PATH);

    let kind = match item.kind {
        AlbumItemKind::Sticker => ImageKind::Sticker,
        AlbumItemKind::Foreground => ImageKind::Canvas,
        AlbumItemKind::Background => ImageKind::Canvas,
        //2: Animation
        //3: Foreground
        _ => panic!("unsupported album item kind: {:?}", item.kind)
    };
    let req_data = CreateRequest {
        name: item.name.to_string(),
        description: format!("from {} pack", album_name), 
        is_premium: false,
        publish_at: None,
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new(),
        kind
    };

    let request = reqwest::Client::new()
        .post(&url)
        .header("AUTHORIZATION", &format!("Bearer {}", &credentials.token))
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

    let CreateResponse { id } = res;
   
    {
        let mut report = report.write().unwrap();
        report.albums[item.album_index].list[item.item_index].remote_id = Some(id.0.to_string());
    }
    let upload_url = format!("{}{}", 
       opts.get_remote_target().api_url(), 
       Upload::PATH.replace("{id}",&id.0.to_string())
    );

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    let request = reqwest::Client::new()
        .patch(&upload_url)
        .header("X-CSRF", &credentials.csrf)
        .header("Cookie", &format!("X-JWT={}", credentials.token))
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

    {
        let mut report = report.write().unwrap();
        report.n_uploaded += 1; 
    }
    log::info!("uploaded #{} {}/{}. id {}", item_count, album_name, item.name, id.0);
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
