pub use serde::{Deserialize, Deserializer, de, serde_if_integer128};
pub use serde_repr::*;
pub use std::{
    path::{Path, PathBuf},
    fs::File,
    fmt,
    future::Future,
    convert::TryFrom,
    io::prelude::*
};

pub use components::stickers::video::ext::{YoutubeUrlExt};
pub use scan_fmt::scan_fmt;
use crate::context::Context;

pub use super::options::*;

pub use transcode::{ 
    config::{REFERENCE_HEIGHT, REFERENCE_WIDTH},
    src_manifest::{
        MediaTranscode, 
        SrcManifest,
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

pub fn load_file(path:PathBuf) -> SrcManifest {
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).unwrap()
}

pub async fn load_url(ctx: &Context, url:&str) -> Option<(SrcManifest, String)> {

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

    let text = ctx 
        .client
        .get(url)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .await
        .unwrap();
    
    let manifest = serde_json::from_str::<SrcManifest>(&text);

    match manifest {
        Ok(manifest) => Some((manifest, text)),
        Err(err) => {
            let game_id = match serde_json::from_str::<MinimalSrcManifest>(&text) {
                Ok(m) => m.game_id(),
                Err(_) => "unknown".to_string()
            };

            writeln!(&ctx.errors_log, "{} unable to parse manifest at {}, error: {:?}", game_id, url, err).unwrap();
            if !ctx.opts.keep_going_if_manifest_parse_error {
                panic!("{} unable to parse manifest at {}, error: {:?}", game_id, url, err);
            } else {
                None
            }
        }
    }

}

pub async fn into_slides(ctx: &Context, manifest: SrcManifest, game_url: &str) -> (Vec<Slide>, Vec<Media>) {
    let mut medias:Vec<Media> = Vec::new();

    let game_id = manifest.game_id();
    let base_url =  manifest.base_url.trim_matches('/').to_string();

    let mut slides:Vec<Slide> = Vec::new();
    
    let max_slides = manifest.structure.slides.len();

    for slide in manifest.structure.slides.into_iter() {
        slides.push(slide::convert(&ctx, slide, &game_url, &game_id, &base_url, &mut medias, max_slides).await);
    }
        

    (slides, medias)
}

mod slide {
    use super::*;

    async fn make_audio_media(ctx: &Context, game_id: &str, game_url: &str, slide: &SrcSlide, base_url: &str, filename: &str, allowed_empty: bool, mut medias: &mut Vec<Media>) -> Option<String> {

        let slide_id = slide.slide_id(); 

        if filename.is_empty() {
            None
        } else {

            let url = format!("{}/{}", base_url, filename);

            let filename = Path::new(&filename).file_name().unwrap().to_str().unwrap().to_string();
            let filename_dest = format!("{}.mp3", Path::new(&filename).file_stem().unwrap().to_str().unwrap().to_string());

            match ctx
                .client
                .head(&url)
                .send()
                .await
                .unwrap()
                .error_for_status() {
                    Ok(_) => {

                        let media = Media {
                            url, 
                            basepath: format!("slides/{}/activity", slide_id), 
                            filename,
                            transcode: Some((MediaTranscode::Audio, filename_dest.clone()))
                        };

                        medias.push(media);

                        Some(filename_dest)
                    },
                    Err(_) => {
                        if allowed_empty {
                            writeln!(&ctx.warnings_log, "{} skipping url {}, filename {}... is 404 and but is allowed to be (slide id: {}, game_url: {})", game_id, url, filename, slide_id, game_url).unwrap();
                            log::warn!("{} skipping url {}, filename {}... is 404 and but is allowed to be (slide id: {}, game_url: {})", game_id, url, filename, slide_id, game_url);
                            None
                        } else {
                            writeln!(&ctx.errors_log, "{} url {}, filename {} is 404 and not allowed to be (slide id: {}, game_url: {})", game_id, url, filename, slide_id, game_url).unwrap();
                            if ctx.opts.panic_on_404_error {
                                panic!("{} url {}, filename {} is 404 and not allowed to be (slide id: {}, game_url: {})", game_id, url, filename, slide_id, game_url);
                            } else {
                                None
                            }
                        }
                    }
                }
        }
    }


    async fn make_video_media(ctx: &Context, game_id: &str, game_url: &str, slide: &SrcSlide, base_url: &str, filename: &str, allowed_empty: bool, mut medias: &mut Vec<Media>) -> Option<String> {

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
                            url, 
                            basepath: format!("slides/{}/activity", slide_id), 
                            filename,
                            transcode: Some((MediaTranscode::Video, filename_dest.clone()))
                        };

                        medias.push(media);

                        Some(filename_dest)
                    },

                    Err(_) => {
                        writeln!(&ctx.errors_log, "{} url {}, filename {} is 404 and not allowed to be (slide id: {}, game_url: {})", game_id, url, filename, slide_id, game_url).unwrap();
                        if ctx.opts.panic_on_404_error {
                            panic!("{} url {}, filename {} is 404 and not allowed to be (slide id: {}, game_url: {})", game_id, url, filename, slide_id, game_url);
                        } else {
                            None
                        }
                    }
                }
        }
    }
    pub async fn convert(ctx: &Context, slide: SrcSlide, game_url: &str, game_id: &str, base_url: &str, mut medias: &mut Vec<Media>, max_slides: usize) -> Slide {
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

        let image_full = {
            let filename = strip_path(&slide.image_full).to_string();
            medias.push(
                Media { 
                    url: format!("{}/{}/{}", base_url, slide_id, filename), 
                    basepath: format!("slides/{}", slide_id), 
                    filename: filename.clone(),
                    transcode: None
                }
            );
            filename
        };
        let image_thumb = {
            let filename = strip_path(&slide.image_thumb).to_string();
            medias.push(
                Media { 
                    url: format!("{}/{}/{}", base_url, slide_id, filename), 
                    basepath: format!("slides/{}", slide_id), 
                    filename: filename.clone(),
                    transcode: None
                }
            );
            filename
        };


        let validate_jump_index = |index: usize| -> Option<usize> {
            if index >= max_slides {
                if opts.allow_bad_jump_index {
                    log::warn!("invalid jump index: {} (there are only {} slides!)", index, max_slides);
                    writeln!(&ctx.warnings_log, "{} invalid jump index: {} (there are only {} slides!), game_url: {}", game_id, index, max_slides,  game_url).unwrap();
                    None
                } else {
                    writeln!(&ctx.errors_log, "{} invalid jump index: {} (there are only {} slides!), game_url: {}", game_id, index, max_slides, game_url).unwrap();
                    panic!("{} invalid jump index: {} (there are only {} slides!), game_url: {}", game_id, index, max_slides, game_url);
                }
            } else {
                Some(index)
            }
        };

        let activity = {
            if activities_len == 0 {
                None
            } else {
                if slide.activity_kind == SrcActivityKind::Questions {
                    let mut items:Vec<QuestionItem> = Vec::with_capacity(activities_len);

                    for activity in slide.activities.clone().into_iter() {

                        if activity.settings.bg_audio.is_some() {
                            panic!("Ask a question shouldn't have bg audio set..");
                        }

                        if activity.shapes.len() > 1 {
                            panic!("Ask a question can't have more than one shape...");
                        } else if activity.shapes.is_empty() {
                            log::warn!("ask a question with no questions?? skipping...");
                        } else {
                            let question_filename = slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &activity.intro_audio, false, &mut medias).await;
                            let shape = activity.shapes[0].clone();
                            let answer_filename = slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &shape.audio, false, &mut medias).await;
                            let wrong_filename = slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &shape.audio_2, false, &mut medias).await;
                            let hotspot = shape::convert_to_hotspot(shape);

                            items.push(QuestionItem{
                                question_filename,
                                answer_filename,
                                wrong_filename,
                                hotspot
                            });
                        }
                    }

                    Some(Activity::AskQuestions(AskQuestions {
                        items
                    }))
                } else {
                    let activity = slide.activities[0].clone();

                    let audio_filename = slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &activity.intro_audio, false, &mut medias).await;
                    let bg_audio_filename = match activity.settings.bg_audio {
                        None => None,
                        Some(bg_audio) => slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &bg_audio, true, &mut medias).await
                    };

                    match slide.activity_kind {
                        SrcActivityKind::SaySomething => {
                            Some(Activity::SaySomething(SaySomething {
                                audio_filename,
                                advance_trigger: if activity.settings.advance.unwrap_or_default() {
                                    AdvanceTrigger::AudioEnd
                                } else {
                                    AdvanceTrigger::Tap
                                },
                                advance_index: activity.settings.jump_index.and_then(validate_jump_index)
                            }))
                        },
                        SrcActivityKind::Soundboard => {

                            let mut items:Vec<SoundboardItem> = Vec::new();
                            let mut highlight_color:Option<String> = None;

                            for shape in activity.shapes.into_iter() {
                                match (highlight_color.as_ref(), shape.settings.highlight_color.as_ref()) {
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
                                    audio_filename: slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &shape.audio, false, &mut medias).await,
                                    text: map_text(&shape.settings.text),
                                    jump_index: shape.settings.jump_index.and_then(validate_jump_index),
                                    hotspot: shape::convert_to_hotspot(shape)
                                });
                            }

                            let one_at_a_time = match (activity.settings.fun_mode, activity.settings.fun_mode_v2) {
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

                            let show_hints = match activity.settings.hide_hints {
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
                            match activity.settings.video_url {
                                None => None,
                                Some(video_url) => {
                                    let transform_matrix = activity.settings.transform.map(convert_transform);
                                    let video_url = video_url.replace("http://", "https://");

                                    let src = match <YoutubeUrl as YoutubeUrlExt>::try_parse(video_url.clone()) {
                                        Ok(yt) => {
                                            log::info!("yt: {}", yt.get_id());
                                            VideoSource::Youtube(yt)
                                        },
                                        Err(_) => {
                                            let filename = video_url.replace("local://", "");

                                            match slide::make_video_media(&ctx,&game_id, &game_url, &slide, base_url, &filename, false, &mut medias).await {
                                                None => {
                                                    panic!("unable to get url from {}", video_url);
                                                },
                                                Some(filename) => {
                                                    log::info!("not yt: {}", filename);
                                                    VideoSource::Direct(filename)
                                                }
                                            }

                                        }
                                    };

                                    let range = activity.settings.video_range.and_then(|range_str| {
                                        //yes, really
                                        scan_fmt!(&range_str, "{{{}, {}}}", f64, f64).ok()
                                    });
                                    
                                    Some(Activity::Video(Video {
                                        transform_matrix,
                                        src,
                                        range
                                    }))
                                }
                            } 
                        },
                        SrcActivityKind::Puzzle => {
                            let mut items:Vec<PuzzleItem> = Vec::new();

                            for shape in activity.shapes.into_iter() {
                                items.push(PuzzleItem {
                                    audio_filename: slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &shape.audio, false, &mut medias).await,
                                    hotspot: shape::convert_to_hotspot(shape)
                                });
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

                            let theme = if map_theme(&activity.settings.theme) == PuzzleTheme::Extrude || activity.settings.theme_v2.unwrap_or_default() == true {
                                PuzzleTheme::Extrude
                            } else {
                                PuzzleTheme::Regular
                            };


                            Some(Activity::Puzzle(Puzzle {
                                audio_filename,
                                jump_index: activity.settings.jump_index.and_then(validate_jump_index),
                                full_cutout_img: image_full,
                                //show_hints: activity.settings.tooltip.unwrap_or(false),
                                show_hints: !activity.settings.hints_disabled.unwrap_or(true),
                                fly_back_to_origin: !activity.settings.fun_mode.unwrap_or(false),
                                show_preview: activity.settings.show_shape.unwrap_or(false) || activity.settings.show_shape_v2.unwrap_or(false),
                                theme,
                                items
                            }))

                        },
                        SrcActivityKind::TalkType => {

                            let mut items:Vec<TalkTypeItem> = Vec::new();

                            for shape in activity.shapes.into_iter() {
                                items.push(TalkTypeItem {
                                    audio_filename: slide::make_audio_media(&ctx, &game_id, &game_url, &slide, base_url, &shape.audio, false, &mut medias).await,
                                    texts: match shape.settings.text_answers.as_ref() {
                                        None => None,
                                        Some(answers) => {
                                            Some(answers.iter().map(|x| x.trim().to_string()).collect())
                                        }
                                    },
                                    input_language: shape.settings.text_input_language.clone(),
                                    answer_kind: if shape.settings.speaking_mode.unwrap_or(false) {
                                        TalkTypeAnswerKind::Audio
                                    } else {
                                        TalkTypeAnswerKind::Text
                                    },
                                    hotspot: shape::convert_to_hotspot(shape)
                                });
                            }
                            Some(Activity::TalkType(TalkType {
                                audio_filename,
                                jump_index: activity.settings.jump_index.and_then(validate_jump_index),
                                show_hints: activity.settings.tooltip.unwrap_or(false),
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


        let design = convert_design(&game_url, &game_id, &slide_id, &base_url, &mut medias, slide.layers);

        Slide {
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

fn convert_design(game_url: &str, game_id: &str, slide_id: &str, base_url: &str, mut medias: &mut Vec<Media>, layers: Vec<SrcLayer>) -> Design {
    let mut stickers: Vec<Sticker> = Vec::new();
    let mut bgs:Vec<String> = Vec::new();
  
    for layer in layers {

        if let Some(filename) = layer.filename.as_ref() {
            if !filename.is_empty() {
                medias.push(
                    Media { 
                        url: format!("{}/{}/layers/{}", base_url, slide_id, filename), 
                        basepath: format!("slides/{}", slide_id), 
                        filename: filename.to_string(),
                        transcode: None
                    }
                );
            }
        }

        let audio_filename = layer.audio.as_ref().and_then(|audio| {
            if audio.is_empty() {
                None
            } else {
                log::info!("{:?}", audio);

                let filename_dest = format!("{}.mp3", Path::new(&audio).file_stem().unwrap().to_str().unwrap().to_string());

                medias.push(Media { 
                    url: format!("{}/{}/layers/{}", base_url, slide_id, audio), 
                    basepath: format!("slides/{}", slide_id), 
                    filename: audio.to_string(),
                    transcode: Some((MediaTranscode::Audio, filename_dest.clone()))
                });

                Some(filename_dest)
            }
        });


        match layer.kind {
            SrcLayerKind::Background => {
                bgs.push(layer.filename.unwrap());
            },
            SrcLayerKind::Text | SrcLayerKind::Image | SrcLayerKind::Animation => {
                if let Some(filename) = layer.filename.as_ref() {
                    let sticker = Sticker { 
                        filename: filename.to_string(),
                        transform_matrix: convert_transform(layer.transform),
                        hide: match layer.show_kind {
                            SrcShowKind::ShowOnLoad => false, 
                            SrcShowKind::HideOnTap => false, 
                            SrcShowKind::ShowOnTap => true, 
                        },

                        hide_toggle: match layer.show_kind {
                            SrcShowKind::ShowOnLoad => None, 
                            _ => Some(
                                if layer.toggle_show {
                                    HideToggle::Always
                                } else {
                                    HideToggle::Once
                                }
                            ), 
                        },

                        animation: {
                            if layer.kind == SrcLayerKind::Animation {
                                Some(
                                    match layer.loop_kind {
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
                            Some((layer.width, layer.height))
                        },

                        audio_filename,


                    };

                    stickers.push(sticker);
                } else {
                    log::warn!("expected filename for layer kind {:?}, game_id: {}, slide_id: {}, game_url: {}", &layer.kind, &game_id, &slide_id, &game_url);
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

    pub fn convert_to_hotspot(shape: SrcShape) -> Hotspot {
        Hotspot {
            shape: TraceShape::PathCommands(
               shape 
                    .path
                    .into_iter()
                    .map(|point| (convert_point(point), true))
                    .collect()
            ),
            transform_matrix: shape.settings.transform.and_then(|t| {
                if !transform_is_identity(t) {
                    Some(convert_transform(t))
                } else {
                    None
                }
            })
        }
    }

    pub fn convert_point(point: SrcPathPoint) -> PathCommand {
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

fn strip_path(orig:&str) -> &str {
    match orig.rsplit_once("/") {
        None => orig,
        Some((_, end)) => end 
    }
}