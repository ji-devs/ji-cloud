#![allow(warnings)]

use std::{future::Future, path::PathBuf, vec};
use components::module::_common::prelude::Image;
use dotenv::dotenv;
use shared::{domain::{image::user::UserImageUploadResponse, jig::{module::{body::Background, ModuleUpdateRequest}, PrivacyLevel}, meta::{AgeRangeId, AffiliationId}}, media::MediaLibrary, config::RemoteTarget};
use tokio_util::codec::{BytesCodec, FramedRead};
use reqwest::Body;
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, atomic::Ordering};
use std::convert::TryInto;
use uuid::Uuid;
use ::migrate::{
    src_manifest::*,
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
use crate::context::Context;


pub struct LocalMeta {
    pub affiliations: Vec<AffiliationId>,
    pub age_ranges: Vec<AgeRangeId>,
}

pub async fn run(ctx: &Context, local_meta: &LocalMeta, manifest: &SrcManifest) -> String {
    log::info!("creating jig for game_id: {}", manifest.game_id());

    let image_id = if !ctx.opts.update_jigs_skip_cover_page && manifest.structure.slides.len() > 0 {
        Some(upload_cover_image(ctx, &manifest.game_id(), &manifest.structure.slides[0]).await)
    } else {
        None
    };
    
    let jig_id = make_jig(ctx, local_meta, manifest).await;
    log::info!("got jig id: {}", jig_id.0.to_string());
    assign_modules(ctx, &manifest.game_id(), &jig_id, manifest).await;

    if !ctx.opts.dry_run {
        if let Some(image_id) = image_id {
            log::info!("setting cover");
            assign_cover_image(ctx, &jig_id, image_id).await;
        }

        publish_jig(ctx, &jig_id).await;
    }

    jig_id.0.to_string()
}

async fn upload_cover_image(ctx:&Context, game_id: &str, slide: &migrate::src_manifest::Slide) -> ImageId {
    //get file info

    let url = format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/transcode/games/{}/media/slides/{}", game_id, slide.image_full);

    let res = ctx 
        .client
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

        let res = ctx 
            .client
            .post(url)
            .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
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

        let res = ctx 
            .client
            .put(url)
            .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
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
        let res = ctx 
            .client
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


    let res = ctx 
        .client
        .get(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
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


    let res = ctx 
        .client
        .get(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
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

    let res = ctx
        .client 
        .patch(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
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

    let res = ctx 
        .client
        .patch(url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
        .send()
        .await
        .unwrap();
        
    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Unable to indicate that jig has cover!");
    }
}

async fn make_jig(ctx:&Context, local_meta: &LocalMeta, manifest: &SrcManifest) -> JigId {

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
        age_ranges: local_meta.age_ranges.clone(), 
        affiliations: local_meta.affiliations.clone(), 
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
            .header("Authorization", &format!("Bearer {}", ctx.opts.token))
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
        privacy_level: if manifest.album_store.public.unwrap_or(true) {
            Some(PrivacyLevel::Public)
        } else {
            Some(PrivacyLevel::Private)
        },
        ..Default::default()
    };

    if !ctx.opts.dry_run {
        let res = ctx.client
            .patch(&url)
            .header("Authorization", &format!("Bearer {}", ctx.opts.token))
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
                    .header("Authorization", &format!("Bearer {}", ctx.opts.token))
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
        .header("Authorization", &format!("Bearer {}", ctx.opts.token))
        .header("content-length", 0)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("unable to publish jig!"); 
    }
}

async fn delete_all_jigs_first(ctx:&Context) {

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        endpoints::jig::DeleteAll::PATH
    );

    log::info!("url: {}", url);

    let res = ctx
        .client 
        .delete(&url)
        .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
        .header("content-length", 0)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
        panic!("Failed to delete all jigs");
    }
}
