use web_sys::AudioContext;
use awsm_web::audio::AudioPlayer as AwsmAudio;
use shared::{
    domain::audio::AudioId,
    media::MediaLibrary,
};
use utils::{prelude::*, path::audio_lib_url};
use std::rc::Rc;
use std::cell::RefCell;
use once_cell::unsync::OnceCell;
use std::sync::Mutex;

pub struct AudioPlayer {
    audio: Rc<RefCell<Option<AwsmAudio>>>
}

impl AudioPlayer {
    pub fn play_oneshot(ctx: &AudioContext, lib: MediaLibrary, id: AudioId) -> Self {
        Self::play_oneshot_callback(ctx, lib, id, None as Option<Box<dyn FnMut()>>)
    }

    pub fn play_oneshot_callback(ctx: &AudioContext, lib: MediaLibrary, id: AudioId, on_ended: Option<impl FnMut() + 'static>) -> Self {
        let url = audio_lib_url(lib, id);

        log::info!("playing {}", url);

        Self {
            audio: AwsmAudio::play_oneshot_url(ctx, &url, on_ended).unwrap_ji()
        }
    }
}
