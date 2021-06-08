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

pub struct AudioInstance {
    pub(super) one_shot: Rc<RefCell<Option<AwsmAudio>>>
}


//One-shot audios drop themselves when finished
//to force early stop, need to manually take()
impl Drop for AudioInstance {
    fn drop(&mut self) {
        log::info!("audio dropped!!");
        self.one_shot.borrow_mut().take();
    }
}

