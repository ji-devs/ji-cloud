use super::state::*;

use std::rc::Rc;
use utils::prelude::*;


impl TalkType {

    pub fn on_start(self: Rc<Self>) {
        let state = self;

        state.base.allow_stage_click();

        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            state.base.audio_manager.play_clip_on_ended(
                state.base.activity_media_url(&audio_filename),
                || {},
            );
        }
    }

    pub fn evaluate_all(self: Rc<Self>) {
        let state = self;

        if state.items.iter().all(|item| item.phase.get() == TalkTypeItemPhase::Correct) {
            log::info!("all finished!");

            let msg = match state.raw.jump_index {
                Some(index) => {
                    let index = index + 1; // bump for cover
                    log::info!("going to index {}!", index);
                    IframeAction::new(ModuleToJigPlayerMessage::JumpToIndex(index))
                }
                None => {
                    log::info!("going next!");
                    IframeAction::new(ModuleToJigPlayerMessage::Next)
                }
            };

            let _ = msg.try_post_message_to_top();
        }
    }
}


impl TalkTypeItem {
    pub fn evaluate(self: Rc<Self>, parent: Rc<TalkType>) {
        let state = self;

        match state.raw.texts.as_ref() {
            Some(texts) => {
                let value = &*state.value.lock_ref();

                if texts.iter().any(|text| text == value) {
                    state.phase.set(TalkTypeItemPhase::Correct);
                    state.base.audio_manager.play_positive_clip();
                    parent.evaluate_all();
                } else {
                    let hint_letters = &mut *state.hint_letters.borrow_mut();

                    match hint_letters.pop() {
                        Some(_) => {
                            // the newly revealed letter...
                        },
                        None => {
                            log::info!("out of hints!");
                        }
                    }

                    // not set_neq because we want it to always re-render
                    state.phase.set(TalkTypeItemPhase::Wrong);


                    state.base.audio_manager.play_negative_clip();
                }
            },
            None => {}
        };
        
    }

    pub fn play_audio(&self) {
        if let Some(audio_filename) = self.raw.audio_filename.as_ref() {
            let url = self.base.activity_media_url(audio_filename);
            self.base.audio_manager.play_clip(url);
        }

    }
}

impl HintLetters {
    fn pop(&mut self) -> Option<&str> {
        match self.indices.pop() {
            None => None,
            Some(index) => {
                let mut entry = &mut self.letters[index];
                entry.revealed = true;
                Some(&entry.letter)
            }
        }
    }
}