use std::{collections::HashMap, cell::RefCell};

use awsm_web::audio::AudioSource;
use shared::domain::jig::{AudioFeedbackPositive, AudioFeedbackNegative};
use utils::{prelude::{IframeAction, ModuleToJigPlayerMessage}, iframe::IframeMessageExt};

use crate::audio::mixer::AUDIO_MIXER;

thread_local! {
    pub static IFRAME_AUDIO_MIXER:IframeAudioMixer = IframeAudioMixer {
        callbacks: RefCell::new(HashMap::new()),
    }
}

// TODO: should I newtype random_id

pub struct IframeAudioMixer {
    /// map of random ids with callbacks to call on ended
    callbacks: RefCell<HashMap<String, Box<dyn FnMut()>>>
}

impl IframeAudioMixer {
    pub fn done_playing(&self, random_id: String) {
        let callback = self.callbacks.borrow_mut().remove(&random_id);
        if let Some(mut callback) = callback {
            (callback)();
        }
    }

    pub fn get_random_positive(&self) -> AudioFeedbackPositive {
        AUDIO_MIXER.with(|mixer| mixer.get_random_positive())
    }

    pub fn get_random_negative(&self) -> AudioFeedbackNegative {
        AUDIO_MIXER.with(|mixer| mixer.get_random_negative())
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively
    pub fn play_oneshot<A: Into<AudioSource>>(&self, audio: A) {
        let source: AudioSource = audio.into();
        let audio_path = match source {
            AudioSource::Url(audio_path) => audio_path,
            AudioSource::Buffer(_) => todo!(),
        };
        self.send_play_message(audio_path, false, String::new());
    }

    pub fn play_oneshot_on_ended<F, A>(&self, audio: A, on_ended: F)
    where
        F: FnMut() + 'static,
        A: Into<AudioSource>,
    {
        let random_id = js_sys::Math::random().to_string();
        let source: AudioSource = audio.into();
        let audio_path = match source {
            AudioSource::Url(audio_path) => audio_path,
            AudioSource::Buffer(_) => todo!(),
        };
        let mut callbacks = self.callbacks.borrow_mut();
        callbacks.insert(random_id.clone(), Box::new(on_ended));

        self.send_play_message(audio_path, false, random_id);
    }

    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play<A: Into<AudioSource>>(&self, audio: A, is_loop: bool) {
        let source: AudioSource = audio.into();
        let audio_path = match source {
            AudioSource::Url(audio_path) => audio_path,
            AudioSource::Buffer(_) => todo!(),
        };
        self.send_play_message(audio_path, is_loop, String::new());

    }

    pub fn play_on_ended<F, A>(&self, audio: A, is_loop: bool, on_ended: F)
    where
        F: FnMut() + 'static,
        A: Into<AudioSource>,
    {
        let random_id = js_sys::Math::random().to_string();
        let source: AudioSource = audio.into();
        let audio_path = match source {
            AudioSource::Url(audio_path) => audio_path,
            AudioSource::Buffer(_) => todo!(),
        };
        let mut callbacks = self.callbacks.borrow_mut();
        callbacks.insert(random_id.clone(), Box::new(on_ended));

        self.send_play_message(audio_path, is_loop, random_id);
    }

    // /// Add a source with various options and get a Handle to hold
    // pub fn add_source<F, A: Into<AudioSource>>(
    //     &self,
    //     audio: A,
    //     options: AudioClipOptions<F>,
    // ) -> AudioHandle
    // where
    //     F: FnMut() + 'static,
    // {
    //     self.inner.add_source(audio.into(), options).unwrap_ji()
    // }

    fn send_play_message(&self, audio_path: String, is_loop: bool, random_id: String)  {
        let msg = IframeAction::new(
            ModuleToJigPlayerMessage::PlayAudio {
                audio_path,
                is_loop,
                random_id: random_id.clone(),
            },
        );
        if let Err(err) = msg.try_post_message_to_player() {
            todo!("{:?}", err);
        }
    }
}
