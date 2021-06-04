use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt
};
use components::{
    instructions::editor::dom::render as render_instructions,
};

pub fn render(state: Rc<Step3>) -> Dom {

    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(render_instructions(state.instructions_editor.clone()))
    })
}
