use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds,
    module::_common::edit::prelude::*,
    stickers::dom::{render_stickers, render_stickers_raw},
    traces::{bubble::TraceBubble, edit::TracesEdit},
};
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .children_signal_vec(
                state.phase_signal().map(clone!(state => move |phase| {
                    match phase {
                        Phase::Layout => {
                            vec![
                                render_stickers(state.base.stickers.clone())
                            ]
                        },
                        Phase::Trace => {
                            let raw_stickers = state.base.stickers.to_raw();
                            let theme_id = state.base.theme_id.get();

                            vec![
                                render_stickers_raw(&raw_stickers, theme_id),
                                TracesEdit::render(state.base.traces.clone()),
                                html!("overlay-container", {
                                    .children_signal_vec(
                                        state.trace_bubbles()
                                            .map(|bubble| {
                                                TraceBubble::render(bubble)
                                            })
                                    )
                                })
                            ]
                        }
                    }
                }))
                .to_signal_vec()
            )
        })
    }
}

impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}
