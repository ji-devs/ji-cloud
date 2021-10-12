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
use shared::domain::jig::module::body::legacy::{
    Manifest,
    ModuleData,
    design::*,
    activity::*
};

impl SrcManifest {
    pub fn convert(self, base_id:&str) -> (Manifest, Vec<ModuleData>) {
        let src = self.structure;

        let background_audio = if src.music_file == "" { None } else { Some(src.music_file) };

        let manifest = Manifest {
            background_audio,
            modules: src
                .slides
                .iter()
                .map(|slide| slide.file_path.trim_matches('/').to_string())
                .collect()
        };


        let slides = 
            src
                .slides
                .into_iter()
                .map(|src_slide| {
                    src_slide.convert(base_id)
                })
                .collect();



        (manifest, slides)
    }
}

impl SrcSlide {
    pub fn convert(self, base_id: &str) -> ModuleData {

        let activities_len = self.activities.len();
        let layers_len = self.layers.len();

        if activities_len > 1 && self.activity_kind != SrcActivityKind::Questions {
            log::error!("{:#?}", self.activities);
            panic!("{} is more than one activity and not ask a question?!", self.activities.len());
        }

        let id = self.file_path.trim_matches('/').to_string();
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

        ModuleData {
            base_id: base_id.to_string(),
            id,
            image_full,
            image_thumb,
            activity,
            design,
        }
    }
}

fn convert_design(layers: Vec<SrcLayer>) -> Design {
    let mut stickers: Vec<Sticker> = Vec::new();
    let mut bg:Option<String> = None;
   
    for layer in layers {
        match layer.kind {
            SrcLayerKind::Background => {
                bg = Some(layer.filename.unwrap());
            },
            SrcLayerKind::Image | SrcLayerKind::Animation => {
                let sticker = Image {
                    src: layer.filename.unwrap(),
                    width: layer.width,
                    height: layer.height,
                    transform: layer.transform,
                    show_kind: match layer.show_kind {
                        SrcShowKind::ShowOnLoad => ShowKind::ShowOnLoad,
                        SrcShowKind::HideOnTap => ShowKind::HideOnTap,
                        SrcShowKind::ShowOnTap => ShowKind::ShowOnTap,
                    }
                };

                stickers.push(Sticker::Image(sticker));
            },
            SrcLayerKind::Text => {
                let sticker = Text {
                    html: layer.html.unwrap(),
                    width: layer.width,
                    height: layer.height,
                    transform: layer.transform,
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
        bg,
        stickers
    }
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Design {
    pub bg: Option<String>,
    pub stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub src: String,
    pub width: f64,
    pub height: f64,
    pub transform: [f64;6],
    pub show_kind: ShowKind, 
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ShowKind {
    ShowOnLoad,
    HideOnTap,
    ShowOnTap,
}
*/


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
