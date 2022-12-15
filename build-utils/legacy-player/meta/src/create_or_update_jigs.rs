
use crate::report::GameJigInfo;

use super::context::Context;
use std::sync::Arc;
use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use shared::domain::asset::AssetId;
use shared::domain::module::{ModuleKind, Module};
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use uuid::Uuid;
use std::process::Command;
use reqwest::Client; 
use serde::{Serialize, Deserialize};
use serde_json::{Result, value::RawValue};
use futures::stream::{FuturesUnordered, StreamExt};
use futures::lock::Mutex;
use std::collections::{HashMap, HashSet};
use shared::{
    api::{
        ApiEndpoint,
        PathParts,
        endpoints,
    },
    domain::{
        meta::{AffiliationId, AgeRangeId, MetadataResponse},
        jig::{JigResponse, JigId, JigCreateRequest, JigPlayerSettings, JigUpdateDraftDataRequest, AudioBackground},
        module::{ModuleResponse, ModuleBody, ModuleCreateRequest, ModuleDeleteRequest, ModuleId, body::legacy::ModuleData},
        asset::PrivacyLevel,
        CreateResponse
    }
};
use csv::Writer;
use crate::src_manifest::{SrcManifest, SrcManifestData, load_manifest, Slide as SrcSlide};

pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();

    // doesn't need to be done every time, just to be sure
    // let games_on_disk = crate::report::get_games_on_disk(&ctx);
    // sanity_check(&ctx, &games_on_disk);

    let (games_to_update, games_to_create) = get_lists(&ctx);

    log::info!("{} games to update, {} games to create", games_to_update.len(), games_to_create.len());

    _run(ctx.clone(), get_jig_update_futures(ctx.clone(), games_to_update).await).await;
    _run(ctx.clone(), get_jig_create_futures(ctx.clone(), games_to_create).await).await;

    if !ctx.opts.dry_run {
        let mut wtr = Writer::from_writer(vec![]);

        for record in ctx.records.lock().unwrap().iter() {
            wtr.serialize(&record).unwrap();
        }
        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

        let mut file = File::create(&ctx.opts.output_csv_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    log::info!("{} jigs updated, {} jigs created", ctx.stats.jig_update_count(), ctx.stats.jig_create_count());
}


async fn _run(ctx: Arc<Context>, mut jobs: Vec<impl Future>) {
    let batch_size = ctx.opts.download_jigs_batch_size;

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

}

async fn get_jig_update_futures(ctx: Arc<Context>, games_to_update: Vec<GameJigInfo>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    for game_to_update in games_to_update {
        futures.push({
            let ctx = ctx.clone();
            async move {
                // let manifest = load_manifest(&ctx, &game_to_update.game_id).await;
                //delete_jig_modules(&ctx, &game_to_update.jig_id).await;

                // let jig_id = JigId(Uuid::parse_str(&game_to_update.jig_id).unwrap());
                // assign_modules(&ctx, &game_to_update.game_id, &jig_id, &manifest).await;
    
                // publish_jig(&ctx, &jig_id).await;

                // do nothing other than set the record straight
                let index = ctx.records.lock().unwrap().iter().position(|r| r.game_id == game_to_update.game_id).unwrap();
                ctx.records.lock().unwrap()[index].jig_new = "NO".to_string();
                ctx.records.lock().unwrap()[index].jig_id = game_to_update.jig_id;
                ctx.stats.jig_update_increase();
            }
        });
    }

    futures
}

async fn get_jig_create_futures(ctx: Arc<Context>, games_to_update: Vec<String>) -> Vec<impl Future> {
    let mut futures = Vec::new();

    let local_meta = LocalMeta::load(&ctx).await;

    for game_id in games_to_update {
        futures.push({
            let ctx = ctx.clone();
            let local_meta = local_meta.clone();
            async move {
                let manifest = load_manifest(&ctx, &game_id).await;
                let jig_id = create_jig(&ctx, &local_meta, &manifest).await;
                let index = ctx.records.lock().unwrap().iter().position(|r| r.game_id == game_id).unwrap();
                ctx.records.lock().unwrap()[index].jig_new = "YES".to_string();
                ctx.records.lock().unwrap()[index].jig_id = jig_id; 
                ctx.stats.jig_create_increase();
            }
        });
    }

    futures
}

fn get_lists(ctx: &Context) -> (Vec<GameJigInfo>, Vec<String>) {
    let mut games_to_update = Vec::new();
    let mut games_to_create = Vec::new();

    let games_in_jigs = crate::report::get_games_in_jigs(&ctx);
    let games_in_records:Vec<(String, String)> = ctx.records.lock().unwrap().iter().map(|record| {
        let mut jig_id = record.jig_id.clone();
        if record.jig_new.to_uppercase() != "YES" {
            jig_id = "".to_string();
        }
        (record.game_id.clone(), jig_id)
    }).collect();

    for (game_id, jig_id) in games_in_records {
        if !jig_id.is_empty() {
            games_to_update.push(GameJigInfo { game_id, jig_id });
        } else if let Some(x) = games_in_jigs.iter().find(|x| x.game_id == game_id) {
            games_to_update.push(x.clone());
        } else {
            games_to_create.push(game_id);
        }
    }
    (games_to_update, games_to_create)
}

fn sanity_check(ctx: &Context, games_on_disk: &[String]) {
    let games_in_records:Vec<String> = ctx.records.lock().unwrap().iter().map(|record| record.game_id.clone()).collect();

    for game_on_disk in games_on_disk {
        if !games_in_records.contains(game_on_disk) {
            panic!("game {} on disk but not in record!", game_on_disk);
        }
    }

    for game_in_record in games_in_records {
        if !games_on_disk.contains(&game_in_record) {
            panic!("game {} in record but but not on disk!", game_in_record);
        }
    }
}


//////////// CREATE JIG //////////////////////

#[derive(Clone)]
pub struct LocalMeta {
    pub affiliations: Vec<AffiliationId>,
    pub age_ranges: Vec<AgeRangeId>,
}

impl LocalMeta {
    async fn load(ctx:&Context) -> LocalMeta {

        let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), <endpoints::meta::Get as ApiEndpoint>::Path::PATH);

        let res = reqwest::Client::new()
            .get(url)
            .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
            panic!("Failed to get meta data");
        }

        let MetadataResponse { affiliations, age_ranges, .. } = res.json().await.unwrap();

        let affiliations = affiliations
            .iter()
            .map(|x| x.id)
            .collect();

        let age_ranges = age_ranges
            .iter()
            .map(|x| x.id)
            .collect();

        LocalMeta { age_ranges, affiliations }
    }
}

pub async fn create_jig(ctx: &Context, local_meta: &LocalMeta, manifest: &SrcManifest) -> String {
    log::info!("creating jig for game_id: {}", manifest.game_id());

    // let image_id = if !ctx.opts.update_jigs_skip_cover_page && manifest.structure.slides.len() > 0 {
    //     Some(upload_cover_image(ctx, &manifest.game_id(), &manifest.structure.slides[0]).await)
    // } else {
    //     None
    // };
    
    let jig_id = make_jig(ctx, local_meta, manifest).await;
    log::info!("got jig id: {}", jig_id.0.to_string());
    assign_modules(ctx, &manifest.game_id(), &jig_id, manifest).await;

    if !ctx.opts.dry_run {
        // if let Some(image_id) = image_id {
        //     log::info!("setting cover");
        //     assign_cover_image(ctx, &jig_id, image_id).await;
        // }
    }
        
    publish_jig(ctx, &jig_id).await;

    jig_id.0.to_string()
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
        language: manifest.lang_str().to_string(),
        categories: Vec::new(), 
        description: format!("{} {}", 
            manifest.album_store.album.fields.description.clone().unwrap_or_default(),
            author_byline
        ),
        default_player_settings: JigPlayerSettings::default(),
    };

    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), <endpoints::jig::Create as ApiEndpoint>::Path::PATH);

    let jig_id = if(ctx.opts.dry_run) {
        log::info!("CREATE JIG URL: {}", url);

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
    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        <endpoints::jig::UpdateDraftData as ApiEndpoint>::Path::PATH
            .replace("{jig_id}", &jig_id.0.to_string())
    );

    let mut req = JigUpdateDraftDataRequest {
        privacy_level: if manifest.album_store.public.unwrap_or(true) {
            Some(PrivacyLevel::Public)
        } else {
            Some(PrivacyLevel::Unlisted)
        },
        ..Default::default()
    };

    let bg_audio = {
        if let Some(music_file) = manifest.structure.music_file.as_ref() {
            let name = music_file.to_lowercase();
            if name.contains("halalu") {
                Some(AudioBackground::LegacyHanerotHalalu)
            } else if name.contains("jitap") {
                Some(AudioBackground::LegacyJiTap)
            } else if name.contains("maoz") {
                Some(AudioBackground::LegacyMaozTzur)
            } else if name.contains("modeh") {
                Some(AudioBackground::LegacyModehAni)
            } else if name.contains("hechiyanu") || name.contains("cheyanu") {
                Some(AudioBackground::LegacyShehechiyanu)
            } else if name.contains("zoo") {
                Some(AudioBackground::LegacyMorningZoo)
            } else if name.contains("march") {
                Some(AudioBackground::LegacyPlaylandMarch)
            } else if name.contains("walrus") {
                Some(AudioBackground::LegacyWanderingWalrus)
            } else if name.contains("romp") {
                Some(AudioBackground::LegacyIslandRomp)
            } else if name.contains("bars") {
                Some(AudioBackground::LegacyMonkeyBars)
            } else if name.contains("clouds") {
                Some(AudioBackground::LegacySunAndNoClouds)
            } else if name.contains("etude") {
                Some(AudioBackground::LegacyFirstEtude)
            } else if name.contains("bear") {
                Some(AudioBackground::LegacyTeddysBear)
            } else if name.contains("time") {
                Some(AudioBackground::LegacyNapTime)
            } else if name.contains("lullaby") {
                Some(AudioBackground::LegacyWindupLullaby)
            } else if name.contains("cuckoo") || name.contains("show") {
                Some(AudioBackground::LegacyCuckooToYou)
            } else if name.is_empty() || name.contains("silence") {
                None
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(bg_audio) = bg_audio {
        req.audio_background = Some(Some(bg_audio));
    }


    if ctx.opts.dry_run {
        log::info!("UPDATE JIG URL: {}", url);
    } else {
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

async fn delete_jig_modules(ctx:&Context, jig_id_str: &str) {
    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        <endpoints::jig::GetLive as ApiEndpoint>::Path::PATH
            .replace("{jig_id}", &jig_id_str)
    );

    let res = ctx.client
        .get(&url)
        .header("Authorization", &format!("Bearer {}", ctx.opts.token))
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
    let body:JigResponse = serde_json::from_value(body).unwrap();

    let jig_id = JigId(Uuid::parse_str(jig_id_str).unwrap());
    let parent_id: AssetId = jig_id.into();
    for module in body.jig_data.modules {
        let module_id_str = module.id.0.to_string();
        let url = format!("{}{}", 
            ctx.opts.get_remote_target().api_url(), 
            <endpoints::module::Delete as ApiEndpoint>::Path::PATH
                .replace("{module_id}", &module_id_str)
        );

        let req = ModuleDeleteRequest {
            parent_id
        };

        log::info!("deleting module {} in jig {}", module_id_str, jig_id_str);
        if(ctx.opts.dry_run) {
            log::info!("DELETE MODULE URL: {}", url);
        } else {
            let res = ctx.client
                .delete(&url)
                .header("Authorization", &format!("Bearer {}", ctx.opts.token))
                .json(&req)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap();

            if !res.status().is_success() {
                log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
                panic!("unable to delete module!"); 
            }

        }
    }
}
async fn assign_modules(ctx:&Context, game_id: &str, jig_id: &JigId, manifest: &SrcManifest) {

    for (index, slide) in manifest.structure.slides.iter().enumerate() {
        let req = ModuleCreateRequest {
            parent_id: (*jig_id).into(),
            body: ModuleBody::Legacy(
                ModuleData {
                    game_id: game_id.to_string(),
                    slide_id: slide.slide_id()
                },
            )
        };

        let url = format!("{}{}", 
            ctx.opts.get_remote_target().api_url(), 
            <endpoints::module::Create as ApiEndpoint>::Path::PATH
                .replace("{jig_id}", &jig_id.0.to_string())
        );

        log::info!("creating module for slide #{} in jig {}", index+1, jig_id.0.to_string());

        let module_id = {
            if(ctx.opts.dry_run) {
                log::info!("CREATE MODULE URL: {}", url);
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

    let url = format!("{}{}", 
        ctx.opts.get_remote_target().api_url(), 
        <endpoints::jig::Publish as ApiEndpoint>::Path::PATH
            .replace("{jig_id}", &jig_id.0.to_string())
    );

    if ctx.opts.dry_run {
        log::info!("PUBLISH URL: {}", url);
    } else {
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
}


// async fn upload_cover_image(ctx:&Context, game_id: &str, slide: &SrcSlide) -> ImageId {
//     //get file info

//     let url = format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/transcode/games/{}/media/slides/{}", game_id, slide.image_full);

//     let res = ctx 
//         .client
//         .get(&url)
//         .send()
//         .await
//         .unwrap();

//     let data = res.bytes().await.unwrap();
//     let file_size = data.len();

//     let content_type = {
//         if slide.image_full.contains(".png") {
//             "image/png"
//         } else if slide.image_full.contains(".jpg") {
//             "image/jpeg"
//         } else if slide.image_full.contains(".gif") {
//             "image/gif"
//         } else if slide.image_full.contains(".svg") {
//             "image/svg+xml"
//         } else {
//             panic!("unknown content type!");
//         }
//     };
//     //get image id

//     let url = format!("{}{}", 
//         ctx.opts.get_remote_target().api_url(), 
//         endpoints::image::user::Create::PATH
//     );

//     let req_data = UserImageCreateRequest {
//         kind: ImageKind::Canvas,
//     };


//     let image_id = if ctx.opts.dry_run {
//         ImageId(Uuid::nil())
//     } else {

//         let res = ctx 
//             .client
//             .post(url)
//             .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
//             .json(&req_data)
//             .send()
//             .await
//             .unwrap();

//             //.json::<CreateResponse<ImageId>>()
//         if !res.status().is_success() {
//             log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//             panic!("Failed to get CreateResponse!");
//         }

//         let CreateResponse { id } = res.json().await.unwrap();

//         id
//     };

//     //get upload url
        
//     let req_data = UserImageUploadRequest {
//         file_size: file_size.try_into().unwrap()
//     };

//     let url = format!("{}{}", 
//         ctx.opts.get_remote_target().api_url(), 
//         endpoints::image::user::Upload::PATH.replace("{id}", &image_id.0.to_string())
//     );

//     let session_uri = if ctx.opts.dry_run {
//         "https://example.com".to_string()
//     } else {

//         let res = ctx 
//             .client
//             .put(url)
//             .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
//             .json(&req_data)
//             .send()
//             .await
//             .unwrap();
        
//         if !res.status().is_success() {
//             log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//             panic!("Failed to get UserImageUploadResponse!");
//         }
//         let UserImageUploadResponse { session_uri } = res.json().await.unwrap();

//         session_uri
//     };

//     //upload it

//     let body:Body = data.into(); 

//     if ctx.opts.dry_run {
//         image_id
//     } else {
//         let res = ctx 
//             .client
//             .put(&session_uri)
//             .header("Content-Type", content_type)
//             .header("Content-Length", file_size) 
//             .body(body)
//             .send()
//             .await
//             .unwrap();


//         if !res.status().is_success() {
//             log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//             panic!("Failed to upload image to storage!");
//         }

//         image_id
//     }
// }

// async fn assign_cover_image(ctx:&Context, jig_id: &JigId, image_id: ImageId) {

//     //get jig data 
        
//     let url = format!("{}{}", 
//         ctx.opts.get_remote_target().api_url(), 
//         endpoints::jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string())
//     );


//     let res = ctx 
//         .client
//         .get(url)
//         .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
//         .send()
//         .await
//         .unwrap();
    
//     if !res.status().is_success() {
//         log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//         panic!("Failed to get jig data");
//     }

//     let JigResponse { jig_data, .. } = res.json().await.unwrap();

//     // get cover data
//     let lite_module = jig_data.modules.iter().find(|m| m.kind == ModuleKind::Cover).unwrap();

//     let url = format!("{}{}", 
//         ctx.opts.get_remote_target().api_url(), 
//         endpoints::module::GetDraft::PATH
//             .replace("{id}", &jig_id.0.to_string())
//             .replace("{module_id}", &lite_module.id.0.to_string())
//     );


//     let res = ctx 
//         .client
//         .get(url)
//         .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
//         .send()
//         .await
//         .unwrap();
    
//     if !res.status().is_success() {
//         log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//         panic!("Failed to get jig data");
//     }

//     let ModuleResponse { module } = res.json().await.unwrap();

//     // mutate the data
//     let mut body = match module.body {
//         ModuleBody::Cover(body) => body,
//         _ => panic!("couldn't get module body!")
//     };

//     match body.content.as_mut() {
//         Some(content) => {
//             content.base.backgrounds.layer_1 = Some(Background::Image(Image {
//                 id: image_id,
//                 lib: MediaLibrary::User
//             }))
//         },
//         None => {
//             panic!("couldn't get body content!");
//         }
//     }

//     // Upload the new module data
 
//     let req_data = ModuleUpdateRequest {
//         id: StableOrUniqueId::Unique(lite_module.id),
//         body: Some(ModuleBody::Cover(body)),
//         index: None,
//         is_complete: Some(true)
//     };

//     let url = format!("{}{}", 
//         ctx.opts.get_remote_target().api_url(), 
//         endpoints::module::Update::PATH.replace("{id}", &jig_id.0.to_string())
//     );

//     let res = ctx
//         .client 
//         .patch(url)
//         .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
//         .json(&req_data)
//         .send()
//         .await
//         .unwrap();
        
//     if !res.status().is_success() {
//         log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//         panic!("Failed to update module!");
//     }

//     // indicate that the jig has a cover

//     let url = format!("{}{}", 
//         ctx.opts.get_remote_target().api_url(), 
//         endpoints::jig::Cover::PATH.replace("{id}", &jig_id.0.to_string())
//     );

//     let res = ctx 
//         .client
//         .patch(url)
//         .header("AUTHORIZATION", &format!("Bearer {}", &ctx.opts.token))
//         .send()
//         .await
//         .unwrap();
        
//     if !res.status().is_success() {
//         log::error!("error code: {}, details: {:?}", res.status().as_str(), res);
//         panic!("Unable to indicate that jig has cover!");
//     }
// }