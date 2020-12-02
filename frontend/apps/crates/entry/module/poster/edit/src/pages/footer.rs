use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use crate::data::*;
use dominator::{Dom, clone, DomBuilder, events, apply_methods};
use dominator_helpers::{elem, with_data_id};
use futures_signals::signal::SignalExt;
use crate::templates;


pub fn render(state:Rc<State>) -> Dom {
    elem!(templates::footer(), {
        .with_data_id!("next", {
            .event(clone!(state => move |evt:events::Click| {
                let curr_tool = state.tool.get();
                match curr_tool {
                    Tool::Layout => {
                        state.tool.set(Tool::Images);
                    },
                    _ => {}
                }
            }))
        })
    })
}
 
