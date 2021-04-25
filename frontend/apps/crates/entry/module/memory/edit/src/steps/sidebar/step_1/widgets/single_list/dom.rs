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
use components::tooltip::types::*;

pub struct SingleListDom {}
impl SingleListDom {
    pub fn render(state: Rc<State>) -> Dom { 

        html!("sidebar-widget-single-list", {
            .children(&mut [

                html!("button-text", {
                    .property("slot", "clear")
                    .text(crate::strings::STR_CLEAR)
                    .event(clone!(state => move |evt:events::Click| {
                        state.clear();
                    }))
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
                        match state.derive_list() {
                            Ok(list) => {
                                state.app.replace_single_list(list);
                            },
                            Err(err) => {
                                log::info!("why not rendering...");
                                state.app.overlay.tooltips.list_error.set(Some(
                                    TooltipData::Error(TooltipError {
                                        elem: state.error_element_ref.borrow().as_ref().unwrap_ji().clone(), 
                                        max_width: Some(185.0),
                                        placement: Placement::Right, 
                                        slot: None,
                                        body: crate::strings::error::STR_SINGLE_LIST_NUM_WORDS.to_string(),
                                        on_close: Some(Rc::new(Box::new(clone!(state => move || {
                                            state.app.overlay.tooltips.list_error.set(None); 
                                        })))),
                                    })
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
                        //couldn't get it to compile by moving this into an Rc at a higher level
                        //but closures should be cheap in a JS VM anyway :P
                        let constrain_cb = Closure::wrap(Box::new(clone!(state => move |text:String| {
                            state.app.limit_text(crate::config::SINGLE_LIST_CHAR_LIMIT, text)
                        })) as Box<dyn FnMut(String) -> String>);

                        Dom::with_state(constrain_cb, clone!(state => move |constrain_cb| {
                            let index = index.get().unwrap_or_default();
                            let mode = state.app.mode.get_cloned().unwrap_ji();

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
                                .property("constrain", constrain_cb.as_ref())
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
                    }))
            )
        })
    }
}
