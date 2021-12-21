#![allow(warnings)]

use std::{future::Future, path::PathBuf, vec};
use components::module::_common::prelude::Image;
use dotenv::dotenv;
use shared::{domain::{image::user::UserImageUploadResponse, jig::{module::{body::Background, ModuleUpdateRequest}, PrivacyLevel}, meta::{GoalId, AgeRangeId, AffiliationId}}, media::MediaLibrary, config::RemoteTarget};
use tokio_util::codec::{BytesCodec, FramedRead};
use reqwest::Body;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::convert::TryInto;
use uuid::Uuid;
use ::transcode::{
    src_manifest::*,
    jig_log::JigInfoLogLine
};
use futures::stream::{FuturesUnordered, StreamExt};
pub use shared::{
    api::{
        ApiEndpoint,
        endpoints
    },
    domain::{
        CreateResponse,
        image::{
            ImageId,
            ImageKind,
            user::{
                UserImageCreateRequest,
                UserImageUploadRequest,
            },
        },
        category::CategoryId,
        jig::{
            JigId,
            JigCreateRequest, 
            JigData, 
            JigPlayerSettings, 
            JigResponse,
            JigUpdateDraftDataRequest,
            module::{
                ModuleCreateRequest, 
                ModuleBody, 
                ModuleId,
                ModuleKind,
                ModuleResponse,
                StableOrUniqueId,
                body::{
                    Transform,
                    _groups::design::{PathCommand, TraceKind, TraceShape, YoutubeUrl},
                    legacy::{
                        ModuleData,
                        slide::*,
                        design::*,
                        activity::*
                    },
                }
            }
        }
    }
};
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::process::Command;
use reqwest::Client; 

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

    let ctx = Arc::new(Context::new(opts).await);



    let batch_size = *&ctx.opts.batch_size;
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

    log::info!("done!");
}

async fn get_futures(ctx:Arc<Context>) -> Vec<impl Future> {
    match ctx.opts.game_id.clone() {
        None => {

            let res = reqwest::Client::new()
                .get(&ctx.opts.game_ids_list_url)
                .send()
                .await
                .unwrap();

            if !res.status().is_success() {
                log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                panic!("Failed to get jig data");
            }

            res
                .text()
                .await
                .unwrap()
                .lines()
                .map(|line| parse(ctx.clone(), line.to_string()))
                .collect()
        },
        Some(game_id) => {
            vec![parse(ctx.clone(), game_id)]
        }
    }
}
async fn parse(ctx: Arc<Context>, game_id: String) {
    let ctx = &ctx;

    if let Some(game_id) = ctx.skip_game_ids.iter().find(|skip_game_id| game_id == **skip_game_id) {
        log::info!("skipping {}", game_id);
        return;
    }

    let url = format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/games/{}/json/game.json", game_id);

    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Failed to get game json for {}", game_id);
    }

    let manifest:SrcManifest = serde_json::from_str(&res.text().await.unwrap()).unwrap();

    log::info!("{}: {} slides", game_id, manifest.structure.slides.len());

    let image_id = if manifest.structure.slides.len() > 0 {
        Some(upload_cover_image(ctx, &game_id, &manifest.structure.slides[0]).await)
    } else {
        None
    };
    
    let jig_id = make_jig(ctx, &manifest).await;
    log::info!("got jig id: {}", jig_id.0.to_string());
    assign_modules(ctx, &game_id, &jig_id, &manifest).await;

    if !ctx.opts.dry_run {
        if let Some(image_id) = image_id {
            log::info!("setting cover");
            assign_cover_image(ctx, &jig_id, image_id).await;
        }

        publish_jig(ctx, &jig_id).await;

        let game_hash = manifest.album_store.album.fields.hash.unwrap_or_else(|| "[unknown]".to_string());

       
        JigInfoLogLine {
            jig_id: jig_id.0.to_string(),
            game_id,
            game_hash
        }.write_line(&ctx.info_log);
    }


}

async fn upload_cover_image(ctx:&Context, game_id: &str, slide: &transcode::src_manifest::Slide) -> ImageId {
    //get file info

    let url = format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/games/{}/media/slides/{}", game_id, slide.image_full);

    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .unwrap();

    let data = res.bytes().await.unwrap();
    let file_size = data.len();

    let content_type = {
        if slide.image_full.contains(".png") {
            "image/png"
        } else if slide.image_full.contains(".jpg") {
            "image/jpeg"
        } else if slide.image_full.contains(".gif") {
            "image/gif"
        } else if slide.image_full.contains(".svg") {
            "image/svg+xml"
        } else {
            panic!("unknown content type!");
        }
    };
    //get image id

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::image::user::Create::PATH
    );

    let req_data = UserImageCreateRequest {
        kind: ImageKind::Canvas,
    };


    let image_id = if ctx.opts.dry_run {
        ImageId(Uuid::nil())
    } else {

        let res = reqwest::Client::new()
            .post(url)
            .header("AUTHORIZATION", &format!("Bearer {}", &ctx.token))
            .json(&req_data)
            .send()
            .await
            .unwrap();

            //.json::<CreateResponse<ImageId>>()
        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("Failed to get CreateResponse!");
        }

        let CreateResponse { id } = res.json().await.unwrap();

        id
    };

    //get upload url
        
    let req_data = UserImageUploadRequest {
        file_size: file_size.try_into().unwrap()
    };

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::image::user::Upload::PATH.replace("{id}", &image_id.0.to_string())
    );

    let session_uri = if ctx.opts.dry_run {
        "https://example.com".to_string()
    } else {

        let res = reqwest::Client::new()
            .put(url)
            .header("AUTHORIZATION", &format!("Bearer {}", &ctx.token))
            .json(&req_data)
            .send()
            .await
            .unwrap();
        
        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("Failed to get UserImageUploadResponse!");
        }
        let UserImageUploadResponse { session_uri } = res.json().await.unwrap();

        session_uri
    };

    //upload it

    let body:Body = data.into(); 

    if ctx.opts.dry_run {
        image_id
    } else {
        let res = reqwest::Client::new()
            .put(&session_uri)
            .header("Content-Type", content_type)
            .header("Content-Length", file_size) 
            .body(body)
            .send()
            .await
            .unwrap();


        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("Failed to upload image to storage!");
        }

        image_id
    }
}

async fn assign_cover_image(ctx:&Context, jig_id: &JigId, image_id: ImageId) {

    //get jig data 
        
    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string())
    );


    let res = reqwest::Client::new()
        .get(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.token))
        .send()
        .await
        .unwrap();
    
    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Failed to get jig data");
    }

    let JigResponse { jig_data, .. } = res.json().await.unwrap();

    // get cover data
    let lite_module = jig_data.modules.iter().find(|m| m.kind == ModuleKind::Cover).unwrap();

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::jig::module::GetDraft::PATH
            .replace("{id}", &jig_id.0.to_string())
            .replace("{module_id}", &lite_module.id.0.to_string())
    );


    let res = reqwest::Client::new()
        .get(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.token))
        .send()
        .await
        .unwrap();
    
    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Failed to get jig data");
    }

    let ModuleResponse { module } = res.json().await.unwrap();

    // mutate the data
    let mut body = match module.body {
        ModuleBody::Cover(body) => body,
        _ => panic!("couldn't get module body!")
    };

    match body.content.as_mut() {
        Some(content) => {
            content.base.backgrounds.layer_1 = Some(Background::Image(Image {
                id: image_id,
                lib: MediaLibrary::User
            }))
        },
        None => {
            panic!("couldn't get body content!");
        }
    }

    // Upload the new module data
 
    let req_data = ModuleUpdateRequest {
        id: StableOrUniqueId::Unique(lite_module.id),
        body: Some(ModuleBody::Cover(body)),
        index: None,
        is_complete: Some(true)
    };

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::jig::module::Update::PATH.replace("{id}", &jig_id.0.to_string())
    );

    let res = reqwest::Client::new()
        .patch(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.token))
        .json(&req_data)
        .send()
        .await
        .unwrap();
        
    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Failed to update module!");
    }

    // indicate that the jig has a cover

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::jig::Cover::PATH.replace("{id}", &jig_id.0.to_string())
    );

    let res = reqwest::Client::new()
        .patch(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.token))
        .send()
        .await
        .unwrap();
        
    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Unable to indicate that jig has cover!");
    }
}

async fn make_jig(ctx:&Context, manifest: &SrcManifest) -> JigId {

    let author_byline = match &manifest.album_store.album.fields.author {
        None => "(Originally created on Ji Tap)".to_string(),
        Some(author) => {
            match (&author.first_name, &author.last_name) {
                (Some(first_name), Some(last_name)) => {
                    format!("(Originally created on Ji Tap by {} {})", first_name, last_name)
                },

                (Some(first_name), None) => {
                    format!("(Originally created on Ji Tap by {})", first_name)
                },
                _ => "(Originally created on Ji Tap)".to_string(),
            }
        }
    };

    let req = JigCreateRequest { 
        display_name: manifest.album_store.album.fields.name.clone().unwrap_or_default(),
        goals: Vec::new(), 
        age_ranges: ctx.age_ranges.clone(), 
        affiliations: ctx.affiliations.clone(), 
        language: Some(manifest.lang_str().to_string()),
        categories: Vec::new(), 
        description: format!("{} {}", 
            manifest.album_store.album.fields.description.clone().unwrap_or_default(),
            author_byline
        ),
        default_player_settings: JigPlayerSettings::default(),
        ..JigCreateRequest::default()
    };
    let path = endpoints::jig::Create::PATH;
    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    let jig_id = if(ctx.opts.dry_run) {
        //log::info!("{:#?}", req);

        JigId(Uuid::nil())
    } else {

        let res = ctx.client
            .post(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .json(&req)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("unable to create jig!"); 
        }


        let body: serde_json::Value = res.json().await.unwrap();
        let body:CreateResponse<JigId> = serde_json::from_value(body).unwrap();

        body.id
    };

    // update jig settings

    let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id.0.to_string());
    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    let req = JigUpdateDraftDataRequest {
        privacy_level: Some(PrivacyLevel::Public),
        ..Default::default()
    };

    if !ctx.opts.dry_run {
        let res = ctx.client
            .patch(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .json(&req)
            .send()
            .await
            .unwrap();


        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("unable to update jig!"); 
        }
    }

    jig_id

}

async fn assign_modules(ctx:&Context, game_id: &str, jig_id: &JigId, manifest: &SrcManifest) {

    for slide in manifest.structure.slides.iter() {
        let req = ModuleCreateRequest {
            body: ModuleBody::Legacy(
                ModuleData {
                    game_id: game_id.to_string(),
                    slide_id: slide.slide_id()
                },
            )
        };

        let path = endpoints::jig::module::Create::PATH.replace("{id}", &jig_id.0.to_string());
        let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

        log::info!("calling {}", url);

        let module_id = {
            if(ctx.opts.dry_run) {
                //log::info!("{:#?}", req);

                ModuleId(Uuid::nil())
            } else {

                let res = ctx.client
                    .post(&url)
                    .header("Authorization", &format!("Bearer {}", ctx.token))
                    .json(&req)
                    .send()
                    .await
                    .unwrap()
                    .error_for_status()
                    .unwrap();

                if !res.status().is_success() {
                    log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                    panic!("unable to assign module!"); 
                }

                let body: serde_json::Value = res.json().await.unwrap();
                let body:CreateResponse<ModuleId> = serde_json::from_value(body).unwrap();

                body.id
            }
        };
    }
}

async fn publish_jig(ctx:&Context, jig_id: &JigId) {

    log::info!("publishing {}...", jig_id.0.to_string());

    let path = endpoints::jig::Publish::PATH.replace("{id}", &jig_id.0.to_string());
    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    let res = ctx.client
        .put(&url)
        .header("Authorization", &format!("Bearer {}", ctx.token))
        .header("content-length", 0)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("unable to publish jig!"); 
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
