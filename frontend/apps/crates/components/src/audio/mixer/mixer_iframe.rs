use utils::{
    iframe::IframeMessageExt, js_wrappers::set_event_listener, prelude::IframeAction,
    unwrap::UnwrapJiExt,
};
use web_sys::MessageEvent;

use crate::audio::mixer::AUDIO_MIXER;

use super::{AudioMessageFromTop, AudioMessageToTop};

// AudioMixerIframe is just a transparent message passer
pub struct AudioMixerIframe;

impl AudioMixerIframe {
    pub fn new() -> Self {
        setup_parent_to_iframe_listener();
        Self
    }

    pub(super) fn run_audio_message(&self, message: AudioMessageToTop) {
        let message = IframeAction::new(message);
        if let Err(err) = message.try_post_message_to_parent() {
            todo!("{:?}", err);
        }
    }
}

fn setup_parent_to_iframe_listener() {
    let window = web_sys::window().unwrap_ji();
    set_event_listener(
        &window,
        "message",
        Box::new(|evt: MessageEvent| {
            if let Ok(m) =
                serde_wasm_bindgen::from_value::<IframeAction<AudioMessageFromTop>>(evt.data())
            {
                match m.data {
                    AudioMessageFromTop::DonePlaying(handle) => {
                        AUDIO_MIXER.with(|mixer| {
                            mixer.done_playing(handle);
                        });
                    }
                    AudioMessageFromTop::ContextAvailable(available) => {
                        AUDIO_MIXER.with(|mixer| {
                            mixer.set_context_available(available);

                            // forward the message to inner iframes
                            mixer.message_all_iframes(AudioMessageFromTop::ContextAvailable(
                                available,
                            ));
                        });
                    }
                }
            };
        }),
    );
}
