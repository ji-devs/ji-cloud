use serde::{Deserialize, Deserializer, de, serde_if_integer128};
use serde_repr::*;
use std::{
    path::{Path, PathBuf},
    fs::File,
    fmt
};
use super::{MediaTranscode, data::{
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
}};

use shared::domain::jig::{JigCreateRequest, JigData, JigPlayerSettings, module::{ModuleCreateRequest, ModuleBody, body::{
        Transform,
        legacy::{
            ModuleData,
            slide::*,
            design::*,
            activity::*
        }
    }}};

use utils::{math::mat4::Matrix4, prelude::*};

impl SrcManifest {
    pub fn load_file(path:PathBuf) -> Self {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub async fn load_url(url:&str) -> (Self, String) {

        let text = 
            reqwest::get(url)
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

    pub fn into_slides(self) -> (Vec<Slide>, Vec<Media>) {
        let mut medias:Vec<Media> = Vec::new();

        let game_id = self.game_id();
        let base_url =  self.base_url.trim_matches('/').to_string();

        let slides = self.structure
            .slides
            .into_iter()
            .map(|slide| slide.convert(&game_id, &base_url, &mut medias))
            .collect();

        (slides, medias)
    }
}


impl SrcSlide {
    pub fn convert(self, game_id: &str, base_url: &str, mut medias: &mut Vec<Media>) -> Slide {

        let slide_id =  self.file_path.trim_matches('/').to_string();


        let activities_len = self.activities.len();
        let layers_len = self.layers.len();

        if activities_len > 1 && self.activity_kind != SrcActivityKind::Questions {
            log::error!("{:#?}", self.activities);
            panic!("{} is more than one activity and not ask a question?!", self.activities.len());
        }

        let image_full = strip_path(&self.image_full).to_string();
        let image_thumb = strip_path(&self.image_thumb).to_string();

        let activity = {
            if activities_len == 0 {
                None
            } else {
                let activity = &self.activities[0];


                let make_media = |filename:&str, transcode:Option<MediaTranscode>| -> Media {
                    Media { 
                        url: format!("{}/{}/{}/{}", base_url, slide_id, activity.folder_path, filename), 
                        basepath: format!("slides/{}", slide_id), 
                        filename: filename.to_string(),
                        transcode
                    }
                };

                let audio_filename = if !activity.intro_audio.is_empty() {
                    Some(format!("{}.mp3", Path::new(&activity.intro_audio).file_stem().unwrap().to_str().unwrap().to_string()))
                } else {
                    None
                };

                if let Some(filename) = audio_filename.as_ref() {
                    medias.push(Media {
                        url: format!("{}/{}", base_url, activity.intro_audio), 
                        basepath: format!("slides/{}/activity", slide_id), 
                        filename: filename.to_string(),
                        transcode: Some(MediaTranscode::Audio)
                    });
                }

                match self.activity_kind {
                    SrcActivityKind::Questions => {
                        let questions: Vec<Question> = 
                            self.activities
                                .into_iter()
                                .map(|activity| {
                                    activity.convert_question() 
                                })
                                .collect();

                        Some(Activity::Questions(Questions {
                            questions
                        }))
                    },
                    SrcActivityKind::SaySomething => {
                        Some(Activity::SaySomething(SaySomething {
                            audio_filename: audio_filename.unwrap(),
                            advance_trigger: if activity.settings.advance.unwrap_or_default() {
                                AdvanceTrigger::AudioEnd
                            } else {
                                AdvanceTrigger::Tap
                            },
                            advance_index: activity.settings.jump_index
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
    // pub hide: bool,
    // pub hide_toggle: Option<HideToggle>,
    // pub animation: Option<Animation>
fn convert_design(game_id: &str, slide_id: &str, base_url: &str, mut medias: &mut Vec<Media>, layers: Vec<SrcLayer>) -> Design {
    let mut stickers: Vec<Sticker> = Vec::new();
    let mut bgs:Vec<String> = Vec::new();
  
    let make_media = |filename:&str, transcode:Option<MediaTranscode>| -> Media {
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
    let translate_x = orig[4] / 1024.0;
    let translate_y = orig[5] / 768.0;

    let mut m = Matrix4::identity();

    m.translate(&[translate_x, translate_y, 0.0]);
    m.scale(&[scale_x, scale_y, 1.0]);
    m.skew_x(skew_x);
    m.skew_y(skew_y);


    m.values()
}

impl SrcActivity {
    pub fn convert_question(self) -> Question {
        let audio = format!("{}{}", self.folder_path, strip_path(&self.intro_audio));

        let shapes_len = self.shapes.len();

        if shapes_len != 1 {
            panic!("question needs 1 shape! instead got {}", shapes_len);
        }
        Question {
            audio,
            path: self.shapes[0].convert_path()
        }
    }
}

impl SrcShape {
    pub fn convert_path(&self) -> Vec<PathPoint> {
        self
            .path
            .iter()
            .map(|point| point.clone().convert())
            .collect()
    }
}
impl SrcPathPoint {
    pub fn convert(self) -> PathPoint {
        let kind = match self.kind {
            SrcPathElementKind::MoveToPoint => PathElementKind::MoveToPoint,
            SrcPathElementKind::AddLineToPoint => PathElementKind::AddLineToPoint,
            SrcPathElementKind::AddQuadCurveToPoint => PathElementKind::AddQuadCurveToPoint,
            SrcPathElementKind::AddCurveToPoint => PathElementKind::AddCurveToPoint,
            SrcPathElementKind::CloseSubPath => PathElementKind::CloseSubPath,
        };

        let SrcPathPoint { x, y, cp1x, cp1y, cp2x, cp2y, ..} = self;

        PathPoint {
            kind,
            x, y, 
            cp1x, cp1y, 
            cp2x, cp2y
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
