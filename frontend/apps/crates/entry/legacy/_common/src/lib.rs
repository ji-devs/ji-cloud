use serde::{Serialize, Deserialize};
use serde_repr::*;

mod shape;
pub use shape::*;

mod design;
pub use design::*;

mod activities;
pub use activities::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio: Option<String>,
    pub n_slides: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slide {
    pub base_path: String,

    pub image_full: String,

    pub image_thumb: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,

    pub design: Design
}

