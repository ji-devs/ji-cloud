use dominator::{clone, html, Dom};
use std::rc::Rc;

use super::state::*;
use futures_signals::signal::SignalExt;

pub fn render_traces_edit(state: Rc<TracesEdit>) -> Dom {
    html!("empty-fragment", {
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            match phase {
                Phase::Selectable => {
                    Some(TracesEdit::render_selectable(state.clone()))
                },
                Phase::Draw(draw) => {
                    Some(TracesEdit::render_draw(state.clone(), draw.clone()))
                }
            }
        })))
    })
}
