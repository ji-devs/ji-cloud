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
    options::AudioInputOptions,
    state::State as AudioState,
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
        let opts = AudioInputOptions {
            on_change: Some(clone!(state => move |audio_id| {
                state.change_instructions_audio(audio_id);
            })),
            audio_id: state.instructions.audio_id.get_cloned(),
        };

        let audio_state = Rc::new(AudioState::new(opts)); 

        html!("empty-fragment", {
            .future(state.instructions.audio_id
                    .signal()
                    .to_stream()
                    .skip(1)
                    .for_each(clone!(audio_state => move |audio_id| {
                        //This just happens when history is changed really
                        audio_state.set_audio_id_ext(audio_id);
                        async {}
                    }))
            )
            .property("slot", "content")
            .child(render_audio_input(audio_state, None))
        })
    }
}
