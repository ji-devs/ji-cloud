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
    instructions::editor::InstructionsEditor,
    text_editor::dom::render_controls as render_text_editor_controls,
};
use super::state::*;

pub struct Step3Dom {}
impl Step3Dom {
    pub fn render(app: Rc<AppState>) -> Dom {
        let state = Rc::new(State::new(app));

        html!("menu-tabs", {
            .property("slot", "content")
            .children(&mut [
                render_tab(state.clone(), Tab::Text),
                render_tab(state.clone(), Tab::Image),
                render_tab(state.clone(), Tab::Audio),
                html!("module-sidebar-body", {
                    .property("slot", "body")
                    .child_signal(state.tab.signal().map(clone!(state => move |tab| {
                        match tab {
                            Tab::Image => {
                                Some(ImageSearchDom::render(state.clone()))
                            },
                            Tab::Audio=> {
                                Some(AudioDom::render(state.clone()))
                            },
                            Tab::Text => {
                                Some(TextEditorDom::render(state.clone()))
                            },
                            _ => None
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


pub struct TextEditorDom {}
impl TextEditorDom {
    pub fn render(state: Rc<State>) -> Dom {
        render_text_editor_controls(state.app.text_editor.clone())
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

pub struct AudioDom {}
impl AudioDom {
    pub fn render(state: Rc<State>) -> Dom {
        let state = state.app.clone();

        let editor = InstructionsEditor::new(state.instructions.clone(), Box::new(clone!(state => move |instructions, also_history| {
            state.save_instructions(instructions, also_history);
        })));

        editor.render_audio()
    }
}

