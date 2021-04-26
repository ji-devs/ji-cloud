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
            html!("module-sidebar-body", {
                .property("slot", "content")
                .children(&mut [
                    TextDom::render(state.clone()),
                    AudioDom::render(state.clone()),
                ])
            }),
        ]
    }
}

pub struct TextDom {}
impl TextDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("input-form-textarea", {
            .property_signal("value", state.instructions.text.signal_cloned().map(|text| {
                match text {
                    None => "".to_string(),
                    Some(text) => text
                }
            }))
            .property("label", crate::strings::STR_INSTRUCTIONS_LABEL)
            .property("placeholder", crate::strings::STR_INSTRUCTIONS_PLACEHOLDER)
            .property("rows", 4)
            //Input saves every character
            //Change also pushes history
            .event(clone!(state => move |evt:events::CustomInput| {
                state.change_instructions_text(evt.value(), false);
            }))
            .event(clone!(state => move |evt:events::CustomChange| {
                state.change_instructions_text(evt.value(), true);
            }))
        })
    }
}
pub struct AudioDom {}
impl AudioDom {
    pub fn render(state: Rc<State>) -> Dom {
        let opts = AudioInputOptions {
            on_change: Some(Box::new(clone!(state => move |audio_id| {
                state.change_instructions_audio(audio_id);
            }))),
            audio_id: state.instructions.audio.get_cloned().map(|audio| audio.id),
        };

        let audio_state = Rc::new(AudioState::new(opts)); 

        html!("empty-fragment", {
            .future(state.instructions.audio
                    .signal_cloned()
                    .to_stream()
                    .skip(1)
                    .for_each(clone!(audio_state => move |audio| {
                        //This just happens when history is changed really
                        audio_state.set_audio_id_ext(audio.map(|audio| audio.id));
                        async {}
                    }))
            )
            .child(render_audio_input(audio_state, None))
        })
    }
}
