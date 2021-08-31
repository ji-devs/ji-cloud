use dominator::{clone, html, Dom};
use std::rc::Rc;

use super::state::*;
use futures_signals::signal::SignalExt;

impl TracesEdit {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("empty-fragment", {
            .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
                match phase {
                    TracesEditPhase::Selectable => {
                        Some(TracesEdit::render_selectable(state.clone()))
                    },
                    TracesEditPhase::Draw(draw) => {
                        Some(TracesEdit::render_draw(state.clone(), draw.clone()))
                    }
                }
            })))
        })
    }
}
