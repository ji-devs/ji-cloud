use super::state::*;
use std::rc::Rc;
use shared::domain::jig::module::body::Audio;
use crate::base::state::*;
use dominator::clone;

impl Base {
    pub fn set_instructions_audio(&self, audio: Option<Audio>) { 
        self.instructions.replace_with(clone!(audio => move |instructions| {
            let mut instructions = instructions.clone();
            instructions.audio = audio;
            instructions
        }));

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.base.instructions.audio = audio;
            }
        });
    }

}
