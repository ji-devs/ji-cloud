use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use components::audio_input::{
    dom::render as render_audio_input,
    state::{State as AudioState, AudioInputOptions},
};
use futures::stream::StreamExt;

pub struct Step3Dom {}
impl Step3Dom {
    pub fn render(state: Rc<State>) -> Vec<Dom> {
        vec![
            AudioDom::render(state.clone()),
            html!("button-rect", {
                .property("color", "grey")
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "btn")
                .text(crate::strings::STR_DONE)
                .event(clone!(state => move |evt:events::Click| {
                    //state.replace_single_list(list_state.derive_list());
                    
                }))
            })
        ]
    }
}

pub struct AudioDom {}
impl AudioDom {
    pub fn render(state: Rc<State>) -> Dom {
        let audio_state = Rc::new(AudioState::new(AudioInputOptions {
            on_change: Some(clone!(state => move |audio_id| {
                state.change_instructions_audio(audio_id);
            })),
            audio_id: Some(state.instructions.audio_id.clone())
        }));

        render_audio_input(audio_state, Some("content"))
    }
}
