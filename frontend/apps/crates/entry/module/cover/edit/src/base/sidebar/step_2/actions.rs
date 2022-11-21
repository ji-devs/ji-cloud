use crate::base::state::*;
use dominator::clone;
use shared::domain::module::body::Audio;

impl Base {
    pub fn set_audio(&self, audio: Option<Audio>) {
        self.audio.set(audio.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.audio = audio;
            }
        });
    }
}
