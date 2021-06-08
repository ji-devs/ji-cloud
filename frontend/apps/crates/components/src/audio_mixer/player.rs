use web_sys::AudioContext;
use awsm_web::audio::AudioPlayer as AwsmAudio;
use shared::{
    domain::audio::AudioId,
    media::MediaLibrary,
    domain::jig::module::body::Audio,
};
use utils::{prelude::*, path::audio_lib_url};
use std::rc::Rc;
use std::cell::RefCell;
use once_cell::unsync::OnceCell;
use std::sync::Mutex;

pub struct AudioPlayer {
    one_shot: Rc<RefCell<Option<AwsmAudio>>>
}

//One-shot audios drop themselves when finished
//to force early stop, need to manually take()
impl Drop for AudioPlayer {
    fn drop(&mut self) {
        log::info!("audio dropped!!");
        self.one_shot.borrow_mut().take();
    }
}

impl AudioPlayer {
    pub fn play_oneshot(ctx: &AudioContext, audio: Audio) -> Self {
        Self::play_oneshot_callback(ctx, audio, None as Option<Box<dyn FnMut()>>)
    }

    pub fn play_oneshot_callback(ctx: &AudioContext, audio: Audio, on_ended: Option<impl FnMut() + 'static>) -> Self {
        let url = audio_lib_url(audio.lib, audio.id);

        Self {
            one_shot: AwsmAudio::play_oneshot_url(ctx, &url, on_ended).unwrap_ji()
        }
    }
}
