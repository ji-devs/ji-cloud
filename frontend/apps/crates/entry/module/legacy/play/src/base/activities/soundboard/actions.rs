use super::state::*;
use std::rc::Rc;

use dominator::clone;
use utils::prelude::*;

impl Soundboard {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            state.base.audio_manager.play_clip_on_ended(
                state.base.activity_media_url(&audio_filename),
                clone!(state => move || {
                    state.on_intro_finished();
                }),
            );
        } else {
            state.on_intro_finished();
        }
    }

    pub fn on_intro_finished(&self) {
        if let Some(bg_audio_filename) = self.raw.bg_audio_filename.as_ref() {
            self.base
                .audio_manager
                .play_bg(self.base.activity_media_url(&bg_audio_filename));
        }

        if self.raw.show_hints {
            self.phase.set_neq(Phase::Hints);
        } else {
            self.on_hints_finished();
        }
    }

    pub fn on_hints_finished(&self) {
        self.base.allow_stage_click();
        self.phase.set_neq(Phase::Playing);
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
                    let msg = if let Some(index) = state.jump_index {
                        let index = index + 1; // bump for cover
                        log::info!("going to index {}!", index);

                        Some(IframeAction::new(ModuleToJigPlayerMessage::JumpToIndex(index)))
                    } else {
                        let all_revealed = parent.items.iter().all(|item| item.revealed.get());

                        if all_revealed {
                            log::info!("finished all, going next");
                            Some(IframeAction::new(ModuleToJigPlayerMessage::Next))
                        } else {
                            None
                        }
                    };

                    if let Some(msg) = msg {
                        let _ = msg.try_post_message_to_top();
                    }
                })
            );
        }
    }
}
