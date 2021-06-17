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
use super::id::*;
use beach_map::{BeachMap, DefaultVersion};
use super::mixer::InstanceLookup;

pub struct AudioInstance {
    pub(super) id: Id,
    pub(super) instance_lookup: InstanceLookup,
    pub(super) clip: AudioClip 
}

pub(super) enum AudioClip {
    OneShot(Rc<RefCell<Option<AwsmAudio>>>)
}

impl AudioInstance {
    fn with_audio<A>(&self, f: impl FnOnce(&AwsmAudio) -> A) -> Option<A> {
        match &self.clip {
            AudioClip::OneShot(clip) => {
                if let Some(audio) = clip.borrow().as_ref() {
                    Some(f(audio))
                } else {
                    None
                }
            }
        }
    }
}
impl Drop for AudioInstance {
    fn drop(&mut self) {
        log::info!("audio instance dropped!!");
        self.instance_lookup.borrow_mut().remove(self.id);
        match &mut self.clip {
            AudioClip::OneShot(clip) => {
                //One-shot audios drop themselves when finished
                //to force early stop, need to manually take()
                clip.borrow_mut().take();
            }
        }
    }
}
