use super::state::*;
use std::rc::Rc;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::base::styles;
use dominator::{html, Dom, clone, with_node};
use utils::{
    prelude::*,
    image_effects::ImageEffect,
    resize::resize_info_signal
};

impl Puzzle {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("canvas" => web_sys::HtmlCanvasElement, {
            .class(&*styles::FULL_STAGE)
            .with_node!(canvas => {
                .future(state.init_phase.signal_cloned().for_each(clone!(state, canvas => move |init_phase| {
                    clone!(state, canvas => async move {
                        match init_phase {
                            InitPhase::Loading => {
                                let url = state.base.design_media_url(&state.raw.full_cutout_img);
                                let effects = ImageEffect::new_url(&url, None).await;

                                state.init_phase.set(InitPhase::Playing(PuzzleGame::new(&state, canvas, effects)));
                            },
                            _ => {}
                        }
                    })
                })))
            })
            .future(state.game_signal().for_each(clone!(state => move |(init_phase, resize_info)| {
                clone!(state => async move {
                    match init_phase {
                        InitPhase::Playing(game) => {
                            game.draw(&resize_info);
                        },
                        _ => {}
                    } 
                })
            })))

            .event(clone!(state => move |evt:events::MouseDown| {
                match state.init_phase.get_cloned() {
                    InitPhase::Playing(game) => {
                        game.start_drag(evt.x(), evt.y());
                    },
                    _ => {}
                }
            }))
            .global_event(clone!(state => move |evt:events::MouseMove| {
                match state.init_phase.get_cloned() {
                    InitPhase::Playing(game) => {
                        game.try_move_drag(evt.x(), evt.y());
                    },
                    _ => {}
                }
            }))
            .global_event(clone!(state => move |evt:events::MouseUp| {
                match state.init_phase.get_cloned() {
                    InitPhase::Playing(game) => {
                        game.try_end_drag(evt.x(), evt.y());
                    },
                    _ => {}
                }
            }))
        })
    }
}
