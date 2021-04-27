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
use components::instructions::editor::InstructionsEditor;

use futures::stream::StreamExt;

pub struct Step3Dom {}
impl Step3Dom {
    pub fn render(state: Rc<State>) -> Vec<Dom> {
        vec![
            html!("module-sidebar-body", {
                .property("slot", "content")
                .child({
                    let editor = InstructionsEditor::new(state.instructions.clone(), Box::new(clone!(state => move |instructions, also_history| {
                        state.save_instructions(instructions, also_history);
                    })));

                    editor.render()
                })
            }),
        ]
    }
}
