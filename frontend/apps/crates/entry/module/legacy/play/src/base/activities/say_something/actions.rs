use super::state::SaySomething;
use dominator::clone;
use shared::domain::jig::module::body::legacy::activity::AdvanceTrigger;
use std::rc::Rc;
use utils::prelude::*;
use crate::base::actions::NavigationTarget;

impl SaySomething {
    pub fn on_bg_click(self: Rc<Self>) {
        if self.raw.advance_trigger == AdvanceTrigger::Tap {
            self.next();
        }
    }

    pub fn on_start(self: Rc<Self>) {
        let state = self;

        state.base.allow_stage_click();

        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            state.base.audio_manager.play_clip_on_ended(
                state.base.activity_media_url(&audio_filename),
                clone!(state => move || {
                    if state.raw.advance_trigger == AdvanceTrigger::AudioEnd {
                        state.next();
                    }
                }),
            );
        }
    }

    pub fn next(&self) {
        match self.raw.advance_index {
            Some(index) => {
                let index = index + 1; // bump for cover
                log::info!("going to index {}!", index);

                self.base.navigate(NavigationTarget::Index(index));
            }
            None => {
                log::info!("going next!");
                self.base.navigate(NavigationTarget::Next);
            }
        };
    }
}
