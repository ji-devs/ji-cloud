use components::traces::{
    bubble::TraceBubble,
    show::{TracesShow, TracesShowMode},
};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{self, SignalVecExt},
};
use js_sys::Reflect;
use shared::domain::module::body::find_answer::QuestionField;
use wasm_bindgen::JsValue;
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {
    html!("empty-fragment", {
        .apply_if(state.game.base.question_field.is_dynamic(), |dom| {
            dom.child(html!("question-banner", {
                .text(&state.question.question_text)
            }))
        })
        .apply_if(!state.game.base.question_field.is_dynamic(), |dom| {
            if let QuestionField::Text(question_index) = state.game.base.question_field {
                let sticker_ref = state.game.base.sticker_refs.get(question_index).unwrap_ji().get().unwrap_ji();
                Reflect::set(
                    sticker_ref,
                    &JsValue::from_str("textValue"),
                    &JsValue::from_str(&state.question.question_text)
                ).unwrap_ji();
            }

            dom
        })
        .child(TracesShow::render(TracesShow::new(
                state.traces
                    .iter()
                    .map(|t| t.inner.clone())
                    .collect(),
                TracesShowMode::HiddenSolidMap(state.selected_set.clone()),
                Some(clone!(state => move |index| {
                    PlayState::select(state.clone(), index);
                }))
        )))
        .child(html!("overlay-container", {
            .children_signal_vec(
                resize_info_signal()
                    .switch_signal_vec(clone!(state => move |_resize_info| {
                        signal_vec::always(state.traces.clone())
                            .map_signal(|trace| {
                                trace.phase.signal_cloned()
                            })
                            .map(|phase| {
                                match phase {
                                    PlayPhase::Playing(bubble) => {
                                        Some(TraceBubble::render(bubble))
                                    },
                                    _ => None
                                }
                            })
                            .filter(|x| x.is_some())
                            .map(|x| x.unwrap_ji())
                    }))
            )
        }))

    })
}
