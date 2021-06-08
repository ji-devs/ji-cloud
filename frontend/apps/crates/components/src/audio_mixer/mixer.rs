use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::Jig;
use super::instance::*;
use awsm_web::audio::AudioPlayer as AwsmAudio;
use std::cell::RefCell;
use utils::{prelude::*, path::audio_lib_url};
use shared::{
    domain::audio::AudioId,
    media::MediaLibrary,
    domain::jig::module::body::Audio,
};

#[derive(Clone)]
pub struct AudioMixer {
    //All operations need to go through with_ctx
    //So that we can lazy-load it
    ctx: RefCell<Option<AudioContext>>
}

impl AudioMixer {
    pub fn new(ctx: Option<AudioContext>, jig: &Jig) -> Self {

        //TODO - populate jig-level effects
        Self {
            ctx: RefCell::new(ctx)
        }
    }
    pub fn new_without_jig(ctx: Option<AudioContext>) -> Self {

        Self {
            ctx: RefCell::new(ctx)
        }
    }

    //Lazy-creates the AudioContext as needed
    pub fn with_ctx<A>(&self, f: impl FnOnce(&AudioContext) -> A) -> A {
        let mut ctx = self.ctx.borrow_mut();
        if let Some(ctx) = ctx.as_ref() {
            f(ctx)
        } else {
            let new_ctx = AudioContext::new().unwrap_ji();
            let ret = f(&new_ctx);
            *ctx = Some(new_ctx);
            ret
        }
    }

    pub fn play_oneshot(&self, audio: Audio) -> AudioInstance {
        self.play_oneshot_callback(audio, None::<fn()>)
    }

    pub fn play_oneshot_callback(&self, audio: Audio, on_ended: Option<impl FnMut() + 'static>) -> AudioInstance {
        let url = audio_lib_url(audio.lib, audio.id);

        AudioInstance {
            one_shot: self.with_ctx(|ctx| AwsmAudio::play_oneshot_url(ctx, &url, on_ended).unwrap_ji())
        }
    }
}
