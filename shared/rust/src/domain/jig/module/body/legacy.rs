use crate::domain::jig::module::{
    body::{Body, BodyConvert, BodyExt, ModeExt, StepExt, ThemeChoice, _groups::design::*},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Legacy) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {}

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
