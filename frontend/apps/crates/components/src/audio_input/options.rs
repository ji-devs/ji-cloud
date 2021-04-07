use shared::{domain::audio::AudioId, media::MediaLibrary};
pub struct AudioInputOptions <F: Fn(Option<AudioId>)> {
    pub on_change: Option<F>,
    pub audio_id: Option<AudioId>, //initial audio id
}
