use dominator::{clone, html, Dom};
use std::rc::Rc;

use super::state::*;
use futures_signals::signal::SignalExt;

pub fn render_traces_edit(state: Rc<TracesEdit>) -> Dom {
    html!("empty-fragment", {
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            match phase {
                Phase::All => {
                    Some(super::all::dom::render_traces_all(state.clone()))
                },
                Phase::Draw(draw) => {
                    Some(super::draw::dom::render_traces_draw(draw.clone(), state.list.lock_ref()))
                }
            }
        })))
    })
}
