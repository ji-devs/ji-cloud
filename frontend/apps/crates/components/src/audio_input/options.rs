use shared::{domain::audio::AudioId};

#[derive(Default)]
pub struct AudioInputOptions {
    pub audio_id: Option<AudioId>, //initial audio id
}
