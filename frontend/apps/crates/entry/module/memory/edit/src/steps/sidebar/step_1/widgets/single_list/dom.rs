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

pub struct SingleListDom {}
impl SingleListDom {
    pub fn render(state: Rc<State>) -> Dom { 
        html!("sidebar-widget-single-list", {
            .children(&mut [

                html!("button-text", {
                    .property("slot", "clear")
                    .text(crate::strings::STR_CLEAR)
                }),
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
                    .property("color", "grey")
                    .property("size", "small")
                    .property("iconAfter", "done")
                    .property("slot", "done-btn")
                    .text(crate::strings::STR_DONE)
                    .event(clone!(state => move |evt:events::Click| {
                        state.app.replace_single_list(state.derive_list());
                    }))
                }),
            ])
            .children_signal_vec(
                state.list.signal_vec_cloned()
                    .enumerate()
                    .map(clone!(state => move |(index, value)| {
                        let index = index.get().unwrap_or_default();
                        let mode = state.app.game_mode.get_cloned().unwrap_ji();

                        html!("sidebar-widget-single-list-input", {
                            .property_signal("value", {
                                map_ref! {
                                    let value = value.signal_cloned(),
                                    let is_placeholder = state.is_placeholder.signal()
                                        => move {
                                            if *is_placeholder {
                                                match crate::config::get_single_list_init_word(index) {
                                                    Some(s) => s.to_string(),
                                                    None => "".to_string()
                                                }
                                            } else {
                                                value.clone()
                                            }
                                        }
                                }
                            })
                            .property_signal("placeholder", state.is_placeholder.signal())
                            .event(clone!(state => move |evt:events::Focus| {
                                //log::info!("got focus!");
                                state.is_placeholder.set_neq(false);
                            }))
                            .event(clone!(state => move |evt:events::CustomInput| {
                                value.set_neq(evt.value());
                            }))
                        })
                    }))
            )
        })
    }
}
