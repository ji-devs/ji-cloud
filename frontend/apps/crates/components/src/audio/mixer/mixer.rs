use awsm_web::audio::AudioMixer as AwsmAudioMixer;
use shared::domain::jig::module::body::Audio;
use shared::domain::jig::{JigData, self};
use std::cell::RefCell;
use std::rc::Rc;
use utils::{path, prelude::*};
use web_sys::AudioContext;
use std::borrow::Cow;
use std::ops::Deref;
use once_cell::unsync::Lazy;
pub use awsm_web::audio::{AudioClip, AudioClipOptions, AudioHandle, AudioSource, Id};
thread_local! {
    pub static AUDIO_MIXER:AudioMixer = AudioMixer {
        inner: Rc::new(AwsmAudioMixer::new(None)),
        settings: Rc::new(RefCell::new(AudioSettings::default())),
    }
}

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
    pub fn new_from_jig(jig:&JigData) -> Self {
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

//This is a bit confusing...
//but basically it lets us call .as_source()
//on Audio, AudioBackground, etc.
//it does it by way of the intermediate Into<AudioPath> impl
pub trait AudioSourceExt<'a> {
    fn as_source(&'a self) -> AudioSource;
}

impl <'a, F: 'a> AudioSourceExt<'a> for F 
where
    AudioPath<'a>: From<&'a F>
{
    fn as_source(&'a self) -> AudioSource {
        let path:AudioPath = self.into();
        path.into()
    }
}


pub enum AudioPath<'a> {
    Lib(Cow<'a, Audio>),
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

impl From<Audio> for AudioPath<'_> {
    fn from(audio:Audio) -> Self {
        Self::Lib(Cow::Owned(audio))
    }
}
impl <'a> From<&'a Audio> for AudioPath<'a> {
    fn from(audio:&'a Audio) -> Self {
        Self::Lib(Cow::Borrowed(audio))
    }
}

//TODO - make it nicer to implement both ref and owned with macros
impl From<&jig::AudioBackground> for AudioPath<'_> {
    fn from(bg:&jig::AudioBackground) -> Self {
        (*bg).into()
    }
}
impl From<jig::AudioBackground> for AudioPath<'_> {
    fn from(bg:jig::AudioBackground) -> Self {
        Self::Cdn(Cow::Borrowed(match bg {
            jig::AudioBackground::FunForKids => "music-loop/fun-for-kids.mp3",
            jig::AudioBackground::DancingHappy => "music-loop/dancing-happy.mp3",
            jig::AudioBackground::Jigzi1 => "music-loop/jigzi1.mp3",
            jig::AudioBackground::Jigzi2 => "music-loop/jigzi2.mp3",
        }))
    }
}

impl From<&jig::AudioFeedbackPositive> for AudioPath<'_> {
    fn from(p:&jig::AudioFeedbackPositive) -> Self {
        (*p).into()
    }
}
impl From<jig::AudioFeedbackPositive> for AudioPath<'_> {
    fn from(p:jig::AudioFeedbackPositive) -> Self {
        Self::Cdn(Cow::Borrowed(match p {
            jig::AudioFeedbackPositive::Correct => "module/feedback-positive/correct.mp3",
            jig::AudioFeedbackPositive::Keys => "module/feedback-positive/keys.mp3",
            jig::AudioFeedbackPositive::Magic => "module/feedback-positive/magic.mp3",
            jig::AudioFeedbackPositive::Notes => "module/feedback-positive/notes.mp3",
            jig::AudioFeedbackPositive::StarPing => "module/feedback-positive/star-ping.mp3",
            jig::AudioFeedbackPositive::Ting => "module/feedback-positive/ting.mp3",
            jig::AudioFeedbackPositive::Trumpet => "module/feedback-positive/trumpet.mp3",
            jig::AudioFeedbackPositive::VoiceAwesome => "module/feedback-positive/voice-awesome.mp3",
            jig::AudioFeedbackPositive::VoicesHurray => "module/feedback-positive/voices-hurray.mp3",
            jig::AudioFeedbackPositive::VoiceYippee => "module/feedback-positive/voice-yippee.mp3",
            jig::AudioFeedbackPositive::Xylophone => "module/feedback-positive/xylophone.mp3",
            jig::AudioFeedbackPositive::Yes => "module/feedback-positive/yes.mp3",
        }))
    }
}

impl From<&jig::AudioFeedbackNegative> for AudioPath<'_> {
    fn from(n:&jig::AudioFeedbackNegative) -> Self {
        (*n).into()
    }
}
impl From<jig::AudioFeedbackNegative> for AudioPath<'_> {
    fn from(n:jig::AudioFeedbackNegative) -> Self {
        Self::Cdn(Cow::Borrowed(match n {
            jig::AudioFeedbackNegative::Bang => "module/feedback-negative/bang.mp3",
            jig::AudioFeedbackNegative::Boing => "module/feedback-negative/boing.mp3",
            jig::AudioFeedbackNegative::Buzz => "module/feedback-negative/buzz.mp3",
            jig::AudioFeedbackNegative::Buzzer => "module/feedback-negative/buzzer.mp3",
            jig::AudioFeedbackNegative::Clang => "module/feedback-negative/clang.mp3",
            jig::AudioFeedbackNegative::Clicks => "module/feedback-negative/clicks.mp3",
            jig::AudioFeedbackNegative::Incorrect => "module/feedback-negative/incorrect.mp3",
            jig::AudioFeedbackNegative::JumpWrong => "module/feedback-negative/jump-wrong.mp3",
            jig::AudioFeedbackNegative::NotRight => "module/feedback-negative/not-right.mp3",
            jig::AudioFeedbackNegative::OhNo => "module/feedback-negative/oh-no.mp3",
            jig::AudioFeedbackNegative::ShortClang => "module/feedback-negative/short-clang.mp3",
            jig::AudioFeedbackNegative::Whir => "module/feedback-negative/whir.mp3",
        }))
    }
}

impl AudioMixer {

    pub fn set_from_jig(&self, jig: &JigData) {
        *self.settings.borrow_mut() = AudioSettings::new_from_jig(jig);
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively
    pub fn play_oneshot<A: Into<AudioSource>>(&self, audio: A) -> AudioClip
    {
        self.inner
            .play_oneshot(audio.into())
            .unwrap_ji()
    }

    pub fn play_oneshot_on_ended<F, A>(&self, audio: A, on_ended: F) -> AudioClip
    where
        F: FnMut() -> () + 'static,
        A: Into<AudioSource>
    {
        self.inner
            .play_oneshot_on_ended(audio.into(), on_ended)
            .unwrap_ji()
    }

    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play<A: Into<AudioSource>>(&self, audio: A, is_loop: bool) -> AudioHandle {
        self.inner.play(audio.into(), is_loop).unwrap_ji()
    }

    pub fn play_on_ended<F, A>(&self, audio: A, is_loop: bool, on_ended: F) -> AudioHandle 
    where
        F: FnMut() -> () + 'static,
        A: Into<AudioSource>
    {
        self.inner.play_on_ended(audio.into(), is_loop, on_ended).unwrap_ji()
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
