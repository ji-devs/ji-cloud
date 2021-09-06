use super::state::InstructionsPlayer;
use std::sync::atomic::Ordering;
use std::rc::Rc;
use dominator::clone;
use crate::audio::mixer::{AUDIO_MIXER, AudioPath, AudioSourceExt};

impl InstructionsPlayer {

    pub fn play_audio(state: Rc<Self>) {
        *state.audio.borrow_mut() = state
            .data
            .audio
            .as_ref()
            .map(|audio| AUDIO_MIXER.with(|mixer| {
                mixer.play_on_ended(audio.as_source(), false, clone!(state => move || {
                    state.on_audio_ended();
                }))
            }));
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
        self.fade_ended.store(self.data.text.is_none(), Ordering::SeqCst);
        self.audio_ended.store(self.data.audio.is_none(), Ordering::SeqCst);
    }
    pub fn evaluate_all_ended(&self) {
        let fade_ended = self.fade_ended.load(Ordering::SeqCst);
        let audio_ended = self.audio_ended.load(Ordering::SeqCst);

        if fade_ended && audio_ended {
            if let Some(on_ended) = self.on_ended.as_ref() {
                (on_ended) ();
            }
        }
    }
}