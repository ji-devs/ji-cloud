use serde::{Serialize, Deserialize};
use serde_repr::*;

mod shape;
pub use shape::*;

mod design;
pub use design::*;

mod activities;
pub use activities::*;

pub type Id = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio: Option<String>,
    pub modules: Vec<Id> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub id: Id,

    pub image_full: String,

    pub image_thumb: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,

    pub design: Design
}

