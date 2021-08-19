use dominator::animation::MutableAnimation;

use crate::animation::fade::*;
use shared::domain::jig::module::body::Instructions;
use std::cell::RefCell;

use crate::audio::mixer::AudioHandle;

pub struct InstructionsPlayer {
    pub(super) data: Instructions,
    pub(super) fade: Fade,
    pub(super) audio: RefCell<Option<AudioHandle>>,
}

impl Drop for InstructionsPlayer {
    fn drop(&mut self) {
        log::info!("Instructions player dropped...");
    }
}

impl InstructionsPlayer {
    pub fn new(data: Instructions) -> Self {
        /*
        let data = Instructions {
            text: Some("instructions here!".to_string()),
            audio: None
        };
        */
        log::info!("Instructions Player created...");

        let _animation = MutableAnimation::new(1000.0);
        Self {
            data,
            fade: Fade::new(FadeKind::Out, 1000.0, true, Some(3000.0), None::<fn()>),
            audio: RefCell::new(None),
        }
    }
}
