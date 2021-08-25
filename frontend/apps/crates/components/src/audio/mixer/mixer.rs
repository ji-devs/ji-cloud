use awsm_web::audio::AudioMixer as AwsmAudioMixer;
use shared::domain::jig::module::body::Audio;
use shared::domain::jig::{Jig, self};
use std::cell::RefCell;
use std::rc::Rc;
use utils::{path, prelude::*};
use web_sys::AudioContext;
use std::borrow::Cow;
use std::ops::Deref;

pub use awsm_web::audio::{AudioClip, AudioClipOptions, AudioHandle, AudioSource, Id};

//inherently cloneable, conceptually like it's wrapped in Rc itself
#[derive(Clone)]
pub struct AudioMixer {
    inner: Rc<AwsmAudioMixer>,
    settings: Rc<RefCell<AudioSettings>>,
}

pub struct AudioSettings {
    pub bg: jig::AudioBackground,
}

impl AudioSettings {
    pub fn new_from_jig(jig:&Jig) -> Self {
        Self {
            //TODO...
            ..Self::default() 
        }
    }

    pub fn bg_source(&self) -> impl Into<AudioSource> {
        let path:AudioPath = self.bg.clone().into();
        path
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            bg: jig::AudioBackground::FunForKids
        }
    }
}

pub enum AudioPath<'a> {
    Lib(Audio),
    Cdn(Cow<'a, str>)
}

impl AudioPath<'_> {
    pub fn new_cdn(cdn_path:String) -> Self {
        Self::Cdn(Cow::Owned(cdn_path))
    }
}

impl <'a> From<AudioPath<'a>> for AudioSource {
    fn from(audio_path:AudioPath) -> Self {
        match audio_path {
            AudioPath::Lib(audio) => AudioSource::Url(path::audio_lib_url(audio.lib, audio.id)),
            AudioPath::Cdn(cdn_path) => AudioSource::Url(path::audio_cdn_url(cdn_path)),
        }
    }
}


impl From<jig::AudioBackground> for AudioPath<'_> {
    fn from(bg:jig::AudioBackground) -> Self {
        Self::Cdn(Cow::Borrowed(match bg {
            jig::AudioBackground::FunForKids => "music-loop/fun-for-kids.mp3",
            jig::AudioBackground::SukkotLoop => "music-loop/sukkot-loop.mp3",
        }))
    }
}

impl From<jig::AudioFeedbackPositive> for AudioPath<'_> {
    fn from(p:jig::AudioFeedbackPositive) -> Self {
        Self::Cdn(Cow::Borrowed(match p {
            jig::AudioFeedbackPositive::Magic => "module/feedback-positive/magic.mp3",
            jig::AudioFeedbackPositive::StarPing => "module/feedback-positive/star-ping.mp3",
            jig::AudioFeedbackPositive::Ting => "module/feedback-positive/ting.mp3",
        }))
    }
}

impl From<jig::AudioFeedbackNegative> for AudioPath<'_> {
    fn from(n:jig::AudioFeedbackNegative) -> Self {
        Self::Cdn(Cow::Borrowed(match n {
            jig::AudioFeedbackNegative::Boing => "module/feedback-negative/boing.mp3",
            jig::AudioFeedbackNegative::Incorrect => "module/feedback-negative/incorrect.mp3",
            jig::AudioFeedbackNegative::JumpWrong => "module/feedback-negative/jump-wrong.mp3",
        }))
    }
}

impl AudioMixer {
    pub fn new(ctx: Option<AudioContext>) -> Self {
        //TODO - populate jig-level effects
        Self {
            inner: Rc::new(AwsmAudioMixer::new(ctx)),
            settings: Rc::new(RefCell::new(AudioSettings::default())),
        }
    }

    pub fn set_from_jig(&self, jig: &Jig) {
        *self.settings.borrow_mut() = AudioSettings::new_from_jig(jig);
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively
    pub fn play_oneshot<F, A: Into<AudioSource>>(&self, audio: A, on_ended: Option<F>) -> AudioClip
    where
        F: FnMut() -> () + 'static,
    {
        self.inner
            .play_oneshot(audio.into(), on_ended)
            .unwrap_ji()
    }

    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play<A: Into<AudioSource>>(&self, audio: A, is_loop: bool) -> AudioHandle {
        self.inner.play(audio.into(), is_loop).unwrap_ji()
    }

    /// Add a source with various options and get a Handle to hold
    pub fn add_source<F, A: Into<AudioSource>>(&self, audio: A, options: AudioClipOptions<F>) -> AudioHandle
    where
        F: FnMut() -> () + 'static,
    {
        self.inner
            .add_source(audio.into(), options)
            .unwrap_ji()
    }
}

impl Deref for AudioMixer {
    type Target = AwsmAudioMixer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
