use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use components::tooltip::dom::render as render_tooltip;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt}
};

impl DomRenderable for Overlay {
    fn render(state: Rc<Overlay>) -> Dom {
        let sig = map_ref! {
            let delete = state.base.tooltips.delete.signal_cloned(),
            let list_error = state.base.tooltips.list_error.signal_cloned()
            => {
                let mut children:Vec<Dom> = Vec::new();
                if let Some(delete) = delete.as_ref() {
                    children.push(render_tooltip(delete.clone()));
                }
                if let Some(list_error) = list_error.as_ref() {
                    children.push(render_tooltip(list_error.clone()));
                }

                children
            }
        };

        html!("empty-fragment", {
            .children_signal_vec(sig.to_signal_vec())
        })
    }
}
