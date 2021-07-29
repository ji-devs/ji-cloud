use dominator::{Dom, html, clone};
use dominator::animation::MutableAnimation;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use web_sys::HtmlElement;
use shared::domain::jig::module::body::Instructions;
use crate::animation::fade::*;
use std::cell::RefCell;
use web_sys::AudioContext;
use crate::audio_mixer::{AudioMixer, AudioHandle};
use super::state::*;

pub fn render_instructions_player(state: Rc<InstructionsPlayer>, mixer:&AudioMixer) -> Dom {
    *state.audio.borrow_mut() = state.data.audio.as_ref().map(|audio| {
        mixer.play(audio.clone(), false)
    });
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

