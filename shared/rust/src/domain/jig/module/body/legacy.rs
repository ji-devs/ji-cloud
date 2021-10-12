#[allow(missing_docs)]
pub mod activity;
#[allow(missing_docs)]
pub mod design;
#[allow(missing_docs)]
pub mod path;

use crate::domain::jig::module::{
    body::{Body, BodyConvert, BodyExt, ThemeChoice},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// only used for transcoding purposes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    /// backround audio- may not have jig equivilent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio: Option<String>,
    /// list of original module ids
    pub modules: Vec<String>,
}

/// The body for [`Legacy`](crate::domain::jig::module::ModuleKind::Legacy) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// base id for all file loading
    pub base_id: String,
    /// Design layer  
    pub design: design::Design,

    #[allow(missing_docs)]
    pub id: String,

    #[allow(missing_docs)]
    pub image_full: String,

    #[allow(missing_docs)]
    pub image_thumb: String,

    #[allow(missing_docs)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<activity::Activity>,
}

impl BodyExt<(), ()> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Legacy(self.clone())
    }

    fn is_complete(&self) -> bool {
        true
    }

    fn is_legacy() -> bool {
        true
    }

    fn kind() -> ModuleKind {
        ModuleKind::Legacy
    }
    fn new_mode(_mode: ()) -> Self {
        unimplemented!("can't create new legacy modules!")
    }

    fn requires_choose_mode(&self) -> bool {
        false
    }

    fn set_editor_state_step(&mut self, _step: ()) {}
    fn set_editor_state_steps_completed(&mut self, _steps_completed: HashSet<()>) {}

    fn get_editor_state_step(&self) -> Option<()> {
        None
    }

    fn get_editor_state_steps_completed(&self) -> Option<HashSet<()>> {
        None
    }

    fn get_theme(&self) -> Option<ThemeChoice> {
        None
    }
}

impl BodyConvert for ModuleData {}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Legacy(data) => Ok(data),
            _ => Err("cannot convert body to legacy!"),
        }
    }
}
