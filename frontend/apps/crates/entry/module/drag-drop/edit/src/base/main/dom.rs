use components::module::_common::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_stickers_raw},
    traces::edit::dom::render_traces_edit
};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .child_signal(
                state.locked_scene_signal().map(clone!(state => move |locked_scene| Some({
                    if locked_scene {
                        let raw_stickers = state.base.stickers.to_raw();
                        render_stickers_raw(&raw_stickers)
                    } else {
                        render_stickers(state.base.stickers.clone())
                    }
                })))
            )
            .child_signal(
                state.trace_phase_signal().map(clone!(state => move |trace_phase| {
                    trace_phase.map(|trace_phase| {
                        match trace_phase {
                            TracePhase::Edit => {
                                render_traces_edit(state.base.traces.clone())
                            },
                            TracePhase::Show => {
                                html!("div")
                            }
                        }
                    })
                }))
            )
        })
    }
}

impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}
