use std::rc::Rc;
use super::state::Soundboard;
use components::audio::mixer::{AUDIO_MIXER, AudioSource};
use shared::domain::jig::module::body::legacy::activity::AdvanceTrigger;
use utils::prelude::*;
use dominator::{Dom, html, clone};

impl Soundboard {
    pub fn on_start(self: Rc<Self>) {
        let state = self;
    }

}