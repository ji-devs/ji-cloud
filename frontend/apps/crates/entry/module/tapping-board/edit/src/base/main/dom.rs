use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{
    module::_common::edit::prelude::*,
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_stickers_raw},
    traces::{
        edit::TracesEdit,
        bubble::TraceBubble,
    }
};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

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
                                html!("empty-fragment", {
                                    .children_signal_vec(
                                        state.trace_bubbles()
                                            .map(clone!(state => move |bubble| {
                                                TraceBubble::render(bubble, &state.base.audio_mixer)
                                            }))
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
