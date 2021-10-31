use std::{borrow::BorrowMut, rc::Rc};
use super::state::{Soundboard, SoundboardItem};
use shared::domain::jig::module::body::legacy::activity::AdvanceTrigger;
use utils::prelude::*;
use dominator::{Dom, html, clone};

impl Soundboard {
    pub fn on_start(self: Rc<Self>) {

        let state = self;

        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            state.base.audio_manager.play_clip(state.base.activity_media_url(&audio_filename));
        }

        if let Some(bg_audio_filename) = state.raw.bg_audio_filename.as_ref() {
            state.base.audio_manager.play_bg(state.base.activity_media_url(&bg_audio_filename));
        }
    }

}


impl SoundboardItem {
    pub fn on_click(self: Rc<Self>, parent: Rc<Soundboard>) {
        let state = self;

        let was_revealed = state.revealed.replace(true);
        if !was_revealed {
            log::info!("first time!");
        }

        state.hotspot.tooltip_text.set(state.text.clone());


        if let Some(audio_filename) = state.audio_filename.as_ref() {
            state.base.audio_manager.play_clip_on_ended(
                state.base.activity_media_url(&audio_filename),
                clone!(state => move || {
                    if let Some(index) = state.jump_index {
                        let _ = IframeAction::new(ModuleToJigPlayerMessage::JumpToIndex(index)).try_post_message_to_top();
                    } else {
                        //TODO- check if last clip
                    }
                })
            );
        }

        
    }
}