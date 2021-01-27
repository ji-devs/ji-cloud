use serde::{Serialize, Deserialize};
use serde_repr::*;

pub use super::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub id: Id,

    pub image_full: String,

    pub image_thumb: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,

    pub design: Design
}

