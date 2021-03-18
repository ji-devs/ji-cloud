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

                        html!("sidebar-widget-single-list-input", {
                            .property_signal("value", {
                                map_ref! {
                                    let value = value.signal_cloned(),
                                    let is_placeholder = state.is_placeholder.signal()
                                        => move {
                                            if *is_placeholder {
                                                get_placeholder_value(index)
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

fn get_placeholder_value(index: usize) -> String {
    match index {
        0 => crate::strings::input_list::STR_0,
        1 => crate::strings::input_list::STR_1,
        2 => crate::strings::input_list::STR_2,
        3 => crate::strings::input_list::STR_3,
        4 => crate::strings::input_list::STR_4,
        5 => crate::strings::input_list::STR_5,
        _ => ""
    }.to_string()
}
/*
    <sidebar-widget-single-list slot="input-widget">
    ${mapToString(arrayCount(nRows), row => {

        const value = row < 6 
            ? placeholder ? "placeholder='placeholder'" : "value='value'"
            : "";

        return`<sidebar-widget-single-list-input ${value}></sidebar-widget-single-list-input>`
    })}
    </sidebar-widget-single-list>`
    */
