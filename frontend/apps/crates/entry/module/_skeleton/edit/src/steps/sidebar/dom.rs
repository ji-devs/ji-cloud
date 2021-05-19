use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use futures_signals::signal::SignalExt;

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("div", {
            .text_signal(state.base.step.signal().map(|step| {
                step.label()
            }))
        })
    }
}
