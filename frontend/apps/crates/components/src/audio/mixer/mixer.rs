use dominator::{clone, DomBuilder};
use gloo_timers::callback::Timeout;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use shared::domain::jig::{self, AudioFeedbackNegative, AudioFeedbackPositive, JigData};
use shared::domain::module::body::Audio;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::js_wrappers::{is_iframe, set_event_listener};
use utils::{path, prelude::*};
use web_sys::{HtmlIFrameElement, MessageEvent};

use super::{mixer_iframe::AudioMixerIframe, mixer_top::AudioMixerTop};

thread_local! {
    pub static AUDIO_MIXER:AudioMixer = AudioMixer::new()
}

pub enum AudioMixerKind {
    Top(AudioMixerTop),
    Iframe(AudioMixerIframe),
}

fn setup_iframe_to_parent_listener() {
    let window = web_sys::window().unwrap_ji();
    set_event_listener(
        &window,
        "message",
        Box::new(|evt: MessageEvent| {
            if let Ok(m) =
                serde_wasm_bindgen::from_value::<IframeAction<AudioMessageToTop>>(evt.data())
            {
                AUDIO_MIXER.with(|mixer| {
                    mixer.run_audio_message(m.data);
                })
            };
        }),
    );
}

// Might make sense to only have one kind on each entry and use conditional compilation to set correct kind.
// But the compiler might be doing that already.
pub struct AudioMixer {
    kind: AudioMixerKind,
    settings: Rc<RefCell<AudioSettings>>,
    rng: RefCell<ThreadRng>,
    callbacks: RefCell<HashMap<AudioHandleId, Box<dyn FnMut()>>>,
    context_available: RefCell<bool>,

    // having a channel would probably be better here, but wasted to much time on this already
    iframes: RefCell<Vec<HtmlIFrameElement>>,
}

/// Public interface, designed to be as close as possible to old interface
impl AudioMixer {
    pub fn context_available(&self) -> bool {
        *self.context_available.borrow()
    }

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
    pub fn play_oneshot(&self, path: AudioPath) {
        let handle_id = AudioHandleId::new();
        let audio_message = PlayAudioMessage {
            handle_id: handle_id.clone(),
            url: path.url(),
            auto_play: true,
            is_loop: false,
        };
        self.stash_callback_and_play(audio_message, move || {
            AUDIO_MIXER.with(clone!(handle_id => move |mixer| {
                // mixer.audio_handle_dropped(handle_id.clone());
                mixer.run_audio_message(AudioMessageToTop::HandleDropped(handle_id.clone()));
            }));
        });
    }

    pub fn play_oneshot_on_ended<F>(&self, path: AudioPath, mut on_ended: F)
    where
        F: FnMut() + 'static,
    {
        let handle_id = AudioHandleId::new();
        let audio_message = PlayAudioMessage {
            handle_id: handle_id.clone(),
            url: path.url(),
            auto_play: true,
            is_loop: false,
        };
        self.stash_callback_and_play(
            audio_message,
            clone!(handle_id => move || {
                AUDIO_MIXER.with(clone!(handle_id => move |mixer| {
                    mixer.run_audio_message(AudioMessageToTop::HandleDropped(handle_id.clone()));
                }));
                (on_ended)();
            }),
        );
    }

    pub fn play(&self, path: AudioPath, is_loop: bool) -> AudioHandle {
        let handle = AudioHandle::new();
        let audio_message = PlayAudioMessage {
            handle_id: handle.id().clone(),
            url: path.url(),
            auto_play: true,
            is_loop,
        };
        self.stash_callback_and_play(audio_message, || {});
        handle
    }

    pub fn play_on_ended<F>(&self, path: AudioPath, is_loop: bool, on_ended: F) -> AudioHandle
    where
        F: FnMut() + 'static,
    {
        let handle = AudioHandle::new();
        let audio_message = PlayAudioMessage {
            handle_id: handle.id().clone(),
            url: path.url(),
            auto_play: true,
            is_loop,
        };
        self.stash_callback_and_play(audio_message, on_ended);
        handle
    }

    pub fn play_all(&self) {
        self.run_audio_message(AudioMessageToTop::PlayAll);
    }

    pub fn pause_all(&self) {
        self.run_audio_message(AudioMessageToTop::PauseAll);
    }
}

/// Private methods
impl AudioMixer {
    fn new() -> Self {
        setup_iframe_to_parent_listener();

        // once initialized broadcast context available
        // don't think this is the best way of doing this, but this is what I can come up with at the moment
        Timeout::new(0, move || {
            AUDIO_MIXER.with(|mixer| {
                mixer.run_audio_message(AudioMessageToTop::BroadcastContextAvailable);
            })
        })
        .forget();

        Self {
            kind: match use_iframe_audio() {
                true => AudioMixerKind::Iframe(AudioMixerIframe::new()),
                false => AudioMixerKind::Top(AudioMixerTop::new()),
            },
            callbacks: Default::default(),
            context_available: RefCell::new(false), // corresponds to AudioMixerTop.audio_context.is_some()
            settings: Default::default(),
            rng: RefCell::new(thread_rng()),
            iframes: Default::default(),
        }
    }

    fn stash_callback_and_play<F>(&self, audio_message: PlayAudioMessage, on_ended: F)
    where
        F: FnMut() + 'static,
    {
        let id = audio_message.handle_id.clone();
        self.run_audio_message(AudioMessageToTop::Play(audio_message));

        let mut callbacks = self.callbacks.borrow_mut();
        callbacks.insert(id.clone(), Box::new(on_ended));
    }

    pub(super) fn done_playing(&self, audio_handle_id: AudioHandleId) {
        let callback = self.callbacks.borrow_mut().remove(&audio_handle_id);

        match callback {
            Some(mut callback) => {
                (callback)();
            }
            None => {
                let message = AudioMessageFromTop::DonePlaying(audio_handle_id.clone());
                self.message_all_iframes(message);
            }
        }
    }

    pub(super) fn message_all_iframes(&self, message: AudioMessageFromTop) {
        self.iframes.borrow().iter().for_each(move |iframe| {
            let message = IframeAction::new(message.clone());

            // might be a good idea to add iframe IDs in all messages and only send the the relevant iframe
            let _ = iframe
                .content_window()
                .unwrap_ji()
                .post_message(&message.into(), "*");
        });
    }

    // would probably have named this handle_audio_message, but can make it look like it's related to AudioHandle
    pub(super) fn run_audio_message(&self, message: AudioMessageToTop) {
        match &self.kind {
            AudioMixerKind::Top(top) => {
                top.run_audio_message(message);
            }
            AudioMixerKind::Iframe(iframe) => {
                iframe.run_audio_message(message);
            }
        };
    }

    pub(super) fn set_context_available(&self, available: bool) {
        *self.context_available.borrow_mut() = available;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(super) struct AudioHandleId(String);

impl AudioHandleId {
    pub fn new() -> Self {
        Self(js_sys::Math::random().to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AudioHandle(AudioHandleId);

impl AudioHandle {
    pub fn new() -> Self {
        Self(AudioHandleId::new())
    }
    pub(super) fn id(&self) -> AudioHandleId {
        self.0.clone()
    }
    pub fn pause(&self) {
        AUDIO_MIXER.with(|mixer| {
            mixer.run_audio_message(AudioMessageToTop::PauseHandleCalled(self.id()));
        });
    }
    pub fn play(&self) {
        AUDIO_MIXER.with(|mixer| {
            mixer.run_audio_message(AudioMessageToTop::PlayHandleCalled(self.id()));
        });
    }
}

impl Drop for AudioHandle {
    fn drop(&mut self) {
        AUDIO_MIXER.with(|mixer| {
            // mixer.audio_handle_dropped(self.id().clone());
            mixer.run_audio_message(AudioMessageToTop::HandleDropped(self.id().clone()));
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) enum AudioMessageToTop {
    Play(PlayAudioMessage),
    PauseHandleCalled(AudioHandleId),
    PlayHandleCalled(AudioHandleId),
    PauseAll,
    PlayAll,
    HandleDropped(AudioHandleId),

    /// ask top to broadcast if the context is available
    BroadcastContextAvailable,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct PlayAudioMessage {
    pub url: String,
    pub auto_play: bool,
    pub is_loop: bool,
    pub handle_id: AudioHandleId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) enum AudioMessageFromTop {
    DonePlaying(AudioHandleId),
    ContextAvailable(bool),
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

pub fn audio_iframe_messenger(dom: DomBuilder<HtmlIFrameElement>) -> DomBuilder<HtmlIFrameElement> {
    dom.after_inserted(move |iframe| {
        AUDIO_MIXER.with(|mixer| {
            mixer.iframes.borrow_mut().push(iframe);
        })
    })
    .after_removed(move |iframe| {
        AUDIO_MIXER.with(|mixer| {
            let mut iframes = mixer.iframes.borrow_mut();
            let pos = iframes.iter().position(|i| i == &iframe);
            if let Some(pos) = pos {
                iframes.swap_remove(pos);
            }
        })
    })
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

pub enum AudioPath<'a> {
    Lib(Cow<'a, Audio>),
    Cdn(Cow<'a, str>),
    Url(Cow<'a, str>),
}

impl AudioPath<'_> {
    pub fn new_cdn(cdn_path: String) -> Self {
        Self::Cdn(Cow::Owned(cdn_path))
    }

    pub fn new_url(url: String) -> Self {
        Self::Url(Cow::Owned(url))
    }

    pub fn url(&self) -> String {
        match self {
            Self::Lib(audio) => path::audio_lib_url(audio.lib, audio.id),
            Self::Cdn(cdn_path) => path::audio_cdn_url(cdn_path),
            Self::Url(url) => url.to_string(),
        }
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
            jig::AudioBackground::Awestruck => "music-loop/awestruck.mp3",
            jig::AudioBackground::BayBounce => "music-loop/bay-bounce.mp3",
            jig::AudioBackground::CalmAndReflective => "music-loop/calm-and-reflective.mp3",
            jig::AudioBackground::DayWithoutRain => "music-loop/day-without-rain.mp3",
            jig::AudioBackground::DestinationFreedom => "music-loop/destination-freedom.mp3",
            jig::AudioBackground::FutureMemories => "music-loop/future-memories.mp3",
            jig::AudioBackground::HappyInstrumental => "music-loop/happy-instrumental.mp3",
            jig::AudioBackground::HappyWhistle => "music-loop/happy-whistle.mp3",
            jig::AudioBackground::KidsInstrumental => "music-loop/kids-instrumental.mp3",
            jig::AudioBackground::PartyKids => "music-loop/party-kids.mp3",
            jig::AudioBackground::RhythmKids => "music-loop/rhythm-kids.mp3",
            jig::AudioBackground::SunKissed => "music-loop/sun-kissed.mp3",
            jig::AudioBackground::LegacyCuckooToYou => "music-loop/legacy-cuckoo-to-you.mp3",
            jig::AudioBackground::LegacyFirstEtude => "music-loop/legacy-first-etude.mp3",
            jig::AudioBackground::LegacyHanerotHalalu => "music-loop/legacy-hanerot-halalu.mp3",
            jig::AudioBackground::LegacyIslandRomp => "music-loop/legacy-island-romp.mp3",
            jig::AudioBackground::LegacyJiTap => "music-loop/legacy-ji-tap.mp3",
            jig::AudioBackground::LegacyMaozTzur => "music-loop/legacy-maoz-tzur.mp3",
            jig::AudioBackground::LegacyModehAni => "music-loop/legacy-modeh-ani.mp3",
            jig::AudioBackground::LegacyMonkeyBars => "music-loop/legacy-monkey-bars.mp3",
            jig::AudioBackground::LegacyMorningZoo => "music-loop/legacy-morning-zoo.mp3",
            jig::AudioBackground::LegacyNapTime => "music-loop/legacy-nap-time.mp3",
            jig::AudioBackground::LegacyPlaylandMarch => "music-loop/legacy-playland-march.mp3",
            jig::AudioBackground::LegacyShehechiyanu => "music-loop/legacy-shehechiyanu.mp3",
            jig::AudioBackground::LegacySunAndNoClouds => "music-loop/legacy-sun-and-no-clouds.mp3",
            jig::AudioBackground::LegacyTeddysBear => "music-loop/legacy-teddys-bear.mp3",
            jig::AudioBackground::LegacyWanderingWalrus => "music-loop/legacy-wandering-walrus.mp3",
            jig::AudioBackground::LegacyWindupLullaby => "music-loop/legacy-windup-lullaby.mp3",
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

fn use_iframe_audio() -> bool {
    // if local and is top window don't use iframe
    if cfg!(feature = "local") && !is_iframe() {
        return false;
    }
    cfg!(feature = "iframe_audio")
}
