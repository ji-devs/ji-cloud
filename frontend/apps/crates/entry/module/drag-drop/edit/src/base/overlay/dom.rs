use components::module::_common::edit::prelude::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use components::tooltip::dom::render as render_tooltip;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

impl DomRenderable for Overlay {
    fn render(state: Rc<Overlay>) -> Dom {
        html!("empty-fragment", {
        })
    }
}
