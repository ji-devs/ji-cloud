use super::{AudioHandleId, AudioMessageFromTop, AudioMessageToTop, PlayAudioMessage, AUDIO_MIXER};
use awsm_web::audio::AudioMixer as AwsmAudioMixer;
pub use awsm_web::audio::{
    AudioClip, AudioClipOptions, AudioHandle as AwsmWebAudioHandle, AudioSource, Id,
    WeakAudioHandle,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utils::prelude::*;

//inherently cloneable, conceptually like it's wrapped in Rc itself
#[derive(Clone)]
pub struct AudioMixerTop {
    pub(super) inner: Rc<AwsmAudioMixer>,
    pub(super) awsm_handles: RefCell<HashMap<AudioHandleId, AwsmWebAudioHandle>>,
}

impl AudioMixerTop {
    pub(super) fn new() -> Self {
        Self {
            inner: Rc::new(AwsmAudioMixer::new(None)),
            awsm_handles: Default::default(),
        }
    }

    pub(super) fn run_audio_message(&self, message: AudioMessageToTop) {
        match message {
            AudioMessageToTop::Play(play_message) => {
                let handle_id = play_message.handle_id.clone();
                self.play(play_message, move || {
                    AUDIO_MIXER.with(|mixer| mixer.done_playing(handle_id.clone()))
                });
            }
            AudioMessageToTop::PauseHandleCalled(handle_id) => {
                self.pause_handle_called(handle_id);
            }
            AudioMessageToTop::PlayHandleCalled(handle_id) => {
                self.play_handle_called(handle_id);
            }
            AudioMessageToTop::HandleDropped(handle_id) => {
                self.handle_dropped(handle_id);
            }
            AudioMessageToTop::PauseAll => {
                self.inner.pause_all();
            }
            AudioMessageToTop::PlayAll => {
                self.inner.play_all();
            }
            AudioMessageToTop::BroadcastContextAvailable => {
                self.broadcast_context_available_request();
            }
        }
    }

    fn play<F: FnMut() + 'static>(&self, audio_message: PlayAudioMessage, on_ended: F) {
        let awsm_handle = self
            .inner
            .play_on_ended(
                AudioSource::Url(audio_message.path.clone()),
                audio_message.is_loop,
                on_ended,
            )
            .unwrap_ji();
        self.awsm_handles
            .borrow_mut()
            .insert(audio_message.handle_id, awsm_handle);
    }

    fn handle_dropped(&self, handle_id: AudioHandleId) {
        let mut awsm_handles = self.awsm_handles.borrow_mut();
        awsm_handles.remove(&handle_id);
    }

    fn pause_handle_called(&self, handle_id: AudioHandleId) {
        let awsm_handles = self.awsm_handles.borrow();
        if let Some(audio) = awsm_handles.get(&handle_id) {
            audio.pause();
        }
    }

    fn play_handle_called(&self, handle_id: AudioHandleId) {
        let awsm_handles = self.awsm_handles.borrow();
        if let Some(audio) = awsm_handles.get(&handle_id) {
            audio.play();
        }
    }

    fn broadcast_context_available_request(&self) {
        let available = self.inner.context_available();
        AUDIO_MIXER.with(|mixer| {
            mixer.set_context_available(available);
            mixer.message_all_iframes(AudioMessageFromTop::ContextAvailable(available));
        });
    }
}
