use awsm_web::audio::AudioMixer as AwsmAudioMixer;
use shared::domain::jig::module::body::Audio;
use shared::domain::jig::Jig;
use std::cell::RefCell;
use std::rc::Rc;
use utils::{path::audio_lib_url, prelude::*};
use web_sys::AudioContext;

use std::ops::Deref;

pub use awsm_web::audio::{AudioClip, AudioClipOptions, AudioHandle, AudioSource, Id};

//inherently cloneable, conceptually like it's wrapped in Rc itself
#[derive(Clone)]
pub struct AudioMixer {
    inner: Rc<AwsmAudioMixer>,
    feedback: Rc<RefCell<Feedback>>,
}

pub struct Feedback {}

impl Feedback {
    pub fn new_from_jig(_jig: &Jig) -> Self {
        Self {}
    }
}

impl Default for Feedback {
    fn default() -> Self {
        Self {}
    }
}

impl AudioMixer {
    pub fn new(ctx: Option<AudioContext>) -> Self {
        //TODO - populate jig-level effects
        Self {
            inner: Rc::new(AwsmAudioMixer::new(ctx)),
            feedback: Rc::new(RefCell::new(Feedback::default())),
        }
    }

    pub fn set_from_jig(&self, jig: &Jig) {
        *self.feedback.borrow_mut() = Feedback::new_from_jig(jig);
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively
    pub fn play_oneshot<F>(&self, audio: Audio, on_ended: Option<F>) -> AudioClip
    where
        F: FnMut() -> () + 'static,
    {
        let url = audio_lib_url(audio.lib, audio.id);
        self.inner
            .play_oneshot(AudioSource::Url(url), on_ended)
            .unwrap_ji()
    }

    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play(&self, audio: Audio, is_loop: bool) -> AudioHandle {
        let url = audio_lib_url(audio.lib, audio.id);
        self.inner.play(AudioSource::Url(url), is_loop).unwrap_ji()
    }

    /// Add a source with various options and get a Handle to hold
    pub fn add_source<F>(&self, audio: Audio, options: AudioClipOptions<F>) -> AudioHandle
    where
        F: FnMut() -> () + 'static,
    {
        let url = audio_lib_url(audio.lib, audio.id);
        self.inner
            .add_source(AudioSource::Url(url), options)
            .unwrap_ji()
    }
}

impl Deref for AudioMixer {
    type Target = AwsmAudioMixer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
