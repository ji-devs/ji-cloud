use reqwest::IntoUrl;
pub use serde::{Deserialize, Deserializer, de, serde_if_integer128};
pub use serde_repr::*;
pub use std::{
    path::{Path, PathBuf},
    fs::File,
    fmt,
    future::Future,
    convert::{TryFrom,TryInto},
    io::prelude::*,
    sync::Arc,
    collections::HashMap,
};

pub use components::stickers::video::ext::YoutubeUrlExt;
pub use scan_fmt::scan_fmt;
use crate::context::Context;
use crate::process::game_urls::GameJsonUrl;
use futures::stream::{FuturesUnordered, StreamExt};
use futures::lock::Mutex;

pub use migrate::{ 
    config::{REFERENCE_HEIGHT, REFERENCE_WIDTH},
    src_manifest::{
        MediaTranscode, 
        SrcManifest,
        SrcManifestData,
        Media,
        Slide as SrcSlide,
        ActivityKind as SrcActivityKind,
        Activity as SrcActivity,
        Shape as SrcShape,
        PathPoint as SrcPathPoint,
        PathElementKind as SrcPathElementKind,
        Layer as SrcLayer,
        LayerKind as SrcLayerKind,
        LoopKind as SrcLoopKind,
        ShowKind as SrcShowKind,
    }
};

pub use shared::domain::jig::{
    JigCreateRequest, 
    JigData, 
    JigPlayerSettings, 
    module::{
        ModuleCreateRequest, 
        ModuleBody, 
        body::{
            Transform,
            _groups::design::{PathCommand, TraceKind, TraceShape, YoutubeUrl},
            legacy::{
                ModuleData,
                slide::*,
                design::*,
                activity::*
            }
        }
    }
};
pub use reqwest::Client; 
pub use utils::{math::mat4::Matrix4, prelude::*};

type Medias = Arc<Mutex<Vec<Media>>>;

pub async fn run(ctx:Arc<Context>, urls: Vec<GameJsonUrl>, medias: Medias) {
    let loaded_manifests = Arc::new(Mutex::new(Vec::new()));
    _run(ctx.opts.transcode_json_batch_size, get_game_futures(ctx.clone(), urls, loaded_manifests.clone()).await).await;
    _run(ctx.opts.transcode_json_batch_size, get_slide_futures(ctx.clone(), loaded_manifests.lock().await.clone(), medias.clone()).await).await;
}

async fn _run(batch_size: usize, mut jobs: Vec<impl Future>) {

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

async fn get_game_futures(ctx:Arc<Context>, urls: Vec<GameJsonUrl>, loaded_manifests: Arc<Mutex<Vec<SrcManifest>>>) -> Vec<impl Future> {

    let mut futures = Vec::new();

    for g in urls {
        let game_json_url = g.url;
        let target_game_id = g.game_id;
        let games_dir = ctx.games_dir.join(&target_game_id);
        let json_dir = games_dir.join("json");
        let game_json_path = json_dir.join("game.json");

        futures.push({
            let ctx = ctx.clone();
            let loaded_manifests = loaded_manifests.clone();
            async move {

                let opts = &ctx.opts;
                let client = &ctx.client;

                if ctx.opts.transcode_game_json_skip_json_exists && game_json_path.exists() {
                    log::info!("skipping {} because the JSON exists", &target_game_id);
                    return;
                }

                log::info!("loading game data {} from {}", target_game_id, game_json_url);

                let mut write_file = true;

                let loaded = {
                    if ctx.opts.transcode_game_json_file_first && game_json_path.exists() {
                        match load_file(&ctx, game_json_path.clone()) {
                            Some(loaded) => {
                                write_file = false;
                                Some(loaded)
                            }
                            None => {
                                load_url(&ctx, &game_json_url).await
                            }
                        }
                    } else {
                        load_url(&ctx, &game_json_url).await
                    }
                };

                match loaded {
                    None => {
                        log::warn!("no game data for {} as {}", target_game_id, game_json_url);
                    },
                    Some(loaded) => {
                        let (src_manifest, raw_game_json) = loaded;
                        let game_id = src_manifest.game_id();
                        if !target_game_id.is_empty() && game_id != target_game_id {
                            panic!("game id {} != target_game_id {}", game_id, target_game_id);
                        }
                        if write_file {
                            std::fs::create_dir_all(&json_dir);
                            let mut file = File::create(&game_json_path).unwrap();
                            let mut cursor = std::io::Cursor::new(raw_game_json);

                            std::io::copy(&mut cursor, &mut file).unwrap();
                        }

                        log::info!("loaded manifest, game id: {}", game_id);

                        loaded_manifests.lock().await.push(src_manifest);
                    }
                }
            }
        });
    }

    futures
}


async fn get_slide_futures(ctx:Arc<Context>, loaded_manifests: Vec<SrcManifest>, medias: Medias) -> Vec<impl Future> {
    let mut futures = Vec::new();
    for src_manifest in loaded_manifests.into_iter() {

        let game_id = src_manifest.game_id();
        let games_dir = ctx.games_dir.join(&game_id);
        let json_dir = games_dir.join("json");
        let slides_dir = json_dir.join("slides");
        let base_url =  src_manifest.base_url.trim_matches('/').to_string();
        let max_slides = src_manifest.structure.slides.len();

        std::fs::create_dir_all(&slides_dir);

        //let slide_ids:Vec<String> = src_manifest.structure.slides.iter().map(|slide| slide.slide_id()).collect();

        for slide in src_manifest.structure.slides.into_iter() {
            futures.push({
                let game_id = game_id.clone(); 
                let base_url = base_url.clone();
                let ctx = ctx.clone(); 
                let slides_dir = slides_dir.clone();
                let medias = medias.clone();
                async move {
                    let slide_id = slide.slide_id();
                    let data = slide::convert(ctx, slide, game_id, base_url, medias.clone(), max_slides).await;

                    let slide_path = slides_dir.join(format!("{}.json", slide_id));

                    if let Ok(mut file) = File::create(&slide_path) {
                        serde_json::to_writer_pretty(file, &data).unwrap();
                    } else {
                        panic!("unable to create file at {}", slide_path.display().to_string());
                    }
                }
            });
        }
    }

    futures
}

fn text_to_manifest(ctx:&Context, text:String) -> Option<(SrcManifest, String)> {
    #[derive(Deserialize, Debug)]
    pub struct MinimalSrcManifestData {
        pub data: MinimalSrcManifest
    }
    #[derive(Deserialize, Debug)]
    pub struct MinimalSrcManifest {
        pub album_store: MinimalAlbumStore
    }
    #[derive(Deserialize, Debug)]
    pub struct MinimalAlbumStore {
        pub album: MinimalAlbum,
    }
    #[derive(Deserialize, Debug)]
    pub struct MinimalAlbum {
        #[serde(rename="pk")]
        pub key: usize,
    }

    impl MinimalSrcManifest {
        pub fn game_id(&self) -> String {
            format!("{}", self.album_store.album.key)
        }
    }

    let text = text
        .replace(r#""path": {}"#, r#""path": []"#)
        .replace(r#""originTransform": "null""#, r#""originTransform": [1,0,0,1,0,0]"#)
        .replace(r#""transform": "null""#, r#""transform": [1,0,0,1,0,0]"#);

    let manifest = if ctx.opts.transcode_data_url {
        serde_json::from_str::<SrcManifestData>(&text)
            .map(|resp| resp.data)
    } else {
        serde_json::from_str::<SrcManifest>(&text)
    };

    let game_id = match manifest.as_ref() {
        Ok(manifest) => manifest.game_id(),
        Err(err) => {
            let minimal = if ctx.opts.transcode_data_url {
                serde_json::from_str::<MinimalSrcManifestData>(&text)
                    .map(|resp| resp.data)
            } else {
                serde_json::from_str::<MinimalSrcManifest>(&text)
            };
            
            match minimal {
                Ok(m) => m.game_id(),
                Err(_) => "unknown".to_string()
            }
        }
    };

    if text.contains("\"path\": []") {
        writeln!(&ctx.warnings_log, "{} has empty path", game_id).unwrap();
    }

    match manifest {
        Ok(manifest) => Some((manifest, text)),
        Err(err) => {
            writeln!(&ctx.errors_log, "{} unable to parse manifest, error: {:?}", game_id, err).unwrap();
            if ctx.opts.transcode_panic_on_manifest_parse_error {
                panic!("{} unable to parse manifest, error: {:?}", game_id, err);
            } else {
                None
            }
        }
    }
}

pub fn load_file(ctx: &Context, path:PathBuf) -> Option<(SrcManifest, String)> {
    match std::fs::read_to_string(path) {
        Ok(text) => text_to_manifest(ctx, text),
        Err(_) => None
    }
}

async fn load_url(ctx: &Context, url:&str) -> Option<(SrcManifest, String)> {

    let text = match ctx.client.get(url).send().await {
        Err(_) => Err(()),
        Ok(resp) => {
            match resp.error_for_status() {
                Err(_) => Err(()),
                Ok(resp) => {
                    resp.text().await.map_err(|_| ())
                }
            }
        }
    };
    
    match text {
        Ok(text) => text_to_manifest(ctx, text),
        Err(_) => {

            writeln!(&ctx.errors_log, "unknown unable to load manifest raw text at {}", url).unwrap();
            if ctx.opts.transcode_panic_on_manifest_parse_error {
                panic!("unknown unable to load manifest raw text at {}", url);
            } else {
                return None
            }
        }
    }
}

mod slide {
    use super::*;

    async fn make_audio_media(ctx: &Context, game_id: &str, slide: &SrcSlide, base_url: &str, filename: &str, allowed_empty: bool, medias: Medias) -> Option<String> {
        if filename.is_empty() {
            None
        } else {

            let base_filename = match filename.rfind(".") {
                None => filename,
                Some(idx) => &filename[0..idx]
            };

            let slide_id = slide.slide_id(); 

            for ext in vec!["mp3", "aac", "wav", "aiff", "ac3", ""] {

                let dl_filename = if ext.is_empty() { base_filename.to_string() } else { format!("{}.{}", base_filename, ext) };

                let url = format!("{}/{}", base_url, dl_filename);

                log::info!("loading {}", url);

                let filename_dest = format!("{}.mp3", base_filename);

                if url_exists(ctx, &url).await {
                    let media = Media {
                        game_id: game_id.to_string(),
                        url, 
                        basepath: format!("slides/{}/activity", slide_id), 
                        filename: dl_filename,
                        transcode: Some((MediaTranscode::Audio, filename_dest.clone()))
                    };

                    medias.lock().await.push(media);

                    return Some(filename_dest)
                } else {
                    //log::info!("not found, trying next extension..");
                }

            }

            let url = format!("{}/{}", base_url, filename);
            let slide_id = slide.slide_id(); 
            // there were just so many missing, we are *always* allowing empty... but still leaving the param for debugging purposes
            if allowed_empty {
                writeln!(&ctx.warnings_log, "{} skipping url {}, filename {}... is 404 and but is allowed to be (slide id: {})", game_id, url, filename, slide_id).unwrap();
                log::warn!("{} skipping url {}, filename {}... is 404 and but is allowed to be (slide id: {})", game_id, url, filename, slide_id);
            } else {
                writeln!(&ctx.errors_log, "{} url {}, filename {} is 404 and not allowed to be (slide id: {})", game_id, url, filename, slide_id).unwrap();
                if ctx.opts.transcode_panic_on_404_error {
                    panic!("{} url {}, filename {} is 404 and not allowed to be (slide id: {})", game_id, url, filename, slide_id);
                } 
            }

            None
            

        }
    }


    async fn make_video_media(ctx: &Context, game_id: &str, slide: &SrcSlide, base_url: &str, filename: &str, allowed_empty: bool, medias: Medias) -> Option<String> {

        let slide_id = slide.slide_id(); 

        if filename.is_empty() {
            None
        } else {
            let filename = Path::new(&filename).file_name().unwrap().to_str().unwrap().to_string();
            let filename_dest = format!("{}.mp4", Path::new(&filename).file_stem().unwrap().to_str().unwrap().to_string());

            let url = format!("{}/video/{}", base_url, filename);

            match ctx
                .client
                .head(&url)
                .send()
                .await
                .unwrap()
                .error_for_status() {
                    Ok(_) => {

                        let media = Media {
                            game_id: game_id.to_string(),
                            url, 
                            basepath: format!("slides/{}/activity", slide_id), 
                            filename,
                            transcode: Some((MediaTranscode::Video, filename_dest.clone()))
                        };

                        medias.lock().await.push(media);

                        Some(filename_dest)
                    },

                    Err(_) => {
                        writeln!(&ctx.errors_log, "{} url {}, filename {} is 404 and not allowed to be (slide id: {})", game_id, url, filename, slide_id).unwrap();
                        if ctx.opts.transcode_panic_on_404_error {
                            panic!("{} url {}, filename {} is 404 and not allowed to be (slide id: {})", game_id, url, filename, slide_id);
                        } else {
                            None
                        }
                    }
                }
        }
    }
    pub async fn convert(ctx: Arc<Context>, slide: SrcSlide, game_id: String, base_url: String, medias: Medias, max_slides: usize) -> Slide {
        let client = &ctx.client;
        let opts = &ctx.opts;
        let slide_id = slide.slide_id(); 

        log::info!("parsing slide: {}", slide_id);

        let activities_len = slide.activities.len();
        let layers_len = slide.layers.len();

        if activities_len > 1 && slide.activity_kind != SrcActivityKind::Questions {
            log::error!("{:#?}", slide.activities);
            panic!("{} is more than one activity and not ask a question?!", slide.activities.len());
        }

        async fn add_image_media(
            ctx: Arc<Context>,
            medias: Medias,
            base_url: &str,
            slide_id: &str,
            game_id: &str,
            img: &str
        ) -> String {
            let filename = strip_path(&img).to_string();
            let url = format!("{}/{}/{}", base_url, slide_id, filename);
            if url_exists(&ctx, &url).await {
                medias.lock().await.push(
                    Media { 
                        game_id: game_id.to_string(),
                        url,
                        basepath: format!("slides/{}", slide_id), 
                        filename: filename.clone(),
                        transcode: None
                    }
                );
                filename
            } else {
                let filename = img.to_string();
                let url = format!("{}/{}", base_url, filename);
                if url_exists(&ctx, &url).await {
                    medias.lock().await.push(
                        Media { 
                            game_id: game_id.to_string(),
                            url,
                            basepath: format!("slides/{}", slide_id), 
                            filename: filename.clone(),
                            transcode: None
                        }
                    );
                } else {
                    log::warn!("{} (slide id {}) invalid top-level image {}", game_id, slide_id, img);
                    writeln!(&ctx.warnings_log, "{} (slide id {}) invalid top-level image {}", game_id, slide_id, img).unwrap();
                }
                filename
            }
        }

        let image_full = add_image_media(
            ctx.clone(),
            medias.clone(),
            &base_url,
            &slide_id,
            &game_id,
            &slide.image_full
        ).await;
        let image_thumb = match slide.image_thumb.as_ref() {
            None => None,
            Some(image_thumb) => Some(
                add_image_media(
                    ctx.clone(),
                    medias.clone(),
                    &base_url,
                    &slide_id,
                    &game_id,
                    &image_thumb
                ).await
            )
        };


        let validate_jump_index = |index: i128| -> Option<usize> {
            if index >= (max_slides as i128) || index < 0 {
                if opts.transcode_allow_bad_jump_index {
                    log::warn!("invalid jump index: {} (there are only {} slides!)", index, max_slides);
                    writeln!(&ctx.warnings_log, "{} invalid jump index: {} (there are only {} slides!)", game_id, index, max_slides,).unwrap();
                    None
                } else {
                    writeln!(&ctx.errors_log, "{} invalid jump index: {} (there are only {} slides!)", game_id, index, max_slides).unwrap();
                    panic!("{} invalid jump index: {} (there are only {} slides!)", game_id, index, max_slides);
                }
            } else {
                index.try_into().ok()
            }
        };

        let activity = {
            if activities_len == 0 {
                None
            } else {
                if slide.activity_kind == SrcActivityKind::Questions {
                    let mut items:Vec<QuestionItem> = Vec::with_capacity(activities_len);

                    for activity in slide.activities.clone().into_iter() {
                        let activity_settings = activity.settings.clone().unwrap_or_default();

                        if activity_settings.bg_audio.is_some() {
                            panic!("Ask a question shouldn't have bg audio set..");
                        }

                        if activity.shapes.len() > 1 {
                            panic!("Ask a question can't have more than one shape...");
                        } else if activity.shapes.is_empty() {
                            log::warn!("ask a question with no questions?? skipping...");
                        } else {
                            let shape = activity.shapes[0].clone();

                            if let Some(hotspot) = shape::convert_to_hotspot(&shape) {
                                let question_filename = match activity.intro_audio.as_ref() {
                                    Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &audio, true, medias.clone()).await,
                                    None => None
                                };

                                let answer_filename = match shape.audio.as_ref() {
                                    Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &audio, true, medias.clone()).await,
                                    None => None
                                };

                                let wrong_filename = match shape.audio_2.as_ref() {
                                    Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &audio, true, medias.clone()).await,
                                    None => None
                                };

                                items.push(QuestionItem{
                                    question_filename,
                                    answer_filename,
                                    wrong_filename,
                                    hotspot
                                });
                            } 
                        }

                    }

                    Some(Activity::AskQuestions(AskQuestions {
                        items
                    }))
                } else {
                    let activity = slide.activities[0].clone();
                    let activity_settings = activity.settings.clone().unwrap_or_default();

                    let audio_filename = match activity.intro_audio.as_ref() {
                        Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &audio, true, medias.clone()).await,
                        None => None
                    };

                    let bg_audio_filename = match activity_settings.bg_audio {
                        None => None,
                        Some(bg_audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &bg_audio, true, medias.clone()).await
                    };

                    match slide.activity_kind {
                        SrcActivityKind::SaySomething => {
                            Some(Activity::SaySomething(SaySomething {
                                audio_filename,
                                advance_trigger: if activity_settings.advance.unwrap_or_default() {
                                    AdvanceTrigger::AudioEnd
                                } else {
                                    AdvanceTrigger::Tap
                                },
                                advance_index: activity_settings.jump_index.and_then(validate_jump_index)
                            }))
                        },
                        SrcActivityKind::Soundboard => {

                            let mut items:Vec<SoundboardItem> = Vec::new();
                            let mut highlight_color:Option<String> = None;

                            for shape in activity.shapes.into_iter() {
                                if let Some(hotspot) = shape::convert_to_hotspot(&shape) {
                                    let shape_settings = shape.settings.clone().unwrap_or_default();

                                    match (highlight_color.as_ref(), shape_settings.highlight_color.as_ref()) {
                                        (Some(c1), Some(c2)) => {
                                            if c1 != c2.trim() {
                                                panic!("soundboard highlight colors changed between shapes: {} vs. {}", c1, c2);
                                            }
                                        },
                                        (None, Some(c)) => {
                                            log::info!("highlight color: {}", c);

                                            highlight_color = Some(c.trim().to_string());
                                        },
                                        _ => {}
                                    }


                                    items.push(SoundboardItem {
                                        audio_filename: match shape.audio.as_ref() {
                                            Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &audio, true, medias.clone()).await,
                                            None => None
                                        },
                                        text: map_text(&shape_settings.text),
                                        jump_index: shape_settings.jump_index.and_then(validate_jump_index),
                                        hotspot
                                    });
                                }
                            }

                            let one_at_a_time = match (activity_settings.fun_mode, activity_settings.fun_mode_v2) {
                                (Some(x1), Some(x2)) => {
                                    if x1 != x2 {
                                        panic!("soundmode and v2 set, but different!");
                                    }

                                    !x1
                                },
                                (Some(x), None) => {
                                    !x
                                },
                                (None, Some(x)) => {
                                    !x
                                },
                                (None, None) => {
                                    false
                                },
                            };

                            let show_hints = match activity_settings.hide_hints {
                                None => false,
                                Some(x) => !x
                            };

                            Some(Activity::Soundboard(Soundboard{
                                audio_filename,
                                bg_audio_filename,
                                highlight_color,
                                one_at_a_time,
                                show_hints,
                                items
                            }))
                        },
                        SrcActivityKind::Video => {
                            match activity_settings.video_url {
                                None => None,
                                Some(video_url) => {
                                    let transform_matrix = activity_settings.transform.map(convert_transform);
                                    let video_url = video_url.replace("http://", "https://");

                                    let src = match <YoutubeUrl as YoutubeUrlExt>::try_parse(video_url.clone()) {
                                        Ok(yt) => {
                                            log::info!("yt: {}", yt.get_id());
                                            Some(VideoSource::Youtube(yt))
                                        },
                                        Err(_) => {
                                            let filename = video_url.replace("local://", "");

                                            if filename.trim().is_empty() {
                                                writeln!(&ctx.errors_log, "{} empty video", game_id).unwrap();
                                                log::warn!("{} empty video", game_id);
                                                None
                                            } else {
                                                match slide::make_video_media(&ctx,&game_id, &slide, &base_url, &filename, false, medias.clone()).await {
                                                    None => {
                                                        panic!("{} unable to get url from {}", game_id, video_url);
                                                    },
                                                    Some(filename) => {
                                                        log::info!("not yt: {}", filename);
                                                        Some(VideoSource::Direct(filename))
                                                    }
                                                }
                                            }
                                        }
                                    };

                                    let range:Option<(f64, f64)> = activity_settings.video_range.and_then(|range_str| {
                                        //yes, really
                                        scan_fmt!(&range_str, "{{{}, {}}}", f64, f64).ok()
                                    });

                                    src.map(move |src| {
                                        Activity::Video(Video {
                                            transform_matrix,
                                            src,
                                            range
                                        })
                                    })
                                }
                            } 
                        },
                        SrcActivityKind::Puzzle => {
                            let mut items:Vec<PuzzleItem> = Vec::new();

                            for shape in activity.shapes.into_iter() {
                                if let Some(hotspot) = shape::convert_to_hotspot(&shape) {
                                    items.push(PuzzleItem {
                                        audio_filename: match shape.audio.as_ref() {
                                            Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, &audio, true, medias.clone()).await,
                                            None => None
                                        },
                                        hotspot
                                    });
                                }
                            }

                            fn map_theme(x:&Option<u8>) -> PuzzleTheme {
                                match x {
                                    None => PuzzleTheme::Regular,
                                    Some(x) => {
                                        if *x == 0 {
                                            PuzzleTheme::Extrude
                                        } else {
                                            PuzzleTheme::Regular
                                        }
                                    }
                                }
                            };

                            let theme = if map_theme(&activity_settings.theme) == PuzzleTheme::Extrude || activity_settings.theme_v2.unwrap_or_default() == true {
                                PuzzleTheme::Extrude
                            } else {
                                PuzzleTheme::Regular
                            };


                            Some(Activity::Puzzle(Puzzle {
                                audio_filename,
                                jump_index: activity_settings.jump_index.and_then(validate_jump_index),
                                //show_hints: activity.settings.tooltip.unwrap_or(false),
                                show_hints: !activity_settings.hints_disabled.unwrap_or(true),
                                fly_back_to_origin: !activity_settings.fun_mode.unwrap_or(false),
                                show_preview: activity_settings.show_shape.unwrap_or(false) || activity_settings.show_shape_v2.unwrap_or(false),
                                theme,
                                items
                            }))

                        },
                        SrcActivityKind::TalkType => {

                            let mut items:Vec<TalkTypeItem> = Vec::new();

                            for shape in activity.shapes.into_iter() {
                                if let Some(hotspot) = shape::convert_to_hotspot(&shape) {
                                    let shape_settings = shape.settings.clone().unwrap_or_default();

                                    items.push(TalkTypeItem {
                                        audio_filename: match shape.audio.as_ref() {
                                            Some(audio) => slide::make_audio_media(&ctx, &game_id, &slide, &base_url, audio, true, medias.clone()).await,
                                            None => None
                                        },
                                        texts: match shape_settings.text_answers.as_ref() {
                                            None => None,
                                            Some(answers) => {
                                                Some(answers.iter().map(|x| x.trim().to_string()).collect())
                                            }
                                        },
                                        input_language: shape_settings.text_input_language.clone(),
                                        answer_kind: if shape_settings.speaking_mode.unwrap_or(false) {
                                            TalkTypeAnswerKind::Audio
                                        } else {
                                            TalkTypeAnswerKind::Text
                                        },
                                        hotspot
                                    });
                                }
                            }
                            Some(Activity::TalkType(TalkType {
                                audio_filename,
                                jump_index: activity_settings.jump_index.and_then(validate_jump_index),
                                show_hints: activity_settings.tooltip.unwrap_or(false),
                                items
                            }))
                        },

                        _ => {
                            unimplemented!("unsupported activity!")
                        }
                    }
                }
            }
        };


        let design = convert_design(ctx.clone(), &game_id, &slide_id, &base_url, medias.clone(), slide.layers).await;

        Slide {
            image_full,
            activity,
            design,
        }
    }
}

fn map_text<T: AsRef<str>>(text: &Option<T>) -> Option<String> {
    text.as_ref().and_then(|text| {
        let text = text.as_ref();
        if text.is_empty() {
            None
        } else {
            Some(text.to_string())
        }
    })
}

async fn convert_design(ctx: Arc<Context>, game_id: &str, slide_id: &str, base_url: &str, medias: Medias, layers: Vec<SrcLayer>) -> Design {
    let mut stickers: Vec<Sticker> = Vec::new();
    let mut bgs:Vec<String> = Vec::new();
  
    for layer in layers {

        if let Some(filename) = layer.filename.as_ref() {
            if !filename.is_empty() {
                medias.lock().await.push(
                    Media { 
                        game_id: game_id.to_string(),
                        url: format!("{}/{}/layers/{}", base_url, slide_id, filename), 
                        basepath: format!("slides/{}", slide_id), 
                        filename: filename.to_string(),
                        transcode: None
                    }
                );
            }
        }


        let audio_filename = match layer.audio.as_ref() {
            None => None,
            Some(audio) => {
                if audio.is_empty() {
                    None
                } else {
                    let mut ret = None;
                    for ext in vec!["mp3", "aac", "wav", "aiff", "ac3", ""] {
                        let filename_dest = format!("{}.{}", Path::new(&audio).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_string(), ext);
                        let url = format!("{}/{}/layers/{}", base_url, slide_id, audio);

                        if url_exists(&ctx, url).await {
                            medias.lock().await.push(Media { 
                                game_id: game_id.to_string(),
                                url: format!("{}/{}/layers/{}", base_url, slide_id, audio), 
                                basepath: format!("slides/{}", slide_id), 
                                filename: audio.to_string(),
                                transcode: Some((MediaTranscode::Audio, filename_dest.clone()))
                            });

                            ret = Some(filename_dest);
                            break;
                        }

                    }

                    ret
                }
            }
        };

        match layer.kind {
            SrcLayerKind::Background => {
                bgs.push(layer.filename.unwrap());
            },
            SrcLayerKind::Text | SrcLayerKind::Image | SrcLayerKind::Animation => {
                let filename = match layer.filename.as_ref() {
                    Some(f) => Some(f.as_ref()),
                    None => {
                        match layer.kind {
                            SrcLayerKind::Text => {
                                match layer.html.as_ref() {
                                    Some(html) => {
                                        if html.is_empty() {
                                            None
                                        } else {
                                            Some("")
                                        }
                                    },
                                    None => None
                                }
                            },
                            _ => None
                        }
                    }
                };
                if let Some(filename) = filename {
                    let sticker = Sticker { 
                        filename: filename.to_string(),
                        transform_matrix: match layer.transform {
                            Some(transform) => convert_transform(transform),
                            None => convert_transform([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
                        },
                        hide: match layer.show_kind.unwrap_or_default() {
                            SrcShowKind::ShowOnLoad => false, 
                            SrcShowKind::HideOnTap => false, 
                            SrcShowKind::ShowOnTap => true, 
                        },

                        hide_toggle: match layer.show_kind.unwrap_or_default() {
                            SrcShowKind::ShowOnLoad => None, 
                            _ => Some(
                                if layer.toggle_show.unwrap_or_default() {
                                    HideToggle::Always
                                } else {
                                    HideToggle::Once
                                }
                            ), 
                        },

                        animation: {
                            if layer.kind == SrcLayerKind::Animation {
                                Some(
                                    match layer.loop_kind.unwrap_or_default() {
                                        SrcLoopKind::PlayOnLoadLoop => Animation {
                                            once: false,
                                            tap: false 
                                        },
                                        SrcLoopKind::PlayOnLoadOnce => Animation {
                                            once: true,
                                            tap: false 
                                        },
                                        SrcLoopKind::PlayOnTapLoop => Animation {
                                            once: false,
                                            tap: true 
                                        },
                                        SrcLoopKind::PlayOnTapOnce => Animation {
                                            once: true,
                                            tap: true 
                                        },
                                    }
                                )
                            } else {
                                None
                            }
                        },

                        override_size: {
                            // not really needed unless it differs from the real file size
                            // but whatever...
                            match (layer.width, layer.height) {
                                (Some(width), Some(height)) => Some((width, height)),
                                _ => None
                            }
                        },

                        audio_filename,
                        
                        kind: layer.kind.convert(&layer)

                    };

                    stickers.push(sticker);
                } else {
                    log::warn!("expected filename for layer kind {:?}, game_id: {}, slide_id: {}", &layer.kind, &game_id, &slide_id);
                }
            },
        }
    }
    Design {
        bgs,
        stickers
    }
}

fn convert_transform(orig: [f64;6]) -> [f64;16] {

    let scale_x = orig[0];
    let scale_y = orig[3];
    let skew_x = orig[2];
    let skew_y = orig[1];
    let translate_x = orig[4] / REFERENCE_WIDTH;
    let translate_y = orig[5] / REFERENCE_HEIGHT;

    let mut m = Matrix4::identity();

    m.translate(&[translate_x, translate_y, 0.0]);
    m.scale(&[scale_x, scale_y, 1.0]);
    m.skew_x(skew_x);
    m.skew_y(skew_y);


    m.values()
}

fn transform_is_identity(orig: [f64;6]) -> bool {
    orig == [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

mod shape {

    use super::*;

    pub fn convert_to_hotspot(shape: &SrcShape) -> Option<Hotspot> {
        if shape.path.len() > 1 {
            Some(Hotspot {
                shape: TraceShape::PathCommands(
                shape 
                        .path
                        .iter()
                        .map(|point| (convert_point(point), true))
                        .collect()
                ),
                transform_matrix: shape.settings
                    .as_ref()
                    .and_then(|settings| {
                        settings.transform.and_then(|t| {
                            if !transform_is_identity(t) {
                                Some(convert_transform(t))
                            } else {
                                None
                            }
                        })
                    })
            })
        } else {
            log::warn!("empty path data in shape, skipping...");
            None
        }
    }

    pub fn convert_point(point: &SrcPathPoint) -> PathCommand {
        let SrcPathPoint { mut x, mut y, mut cp1x, mut cp1y, mut cp2x, mut cp2y, kind} = point;

        x /= REFERENCE_WIDTH;
        y /= REFERENCE_HEIGHT;
        cp1x /= REFERENCE_WIDTH;
        cp1y /= REFERENCE_HEIGHT;
        cp2x /= REFERENCE_WIDTH;
        cp2y /= REFERENCE_HEIGHT;

        match kind {
            SrcPathElementKind::MoveToPoint => PathCommand::MoveTo(x, y),
            SrcPathElementKind::AddLineToPoint => PathCommand::LineTo(x, y),
            SrcPathElementKind::AddQuadCurveToPoint => PathCommand::QuadCurveTo(cp1x, cp1y, x, y),
            SrcPathElementKind::AddCurveToPoint => PathCommand::CurveTo(cp1x, cp1y, cp2x, cp2y, x, y),
            SrcPathElementKind::CloseSubPath => PathCommand::ClosePath,
        }
    }
}

async fn url_exists(ctx: &Context, url: impl AsRef<str>) -> bool {
    match ctx
        .client
        .head(url.as_ref())
        .send()
        .await
        .unwrap()
        .error_for_status() {
            Ok(_) => {
                true
            },
            Err(_) => {
                log::info!("no valid url at {}", url.as_ref()); 
                false
            }
        }
}
fn strip_path(orig:&str) -> &str {
    match orig.rsplit_once("/") {
        None => orig,
        Some((_, end)) => end 
    }
}
