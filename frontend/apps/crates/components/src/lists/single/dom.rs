
use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use super::state::*;
use crate::tooltip::{
    state::{State as TooltipState, TooltipTarget, TooltipData, TooltipError, MoveStrategy, Placement},
    callbacks::{TooltipErrorCallbacks, TooltipConfirmCallbacks}
};

pub fn render(state: Rc<State>) -> Dom { 

    html!("sidebar-widget-single-list", {
        .children(&mut [

            html!("button-rect", {
                .property("slot", "clear")
                .property("kind", "text")
                .text(super::strings::STR_CLEAR)
                .event(clone!(state => move |evt:events::Click| {
                    state.clear();
                }))
            }),
            // TODO:replace with new hebrew buttons
            html!("button-sidebar", {
                .property("slot", "input-buttons")
                .property("mode", "keyboard")
            }),
            html!("button-sidebar", {
                .property("slot", "input-buttons")
                .property("mode", "dicta")
            }),
            html!("button-sidebar", {
                .property("slot", "input-buttons")
                .property("mode", "sefaria")
            }),
            html!("button-rect", {
                // TODO: think should should be change from color to disabled
                .property_signal("color", state.is_valid_signal().map(|ready| {
                    if ready.is_ok() {
                        "red"
                    } else {
                        "grey"
                    }
                }))
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "done-btn")
                .text(super::strings::STR_DONE)
                .event(clone!(state => move |evt:events::Click| {
                    match state.derive_list() {
                        Ok(list) => {
                            (state.callbacks.replace_list) (list);
                        },
                        Err(err) => {
                            (state.callbacks.set_tooltip_error) (Some(
                                    Rc::new(TooltipState::new(
                                        TooltipTarget::Element(
                                            state.error_element_ref.borrow().as_ref().unwrap_ji().clone(),
                                            MoveStrategy::None
                                        ),

                                        TooltipData::Error(Rc::new(TooltipError {
                                            max_width: Some(185.0),
                                            placement: Placement::Right, 
                                            slot: None,
                                            body: super::strings::error::STR_NUM_WORDS.to_string(),
                                            callbacks: TooltipErrorCallbacks::new(
                                                Some(clone!(state => move || {
                                                    (state.callbacks.set_tooltip_error) (None); 
                                                }))
                                            )
                                        }))
                                    ))
                            ));
                        }
                    }
                }))
            })
        ])
        .children_signal_vec(
            state.list.signal_vec_cloned()
                .enumerate()
                .map(clone!(state => move |(index, value)| {

                        let index = index.get().unwrap_or_default();

                        html!("sidebar-widget-single-list-input", {
                            .property_signal("value", {
                                clone!(state => map_ref! {
                                    let value = value.signal_cloned(),
                                    let is_placeholder = state.is_placeholder.signal()
                                        => move {
                                            if *is_placeholder {
                                                (state.callbacks.get_placeholder) (index)
                                                    .unwrap_or_else(|| "".to_string())
                                            } else {
                                                value.clone()
                                            }
                                        }
                                })
                            })
                            .property("constrain", state.callbacks.constrain.as_ref())
                            .property_signal("placeholder", state.is_placeholder.signal())
                            .event(clone!(state => move |evt:events::Focus| {
                                //log::info!("got focus!");
                                state.is_placeholder.set_neq(false);
                            }))
                            .event(clone!(state => move |evt:events::CustomInput| {
                                value.set_neq(evt.value());
                            }))
                            .after_inserted(clone!(index, state => move |elem| {
                                if index == 2 {
                                    *state.error_element_ref.borrow_mut() = Some(elem);
                                }

                            }))
                        })
                }))
        )
    })
}
