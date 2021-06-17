use components::module::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{backgrounds, stickers, traces};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .children_signal_vec(
                state.phase_signal().map(clone!(state => move |phase| {
                    match phase {
                        Phase::Layout => {
                            vec![
                                backgrounds::dom::render(state.base.backgrounds.clone(), None),
                                stickers::dom::render(state.base.stickers.clone(), None)
                            ]
                        },
                        Phase::Trace => {
                            let raw_stickers = state.base.stickers.to_raw();

                            vec![
                                backgrounds::dom::render(state.base.backgrounds.clone(), None),
                                stickers::dom::render_raw(&raw_stickers),
                                traces::edit::dom::render(state.base.traces.clone()),
                                html!("empty-fragment", {
                                    .children_signal_vec(
                                        state.trace_bubbles()
                                            .map(clone!(state => move |bubble| {
                                                traces::bubble::dom::render(bubble, &state.base.audio_mixer)
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
