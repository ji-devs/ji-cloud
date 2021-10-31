use std::rc::Rc;
use super::state::SaySomething;
use shared::domain::jig::module::body::legacy::activity::AdvanceTrigger;
use utils::prelude::*;
use dominator::{Dom, html, clone};

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
                })
            );
        }
    }

    pub fn next(&self) {

        let msg = match self.raw.advance_index {
            Some(index) => {
                log::info!("going to index {}!", index);
                IframeAction::new(ModuleToJigPlayerMessage::JumpToIndex(index))
            },
            None => {
                log::info!("going next!");
                IframeAction::new(ModuleToJigPlayerMessage::Next)
            },
        };

        if let Err(_) = msg.try_post_message_to_top() {
            log::info!("Couldn't post message to top... debugging?");
        }
    }
}