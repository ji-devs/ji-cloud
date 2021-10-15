use super::src_manifest::{
    SrcManifest,
    slide::Slide as SrcSlide,
    activity::ActivityKind as SrcActivityKind,
    activity::Activity as SrcActivity,
    shape::Shape as SrcShape,
    shape::PathPoint as SrcPathPoint,
    shape::PathElementKind as SrcPathElementKind,
    layer::Layer as SrcLayer,
    layer::LayerKind as SrcLayerKind,
    layer::PlayKind as SrcPlayKind,
    layer::ShowKind as SrcShowKind,
};
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

    pub fn module_reqs(&self, game_id:&str) -> Vec<ModuleCreateRequest> {
        self.structure
            .slides
            .iter()
            .map(|slide| {
                ModuleCreateRequest {
                    body: ModuleBody::Legacy(
                        ModuleData {
                            game_id: game_id.to_string(),
                            slide_id: slide.file_path.trim_matches('/').to_string()
                        },
                    )
                }
            })
            .collect()
    }

    pub fn into_slides(self) -> Vec<Slide> {
        self.structure
            .slides
            .into_iter()
            .map(|slide| slide.convert())
            .collect()
    }
}

impl SrcSlide {
    pub fn convert(self) -> Slide {

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
                    _ => None
                }
            }
        };


        let design = convert_design(self.layers);

        Slide {
            activity,
            design,
        }
    }
}

fn convert_design(layers: Vec<SrcLayer>) -> Design {
    let mut stickers: Vec<Sticker> = Vec::new();
    let mut bgs:Vec<String> = Vec::new();
   
    for layer in layers {
        match layer.kind {
            SrcLayerKind::Background => {
                bgs.push(layer.filename.unwrap());
            },
            SrcLayerKind::Image | SrcLayerKind::Animation => {
                let sticker = Sprite { 
                    src: layer.filename.unwrap(),
                    transform_matrix: convert_transform(layer.transform),
                    show_kind: match layer.show_kind {
                        SrcShowKind::ShowOnLoad => ShowKind::ShowOnLoad,
                        SrcShowKind::HideOnTap => ShowKind::HideOnTap,
                        SrcShowKind::ShowOnTap => ShowKind::ShowOnTap,
                    }
                };

                stickers.push(Sticker::Sprite(sticker));
            },
            SrcLayerKind::Text => {
                let sticker = Text {
                    html: layer.html.unwrap(),
                    width: layer.width,
                    height: layer.height,
                    transform_matrix: convert_transform(layer.transform),
                    show_kind: match layer.show_kind {
                        SrcShowKind::ShowOnLoad => ShowKind::ShowOnLoad,
                        SrcShowKind::HideOnTap => ShowKind::HideOnTap,
                        SrcShowKind::ShowOnTap => ShowKind::ShowOnTap,
                    }
                };

                stickers.push(Sticker::Text(sticker));
            }
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
