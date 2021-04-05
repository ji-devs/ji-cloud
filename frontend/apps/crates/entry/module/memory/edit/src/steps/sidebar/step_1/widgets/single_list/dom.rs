use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::events;
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
            .property("slot", "input-widget")
            .children_signal_vec(
                state.list.signal_vec_cloned()
                    .enumerate()
                    .map(clone!(state => move |(index, value)| {
                        let index = index.get().unwrap_or_default();
                        let mode = state.mode;

                        html!("sidebar-widget-single-list-input", {
                            .property_signal("value", {
                                map_ref! {
                                    let value = value.signal_cloned(),
                                    let is_placeholder = state.is_placeholder.signal()
                                        => move {
                                            if *is_placeholder {
                                                match crate::config::get_init_word_ref(mode, index) {
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
