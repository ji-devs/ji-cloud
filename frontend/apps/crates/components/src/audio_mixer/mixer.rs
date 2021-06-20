use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::Jig;
use awsm_web::audio::AudioMixer as AwsmAudioMixer;
use std::cell::RefCell;
use utils::{prelude::*, path::audio_lib_url};
use shared::{
    domain::audio::AudioId,
    media::MediaLibrary,
    domain::jig::module::body::Audio,
};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use std::ops::Deref;

pub use awsm_web::audio::{Id, AudioHandle, AudioSource, AudioClipOptions, AudioClip};

#[derive(Clone)]
pub struct AudioMixer {
    inner: Rc<AwsmAudioMixer>,
    jig: Rc<Jig>
}

impl AudioMixer {
    pub fn new(ctx: Option<AudioContext>, jig: &Jig) -> Self {

        //TODO - populate jig-level effects
        Self {
            inner: Rc::new(AwsmAudioMixer::new(ctx)),
            jig: Rc::new(jig.clone()),
        }
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively 
    pub fn play_oneshot<F>(&self, audio: Audio, on_ended: Option<F>) -> AudioClip
    where
        F: FnMut() -> () + 'static,

    {
        let url = audio_lib_url(audio.lib, audio.id);
        self.inner.play_oneshot(AudioSource::Url(url), on_ended).unwrap_ji()
    }

    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play(&self, audio: Audio, is_loop: bool) -> AudioHandle
    {
        let url = audio_lib_url(audio.lib, audio.id);
        self.inner.play(AudioSource::Url(url), is_loop).unwrap_ji()
    }

    /// Add a source with various options and get a Handle to hold
    pub fn add_source<F>(&self, audio: Audio, options: AudioClipOptions<F>) -> AudioHandle
    where
        F: FnMut() -> () + 'static,

    {
        let url = audio_lib_url(audio.lib, audio.id);
        self.inner.add_source(AudioSource::Url(url), options).unwrap_ji()
    }
}

impl Deref for AudioMixer {
    type Target = AwsmAudioMixer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
