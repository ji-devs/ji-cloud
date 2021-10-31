use serde::{Deserialize, Deserializer, de, serde_if_integer128};
use serde_repr::*;
use std::{
    path::{Path, PathBuf},
    fs::File,
    fmt,
    future::Future
};
use super::{
    super::super::options::*,
    MediaTranscode, 
    data::{
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

use shared::domain::jig::{
    JigCreateRequest, 
    JigData, 
    JigPlayerSettings, 
    module::{
        ModuleCreateRequest, 
        ModuleBody, 
        body::{
            Transform,
            _groups::design::{PathCommand, TraceKind, TraceShape},
            legacy::{
                ModuleData,
                slide::*,
                design::*,
                activity::*
            }
        }
    }
};
use reqwest::Client; 
use utils::{math::mat4::Matrix4, prelude::*};
use crate::config::{REFERENCE_HEIGHT, REFERENCE_WIDTH};

impl SrcManifest {
    pub fn load_file(path:PathBuf) -> Self {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub async fn load_url(url:&str, client:&Client) -> (Self, String) {

        let text = 
            client.get(url)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .text()
                .await
                .unwrap();

        (
            serde_json::from_str(&text).unwrap(),
            text
        )
    }

    pub fn jig_req(&self) -> JigCreateRequest {
        //let background_audio = if src.music_file == "" { None } else { Some(src.music_file) };
        
        // TODO- populate
        JigCreateRequest { 
            display_name: "".to_string(), 
            goals: Vec::new(), 
            age_ranges: Vec::new(), 
            affiliations: Vec::new(), 
            language: None, 
            categories: Vec::new(), 
            description: "".to_string(), 
            default_player_settings: JigPlayerSettings::default()
        }
    }

    pub fn module_reqs(&self) -> Vec<ModuleCreateRequest> {
        self.structure
            .slides
            .iter()
            .map(|slide| {
                ModuleCreateRequest {
                    body: ModuleBody::Legacy(
                        ModuleData {
                            game_id: self.game_id(),
                            slide_id: slide.slide_id()
                        },
                    )
                }
            })
            .collect()
    }

    pub async fn into_slides(self, client: &Client, opts: &Opts) -> (Vec<Slide>, Vec<Media>) {
        let mut medias:Vec<Media> = Vec::new();

        let game_id = self.game_id();
        let base_url =  self.base_url.trim_matches('/').to_string();

        let mut slides:Vec<Slide> = Vec::new();
       
        let max_slides = self.structure.slides.len();

        for slide in self.structure.slides.into_iter() {
            slides.push(slide.convert(&opts, &client, &game_id, &base_url, &mut medias, max_slides).await);
        }
            

        (slides, medias)
    }
}


impl SrcSlide {
    async fn make_audio_media(&self, client: &Client, base_url: &str, filename: &str, allowed_empty: bool, mut medias: &mut Vec<Media>) -> Option<String> {

        let slide_id = self.slide_id(); 

        if filename.is_empty() {
            None
        } else {
            let filename_mp3 = format!("{}.mp3", Path::new(filename).file_stem().unwrap().to_str().unwrap().to_string());

            let url = format!("{}/{}", base_url, filename);

            match client.head(&url)
                .send()
                .await
                .unwrap()
                .error_for_status() {
                    Ok(_) => {

                        let media = Media {
                            url, 
                            basepath: format!("slides/{}/activity", slide_id), 
                            filename: filename_mp3.to_string(),
                            transcode: Some(MediaTranscode::Audio)
                        };

                        medias.push(media);

                        Some(filename_mp3)
                    },
                    Err(_) => {
                        if allowed_empty {
                            log::info!("skipping {} because file doesn't exist- but this is allowed here", url);
                            None
                        } else {
                            panic!("{} is 404 and not allowed to be", url);
                        }
                    }
                }
        }
    }
    pub async fn convert(self, opts: &Opts, client: &Client, game_id: &str, base_url: &str, mut medias: &mut Vec<Media>, max_slides: usize) -> Slide {
        let slide_id = self.slide_id(); 

        log::info!("parsing slide: {}", slide_id);

        let activities_len = self.activities.len();
        let layers_len = self.layers.len();

        if activities_len > 1 && self.activity_kind != SrcActivityKind::Questions {
            log::error!("{:#?}", self.activities);
            panic!("{} is more than one activity and not ask a question?!", self.activities.len());
        }

        let image_full = strip_path(&self.image_full).to_string();
        let image_thumb = strip_path(&self.image_thumb).to_string();

        let validate_jump_index = |index: usize| -> Option<usize> {
            if index >= max_slides {
                if opts.allow_bad_jump_index {
                    log::warn!("invalid jump index: {} (there are only {} slides!)", index, max_slides);
                    None
                } else {
                    panic!("invalid jump index: {} (there are only {} slides!)", index, max_slides);
                }
            } else {
                Some(index)
            }
        };

        let activity = {
            if activities_len == 0 {
                None
            } else {
                let activity = self.activities[0].clone();


                let audio_filename = self.make_audio_media(&client, base_url, &activity.intro_audio, false, &mut medias).await;
                let bg_audio_filename = match activity.settings.bg_audio {
                    None => None,
                    Some(bg_audio) => self.make_audio_media(&client, base_url, &bg_audio, true, &mut medias).await
                };

                match self.activity_kind {
                    // SrcActivityKind::Questions => {
                    //     let questions: Vec<Question> = 
                    //         self.activities
                    //             .into_iter()
                    //             .map(|activity| {
                    //                 activity.convert_question() 
                    //             })
                    //             .collect();

                    //     Some(Activity::Questions(Questions {
                    //         questions
                    //     }))
                    // },
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
                                audio_filename: self.make_audio_media(&client, base_url, &shape.audio, false, &mut medias).await,
                                text: map_text(&shape.settings.text),
                                jump_index: shape.settings.jump_index.and_then(validate_jump_index),
                                hotspot: shape.convert_to_hotspot()
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
                    _ => None
                }
            }
        };


        let design = convert_design(&game_id, &slide_id, &base_url, &mut medias, self.layers);

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
    // pub hide: bool,
    // pub hide_toggle: Option<HideToggle>,
    // pub animation: Option<Animation>
fn convert_design(game_id: &str, slide_id: &str, base_url: &str, mut medias: &mut Vec<Media>, layers: Vec<SrcLayer>) -> Design {
    let mut stickers: Vec<Sticker> = Vec::new();
    let mut bgs:Vec<String> = Vec::new();
  
    let mut make_media = |filename:&str, transcode:Option<MediaTranscode>| -> Media {
        Media { 
            url: format!("{}/{}/layers/{}", base_url, slide_id, filename), 
            basepath: format!("slides/{}", slide_id), 
            filename: filename.to_string(),
            transcode
        }
    };

    for layer in layers {

        if let Some(filename) = layer.filename.as_ref() {
            medias.push(make_media(&filename, None));
        }
        if let Some(filename) = layer.audio.as_ref() {
            medias.push(make_media(&filename, Some(MediaTranscode::Audio)));
        }

        /// as of today, mp3 has full cross-browser support
        let audio_filename = layer.audio.as_ref().map(|audio| format!("{}.mp3", Path::new(&audio).file_stem().unwrap().to_str().unwrap().to_string()));

        match layer.kind {
            SrcLayerKind::Background => {
                bgs.push(layer.filename.unwrap());
            },
            SrcLayerKind::Text | SrcLayerKind::Image | SrcLayerKind::Animation => {
                let sticker = Sticker { 
                    filename: layer.filename.unwrap(),
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

impl SrcShape {
    pub fn convert_to_hotspot(self) -> Hotspot {
        Hotspot {
            shape: TraceShape::PathCommands(
                self
                    .path
                    .into_iter()
                    .map(|point| (point.convert(), true))
                    .collect()
            ),
            transform_matrix: self.settings.transform.and_then(|t| {
                if !transform_is_identity(t) {
                    Some(convert_transform(t))
                } else {
                    None
                }
            })
        }
    }
}

impl SrcPathPoint {
    pub fn convert(self) -> PathCommand {
        let SrcPathPoint { mut x, mut y, mut cp1x, mut cp1y, mut cp2x, mut cp2y, kind} = self;

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
/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Slide {
    pub base_path: String,

    pub image_full: String,

    pub image_thumb: String,

    pub activity: Option<Activity>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Activity {
    Questions(Questions)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Questions {

}
*/
