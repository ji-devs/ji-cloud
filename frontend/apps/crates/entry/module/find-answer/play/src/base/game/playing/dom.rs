use components::{
    audio::mixer::{AudioSourceExt, AUDIO_MIXER},
    module::_common::play::prelude::ModulePlayPhase,
    traces::{
        bubble::TraceBubble,
        show::{TracesShow, TracesShowMode},
    },
};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{self, SignalVecExt},
};
use js_sys::Reflect;
use shared::domain::module::body::find_answer::QuestionField;
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};
use wasm_bindgen::JsValue;

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {
    html!("empty-fragment", {
        // We don't want the cursor to change when the student mouses over an answer trace, or even an incorrect trace.
        // Make the entire playable area use the pointer cursor.
        .style("cursor", "pointer")
        .future(state.game.base.module_phase.signal_cloned().dedupe().for_each(clone!(state => move |phase| {
            // Only play audio and update the text if we're in the playing phase.
            if let ModulePlayPhase::Playing = phase {
                // Play audio if we have any for this question
                if let Some(audio) = &state.question.question_audio {
                    AUDIO_MIXER.with(|mixer| {
                        mixer.play_oneshot(audio.as_source());
                    });
                }

                // Update the question sticker if it is set and the question has text
                if let QuestionField::Text(question_index) = state.game.base.question_field {
                    let sticker_ref = state.game.base.sticker_refs.get(question_index).unwrap_ji().get().unwrap_ji();

                    // This is weird. If we use "", then subsequent calls to set textValue don't work correctly.
                    let question_text = if state.question.question_text.is_empty() {
                        " "
                    } else {
                        &state.question.question_text
                    };

                    Reflect::set(
                        sticker_ref,
                        &JsValue::from_str("textValue"),
                        &JsValue::from_str(question_text)
                    ).unwrap_ji();
                }
            }

            async {}
        })))
        .child_signal(state.game.base.module_phase.signal_cloned().map(clone!(state => move |phase| {
            // Only render the banner if we're in the playing phase
            let should_render = matches!(phase, ModulePlayPhase::Playing)
                && !state.question.question_text.is_empty()
                && state.game.base.question_field.is_dynamic();
            if should_render {
                Some(html!("question-banner", {
                    .text(&state.question.question_text)
                }))
            } else {
                None
            }
        })))
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
