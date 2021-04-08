use dominator::{html, Dom, clone};
use crate::data::state::State as AppState;
use super::state::*;
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
use crate::steps::sidebar::step_1::widgets::single_list::{
    state::State as ListState,
    dom::SingleListDom,
};

pub struct WordsAndImagesDom {}
impl WordsAndImagesDom {
    pub fn render(app: Rc<AppState>, is_empty: bool) -> Dom {
        let state = Rc::new(State::new(app));

        html!("menu-tabs", {
            .property("slot", "content")
            .children(&mut [
                html!("menu-tab", {
                    .property("slot", "tabs")
                    .property_signal("active", state.tab.signal_ref(|tab| {
                        *tab == Tab::Text
                    }))
                    .child(html!("menu-tab-title", {
                        .property("kind", "text")
                    }))
                    .event(clone!(state => move |evt:events::Click| {
                        state.tab.set_neq(Tab::Text)
                    }))
                }),
                html!("menu-tab", {
                    .property("slot", "tabs")
                    .property_signal("active", state.tab.signal_ref(|tab| {
                        *tab == Tab::Images
                    }))
                    .child(html!("menu-tab-title", {
                        .property("kind", "image")
                    }))
                    .event(clone!(state => move |evt:events::Click| {
                        state.tab.set_neq(Tab::Images)
                    }))
                }),
                html!("module-sidebar-body", {
                    .property("slot", "body")
                    .child_signal(state.tab.signal().map(clone!(state, is_empty => move |tab| {
                        Some(match tab {
                            Tab::Text => {
                                if is_empty {
                                    html!("step1-sidebar-empty")
                                } else {
                                    TextInputDom::render(state.clone())
                                }
                            },
                            Tab::Images => ImageSearchDom::render(state.clone()),
                        })
                    })))
                })
            ])
        })
    }
}
pub struct TextInputDom {}

impl TextInputDom {
    pub fn render(state: Rc<State>) -> Dom {
        let list_state = Rc::new(ListState::new(state.app.clone(), 14));
        SingleListDom::render(list_state.clone())
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
        render_image_search(opts, None) 
    }
}
