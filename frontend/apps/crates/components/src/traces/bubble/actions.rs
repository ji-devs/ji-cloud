use super::{state::TraceBubble, EndPolicy};
use crate::audio::mixer::{AudioSourceExt, AUDIO_MIXER};
use dominator::clone;
use std::rc::Rc;
use std::sync::atomic::Ordering;

impl TraceBubble {
    pub fn play_audio(state: Rc<Self>) {
        *state.audio_handle.borrow_mut() = state.audio.as_ref().map(|audio| {
            AUDIO_MIXER.with(|mixer| {
                mixer.play_on_ended(
                    audio.as_source(),
                    false,
                    clone!(state => move || {
                        state.on_audio_ended();
                    }),
                )
            })
        });
    }

    pub fn on_audio_ended(&self) {
        self.audio_ended.store(true, Ordering::SeqCst);
        self.evaluate_all_ended();
    }

    pub fn on_fade_ended(&self) {
        self.fade_ended.store(true, Ordering::SeqCst);
        self.evaluate_all_ended();
    }

    pub fn reset_ended(&self) {
        self.fade_ended
            .store(self.tooltip.is_none(), Ordering::SeqCst);
        self.audio_ended
            .store(self.audio.is_none(), Ordering::SeqCst);
    }
    pub fn evaluate_all_ended(&self) {
        if !self.dispatched_ended.load(Ordering::SeqCst) {
            let fade_ended = self.fade_ended.load(Ordering::SeqCst);
            let audio_ended = self.audio_ended.load(Ordering::SeqCst);

            let policy = self.end_policy.get();

            let ended = {
                match policy {
                    EndPolicy::Any => fade_ended || audio_ended,
                    EndPolicy::All => fade_ended && audio_ended,
                    EndPolicy::AudioThenFade => {
                        if self.audio.is_none() {
                            fade_ended
                        } else {
                            audio_ended
                        }
                    }
                }
            };

            if ended {
                self.dispatched_ended.store(true, Ordering::SeqCst);
                if let Some(on_ended) = self.on_ended.as_ref() {
                    (on_ended)();
                }
            }
        }
    }
}
