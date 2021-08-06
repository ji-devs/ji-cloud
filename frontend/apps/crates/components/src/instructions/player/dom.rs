use dominator::{html, Dom};

use std::rc::Rc;
use utils::prelude::*;

use super::state::*;
use crate::audio_mixer::AudioMixer;

pub fn render_instructions_player(state: Rc<InstructionsPlayer>, mixer: &AudioMixer) -> Dom {
    *state.audio.borrow_mut() = state
        .data
        .audio
        .as_ref()
        .map(|audio| mixer.play(audio.clone(), false));
    html!("empty-fragment", {
        .apply_if(state.data.text.is_some(), |dom| {
            let text = state.data.text.as_ref().unwrap_ji();

            state.fade.render(dom.child(
                html!("instructions-banner", {
                    .text(text)
                })
            ))

        })
    })
}
