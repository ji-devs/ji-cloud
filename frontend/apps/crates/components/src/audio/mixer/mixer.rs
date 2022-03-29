use awsm_web::audio::AudioMixer as AwsmAudioMixer;
use rand::prelude::*;
use shared::domain::jig::module::body::Audio;
use shared::domain::jig::{self, AudioFeedbackNegative, AudioFeedbackPositive, JigData};
use std::cell::RefCell;
use std::rc::Rc;
use utils::{path, prelude::*};

use std::borrow::Cow;
use std::ops::Deref;

pub use awsm_web::audio::{
    AudioClip, AudioClipOptions, AudioHandle, AudioSource, Id, WeakAudioHandle,
};

thread_local! {
    pub static AUDIO_MIXER:AudioMixer = AudioMixer {
        inner: Rc::new(AwsmAudioMixer::new(None)),
        settings: Rc::new(RefCell::new(AudioSettings::default())),
        rng: RefCell::new(thread_rng()),
    }
}

//inherently cloneable, conceptually like it's wrapped in Rc itself
#[derive(Clone)]
pub struct AudioMixer {
    inner: Rc<AwsmAudioMixer>,
    settings: Rc<RefCell<AudioSettings>>,
    rng: RefCell<ThreadRng>,
}

pub struct AudioSettings {
    pub bg: jig::AudioBackground,
    pub positive: Vec<AudioFeedbackPositive>,
    pub negative: Vec<AudioFeedbackNegative>,
}

impl AudioSettings {
    pub fn reset_from_jig(&mut self, jig: &JigData) {
        // If there is no positive or negative effects configured on the jig, reset the clips to
        // the list returned by `variants()`. Alternatively we could just **not** reset at all, but
        // there could potentially be a case where the the audio was reset for a jig with audio effects
        // and then calling reset_from_jig again would not reset to the defaults.

        self.positive = {
            if !jig.audio_effects.feedback_positive.is_empty() {
                jig.audio_effects
                    .feedback_positive
                    .clone()
                    .into_iter()
                    .collect()
            } else {
                AudioFeedbackPositive::variants()
            }
        };

        self.negative = {
            if !jig.audio_effects.feedback_negative.is_empty() {
                jig.audio_effects
                    .feedback_negative
                    .clone()
                    .into_iter()
                    .collect()
            } else {
                AudioFeedbackNegative::variants()
            }
        };
    }

    pub fn bg_source(&self) -> impl Into<AudioSource> {
        let path: AudioPath = self.bg.into();
        path
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            bg: jig::AudioBackground::FunForKids,
            positive: AudioFeedbackPositive::variants(),
            negative: AudioFeedbackNegative::variants(),
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

impl<'a, F: 'a> AudioSourceExt<'a> for F
where
    AudioPath<'a>: From<&'a F>,
{
    fn as_source(&'a self) -> AudioSource {
        let path: AudioPath = self.into();
        path.into()
    }
}

pub enum AudioPath<'a> {
    Lib(Cow<'a, Audio>),
    Cdn(Cow<'a, str>),
}

impl AudioPath<'_> {
    pub fn new_cdn(cdn_path: String) -> Self {
        Self::Cdn(Cow::Owned(cdn_path))
    }

    pub fn url(&self) -> String {
        match self {
            Self::Lib(audio) => path::audio_lib_url(audio.lib, audio.id),
            Self::Cdn(cdn_path) => path::audio_cdn_url(cdn_path),
        }
    }
}

impl<'a> From<AudioPath<'a>> for AudioSource {
    fn from(audio_path: AudioPath) -> Self {
        AudioSource::Url(audio_path.url())
    }
}

impl From<Audio> for AudioPath<'_> {
    fn from(audio: Audio) -> Self {
        Self::Lib(Cow::Owned(audio))
    }
}
impl<'a> From<&'a Audio> for AudioPath<'a> {
    fn from(audio: &'a Audio) -> Self {
        Self::Lib(Cow::Borrowed(audio))
    }
}

//TODO - make it nicer to implement both ref and owned with macros
impl From<&jig::AudioBackground> for AudioPath<'_> {
    fn from(bg: &jig::AudioBackground) -> Self {
        (*bg).into()
    }
}
impl From<jig::AudioBackground> for AudioPath<'_> {
    fn from(bg: jig::AudioBackground) -> Self {
        Self::Cdn(Cow::Borrowed(match bg {
            jig::AudioBackground::FunForKids => "music-loop/fun-for-kids.mp3",
            jig::AudioBackground::DancingHappy => "music-loop/dancing-happy.mp3",
            jig::AudioBackground::Jigzi1 => "music-loop/jigzi1.mp3",
            jig::AudioBackground::Jigzi2 => "music-loop/jigzi2.mp3",
            jig::AudioBackground::Jigzi3 => "music-loop/jigzi3.mp3",
            jig::AudioBackground::LegacyCuckooToYou => "music-loop/cuckoo-to-you.mp3",
            jig::AudioBackground::LegacyFirstEtude => "music-loop/first-etude.mp3",
            jig::AudioBackground::LegacyHanerotHalalu => "music-loop/hanerot-halalu.mp3",
            jig::AudioBackground::LegacyIslandRomp => "music-loop/island-romp.mp3",
            jig::AudioBackground::LegacyJiTap => "music-loop/ji-tap.mp3",
            jig::AudioBackground::LegacyMaozTzur => "music-loop/maoz-tzur.mp3",
            jig::AudioBackground::LegacyModehAni => "music-loop/modeh-ani.mp3",
            jig::AudioBackground::LegacyMonkeyBars => "music-loop/monkey-bars.mp3",
            jig::AudioBackground::LegacyMorningZoo => "music-loop/morning-zoo.mp3",
            jig::AudioBackground::LegacyNapTime => "music-loop/nap-time.mp3",
            jig::AudioBackground::LegacyPlaylandMarch => "music-loop/playland-march.mp3",
            jig::AudioBackground::LegacyShehechiyanu => "music-loop/shehechiyanu.mp3",
            jig::AudioBackground::LegacySunAndNoClouds => "music-loop/sun-and-no-clouds.mp3",
            jig::AudioBackground::LegacyTeddysBear => "music-loop/teddys-bear.mp3",
            jig::AudioBackground::LegacyWanderingWalrus => "music-loop/wandering-walrus.mp3",
            jig::AudioBackground::LegacyWindupLullaby => "music-loop/windup-lullaby.mp3",
        }))
    }
}

impl From<&jig::AudioFeedbackPositive> for AudioPath<'_> {
    fn from(p: &jig::AudioFeedbackPositive) -> Self {
        (*p).into()
    }
}

impl From<jig::AudioFeedbackPositive> for AudioPath<'_> {
    fn from(p: jig::AudioFeedbackPositive) -> Self {
        Self::Cdn(Cow::Borrowed(match p {
            jig::AudioFeedbackPositive::Correct => "module/feedback-positive/correct.mp3",
            jig::AudioFeedbackPositive::Keys => "module/feedback-positive/keys.mp3",
            jig::AudioFeedbackPositive::Magic => "module/feedback-positive/magic.mp3",
            jig::AudioFeedbackPositive::Notes => "module/feedback-positive/notes.mp3",
            jig::AudioFeedbackPositive::StarPing => "module/feedback-positive/star-ping.mp3",
            jig::AudioFeedbackPositive::Ting => "module/feedback-positive/ting.mp3",
            jig::AudioFeedbackPositive::Trumpet => "module/feedback-positive/trumpet.mp3",
            jig::AudioFeedbackPositive::VoiceAwesome => {
                "module/feedback-positive/voice-awesome.mp3"
            }
            jig::AudioFeedbackPositive::VoicesHurray => {
                "module/feedback-positive/voices-hurray.mp3"
            }
            jig::AudioFeedbackPositive::VoiceYippee => "module/feedback-positive/voice-yippee.mp3",
            jig::AudioFeedbackPositive::Xylophone => "module/feedback-positive/xylophone.mp3",
            jig::AudioFeedbackPositive::Yes => "module/feedback-positive/yes.mp3",
        }))
    }
}

impl From<&jig::AudioFeedbackNegative> for AudioPath<'_> {
    fn from(n: &jig::AudioFeedbackNegative) -> Self {
        (*n).into()
    }
}
impl From<jig::AudioFeedbackNegative> for AudioPath<'_> {
    fn from(n: jig::AudioFeedbackNegative) -> Self {
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
        let mut settings = (*self.settings).borrow_mut();
        settings.reset_from_jig(jig);
    }

    pub fn get_random_positive(&self) -> AudioFeedbackPositive {
        let settings = self.settings.borrow();
        let effects = settings.positive.iter();
        let chosen_effect = effects.choose(&mut *self.rng.borrow_mut());
        *chosen_effect.unwrap_ji()
    }

    pub fn get_random_negative(&self) -> AudioFeedbackNegative {
        let settings = self.settings.borrow();
        let effects = settings.negative.iter();
        let chosen_effect = effects.choose(&mut *self.rng.borrow_mut());
        *chosen_effect.unwrap_ji()
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively
    pub fn play_oneshot<A: Into<AudioSource>>(&self, audio: A) -> WeakAudioHandle {
        self.inner.play_oneshot(audio.into()).unwrap_ji()
    }

    pub fn play_oneshot_on_ended<F, A>(&self, audio: A, on_ended: F) -> WeakAudioHandle
    where
        F: FnMut() + 'static,
        A: Into<AudioSource>,
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
        F: FnMut() + 'static,
        A: Into<AudioSource>,
    {
        self.inner
            .play_on_ended(audio.into(), is_loop, on_ended)
            .unwrap_ji()
    }

    /// Add a source with various options and get a Handle to hold
    pub fn add_source<F, A: Into<AudioSource>>(
        &self,
        audio: A,
        options: AudioClipOptions<F>,
    ) -> AudioHandle
    where
        F: FnMut() + 'static,
    {
        self.inner.add_source(audio.into(), options).unwrap_ji()
    }
}

impl Deref for AudioMixer {
    type Target = AwsmAudioMixer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Utility function to play a random positive audio effect.
pub fn play_random_positive() {
    AUDIO_MIXER.with(|mixer| {
        let path: AudioPath<'_> = mixer.get_random_positive().into();
        mixer.play_oneshot(path)
    });
}

/// Utility function to play a random negative audio effect.
pub fn play_random_negative() {
    AUDIO_MIXER.with(|mixer| {
        let path: AudioPath<'_> = mixer.get_random_negative().into();
        mixer.play_oneshot(path)
    });
}
