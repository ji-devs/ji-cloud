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
use components::image_search::{
    dom::render as render_image_search,
    state::ImageSearchOptions,
};

pub struct WordsAndImagesDom {}
impl WordsAndImagesDom {
    pub fn render(state: Rc<State>) -> Vec<Dom> {
        vec![
            ImageSearchDom::render(state.clone()),
            html!("button-rect", {
                .property("color", "grey")
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "btn")
                .text(crate::strings::STR_DONE)
                .event(clone!(state => move |evt:events::Click| {
                    //state.replace_single_list(list_state.derive_list());
                    
                }))
            })
        ]
    }
}

pub struct ImageSearchDom {}
impl ImageSearchDom {
    pub fn render(state: Rc<State>) -> Dom {
        let opts = ImageSearchOptions {
            background_only: Some(false),
            upload: Some(()),
            filters: Some(()),
            value: Mutable::new(None),
        };
        render_image_search(opts, Some("content"))
    }
}
