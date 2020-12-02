use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use crate::data::*;
use dominator::{Dom, clone, DomBuilder, events, apply_methods};
use dominator_helpers::{elem, with_data_id};
use futures_signals::signal::SignalExt;
use crate::templates;

pub fn render(state:Rc<State>) -> Dom {

    unsafe {
        elem!(
            templates::header("Create a Cover Page", "Introduce your topic<br/>Use the blue panel for selecting layouts, themes, and adding content")
        , {})
    }

}
 
