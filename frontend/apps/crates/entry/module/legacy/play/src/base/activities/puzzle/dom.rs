use super::state::*;
use crate::base::styles;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use utils::{image_effects::ImageEffect, prelude::*};

impl Puzzle {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("canvas" => web_sys::HtmlCanvasElement, {
            .style("touch-action", "none")
            .class(&*styles::FULL_STAGE)
            .with_node!(canvas => {
                .future(state.init_phase.signal_cloned().for_each(clone!(state, canvas => move |init_phase| {
                    clone!(state, canvas => async move {
                        match init_phase {
                            InitPhase::Loading => {
                                let url = state.base.design_media_url(&state.base.slide.image_full);
                                let effects = ImageEffect::new_url(&url, None).await;

                                if state.raw.show_preview {
                                    state.init_phase.set(InitPhase::Preview(PuzzlePreview::new(&state, canvas, effects)));
                                } else {
                                    state.init_phase.set(InitPhase::Playing(PuzzleGame::new(&state, canvas, effects)));
                                }
                            },
                            InitPhase::Preview(preview) => {
                                TimeoutFuture::new(crate::config::PUZZLE_PREVIEW_DELAY).await;
                                preview.start_animation(state);
                            }
                            _ => {}
                        }
                    })
                })))
            })
            .future(state.game_signal().for_each(move |(init_phase, resize_info)| {
                async move {
                    match init_phase {
                        InitPhase::Playing(game) => {
                            game.draw(&resize_info);
                        },
                        InitPhase::Preview(preview) => {
                            preview.game.draw(&resize_info);
                        },
                        _ => {}
                    }
                }
            }))
            .event(clone!(state => move |evt:events::PointerDown| {
                let init_phase = state.init_phase.get_cloned();
                if let InitPhase::Playing(game) = init_phase {
                    game.start_drag(evt.x(), evt.y());
                }
            }))
            .global_event(clone!(state => move |evt:events::PointerMove| {
                let init_phase = state.init_phase.get_cloned();
                if let InitPhase::Playing(game) = init_phase {
                    game.try_move_drag(evt.x(), evt.y());
                }
            }))
            .global_event(clone!(state => move |evt:events::PointerUp| {
                let init_phase = state.init_phase.get_cloned();
                if let InitPhase::Playing(game) = init_phase {
                    game.try_end_drag(evt.x(), evt.y());
                }
            }))
            .global_event(clone!(state => move |evt:events::PointerCancel| {
                let init_phase = state.init_phase.get_cloned();
                if let InitPhase::Playing(game) = init_phase {
                    game.try_end_drag(evt.x(), evt.y());
                }
            }))
        })
    }
}
