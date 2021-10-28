use std::rc::Rc;
use super::state::{Soundboard, SoundboardItem};
use components::audio::mixer::{AUDIO_MIXER, AudioSource};
use shared::domain::jig::module::body::legacy::activity::AdvanceTrigger;
use utils::prelude::*;
use dominator::{Dom, html, clone};

impl Soundboard {
    pub fn on_start(self: Rc<Self>) {

        let state = self;

        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            AUDIO_MIXER.with(|mixer| {
                mixer.pause_all();

                mixer.play_oneshot(AudioSource::Url(state.base.activity_media_url(&audio_filename)))
            });
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


        if let Some(audio_filename) = state.audio_filename.as_ref() {
            AUDIO_MIXER.with(|mixer| {
                mixer.pause_all();
                mixer.play_oneshot_on_ended(AudioSource::Url(state.base.activity_media_url(&audio_filename)), clone!(state => move || {
                    if let Some(index) = state.jump_index {
                        let _ = IframeAction::new(ModuleToJigPlayerMessage::JumpToIndex(index)).try_post_message_to_top();
                    }
                }))
            });
        }

        
    }
}