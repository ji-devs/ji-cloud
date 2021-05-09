use dominator::{html, Dom, clone};
use crate::data::state::State as AppState;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use components::{
    image_search::{
        dom::render as render_image_search,
        state::ImageSearchOptions,
    },
    color_select::{self, state::ColorSelectConfig},
};

use super::state::*;

pub struct Step2Dom {}
impl Step2Dom {
    pub fn render(app: Rc<AppState>) -> Dom {
        let state = Rc::new(State::new(app));

        html!("menu-tabs", {
            .property("slot", "content")
            .children(&mut [
                render_tab(state.clone(), Tab::Image),
                render_tab(state.clone(), Tab::Color),
                render_tab(state.clone(), Tab::Overlay),
                html!("module-sidebar-body", {
                    .property("slot", "body")
                    .child_signal(state.tab.signal().map(clone!(state => move |tab| {
                        match tab {
                            Tab::Image => {
                                Some(ImageSearchDom::render(state.clone()))
                            },
                            Tab::Color => {
                                Some(ColorPickerDom::render(state.clone()))
                            },
                            Tab::Overlay => {
                                Some(ImageSearchDom::render(state.clone()))
                            },
                            _ => None,
                        }
                    })))
                })
            ])
        })
    }
}


fn render_tab(state: Rc<State>, tab:Tab) -> Dom {
    html!("menu-tab", {
        .property("slot", "tabs")
        .property_signal("active", state.tab.signal_ref(clone!(tab => move |curr| {
            *curr == tab 
        })))
        .child(html!("menu-tab-title", {
            .property("kind", tab.as_str())
        }))
        .event(clone!(state, tab => move |evt:events::Click| {
            state.tab.set_neq(tab)
        }))
    })
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
        render_image_search(opts, None) 
    }
}


pub struct ColorPickerDom {}
impl ColorPickerDom {
    pub fn render(state: Rc<State>) -> Dom {
        color_select::dom::render(ColorSelectConfig {
            theme: Some(state.app.theme_id.get()),
            value: Rc::new(Mutable::new(None))
        }, None)
    }
}
