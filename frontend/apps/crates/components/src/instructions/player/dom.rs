use dominator::{html, Dom, clone};

use std::rc::Rc;
use utils::prelude::*;

use super::state::*;
use crate::audio::mixer::{AUDIO_MIXER, AudioPath, AudioSourceExt};

impl InstructionsPlayer {
    pub fn render(state: Rc<Self>) -> Dom {

        *state.audio.borrow_mut() = state
            .data
            .audio
            .as_ref()
            .map(|audio| AUDIO_MIXER.with(|mixer| mixer.play(audio.as_source(), false)));

        html!("empty-fragment", {
            .apply_if(state.data.text.is_some(), |dom| {
                let text = state.data.text.as_ref().unwrap_ji();

                state.fade.render(dom.child(
                    html!("instructions-banner", {
                        .text(text)
                    })
                ))

            })
            .after_removed(clone!(state => move |elem| {
                *state.audio.borrow_mut() = None;
            }))
        })
    }
}
