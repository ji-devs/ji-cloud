use super::state::*;
use crate::audio::mixer::{AudioSourceExt, AUDIO_MIXER};
use crate::buttons::{Button, ButtonStyle, ButtonStyleIcon};
use dominator::{clone, html, Dom};
use std::rc::Rc;

impl AudioPlayerButton {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("empty-fragment", {
            .child_signal(state.handle.signal_ref(clone!(state => move |handle| {
                Some(if handle.is_some() {
                    let style = ButtonStyle::Icon(ButtonStyleIcon::AudioStop);
                    let button = Button::new(style, clone!(state => move || {
                        state.stop();
                    }));
                    Button::render(button, None)
                } else {
                    let style = ButtonStyle::Icon(ButtonStyleIcon::Audio);
                    let button = Button::new(style, clone!(state => move || {
                        AUDIO_MIXER.with(|mixer| {
                            state.handle.set(Some(
                                mixer.play_on_ended(
                                    state.audio.as_source(),
                                    false,
                                    clone!(state => move || {
                                        state.stop();
                                    })
                                )
                            ));
                        });
                    }));
                    Button::render(button, None)
                })
            })))
        })
    }
}
