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
    traces::{edit::*, show::*}
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
                    trace_phase.and_then(|trace_phase| {
                        match trace_phase {
                            TracePhase::Edit => {
                                Some(TracesEdit::render(state.base.traces.clone()))
                            },
                            TracePhase::Show => {
                                Some(TracesShow::render(TracesShow::new(
                                    state.base.traces.to_raw(),
                                    TracesShowMode::Solid,
                                    TracesShow::on_select_noop()
                                )))
                            },
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
