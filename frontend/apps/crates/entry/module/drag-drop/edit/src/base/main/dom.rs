use components::module::_common::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::{
    state::*,
    select::*,
    drag::*
};
use components::{
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_stickers_raw},
    traces::{
        edit::dom::render_traces_edit,
        hints::dom::render_traces_hint,
    }
};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        let theme_id = state.base.theme_id.get();

        html!("empty-fragment", {
            .child_signal(
                state.sticker_phase_signal().map(clone!(state => move |sticker_phase| Some({
                    match sticker_phase {
                        StickerPhase::Scene => {
                            render_stickers(state.base.stickers.clone())
                        },
                        StickerPhase::Select(state) => {
                            MainSelect::render(state)
                        },
                        StickerPhase::Drag(state) => {
                            MainDrag::render(state)
                        },
                        StickerPhase::Static => {
                            let raw_stickers = state.base.stickers.to_raw();
                            render_stickers_raw(&raw_stickers, theme_id)
                        }
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
                                render_traces_hint(state.base.traces.to_raw())
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
