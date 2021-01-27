use serde::{Serialize, Deserialize};
use serde_repr::*;

mod path;
pub use path::*;

mod design;
pub use design::*;

mod activities;
pub use activities::*;

mod module;
pub use module::*;

pub type Id = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio: Option<String>,
    pub modules: Vec<Id> 
}

