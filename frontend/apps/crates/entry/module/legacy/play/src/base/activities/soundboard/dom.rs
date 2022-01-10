use super::state::*;
use crate::base::styles::FULL_STAGE;
use crate::config::HINT_TIME_MS;
use dominator::{clone, html, svg, Dom};
use futures_signals::{
    signal::{self, SignalExt},
};
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use utils::{
    resize::{resize_info_signal, ResizeInfo},
};

use components::traces::svg::{ShapeStyle, ShapeStyleKind, ShapeStyleMode, ShapeStylePlayMode};

impl Soundboard {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .class(&*FULL_STAGE)
            .child_signal(
                state.phase.signal().map(clone!(state => move |phase| {
                    match phase {
                        // Phase::Intro => {
                        //     None
                        // },

                        Phase::Hints => {
                            Some(svg!("svg", {
                                .future(clone!(state => async move {
                                    TimeoutFuture::new(HINT_TIME_MS).await;
                                    state.on_hints_finished();
                                }))
                                .class(&*FULL_STAGE)
                                .children_signal_vec(
                                    resize_info_signal().map(clone!(state => move |resize_info| {
                                        state.items
                                            .iter()
                                            .map(|item| item.clone().render_svg_hint(state.clone(), &resize_info))
                                            .collect()
                                    }))
                                    .to_signal_vec()
                                )
                            }))
                        },

                        Phase::Intro | Phase::Playing => {
                            Some(svg!("svg", {
                                .class(&*FULL_STAGE)
                                .children_signal_vec(
                                    resize_info_signal().map(clone!(state => move |resize_info| {
                                        state.items
                                            .iter()
                                            .map(|item| item.clone().render_svg_playing(state.clone(), &resize_info))
                                            .collect()
                                    }))
                                    .to_signal_vec()
                                )
                            }))
                        }
                    }
                }))
            )
        })
    }
}

impl SoundboardItem {
    pub fn render_svg_hint(
        self: Rc<Self>,
        _parent: Rc<Soundboard>,
        resize_info: &ResizeInfo,
    ) -> Dom {
        let state = self;
        state.hotspot.render(
            resize_info,
            || {},
            signal::always(ShapeStyle {
                interactive: false,
                mode: ShapeStyleMode::Play(ShapeStylePlayMode::Hint),
                kind: ShapeStyleKind::General,
            }),
        )
    }

    pub fn render_svg_playing(
        self: Rc<Self>,
        parent: Rc<Soundboard>,
        resize_info: &ResizeInfo,
    ) -> Dom {
        let state = self;
        state.hotspot.render(
            resize_info,
            clone!(state, parent => move || {
                state.clone().on_click(parent.clone());
            }),
            state.revealed.signal().map(|revealed| ShapeStyle {
                interactive: true,
                mode: if revealed {
                    ShapeStyleMode::Play(ShapeStylePlayMode::Selected)
                } else {
                    ShapeStyleMode::Transparent
                },
                kind: ShapeStyleKind::General,
            }),
        )
    }
}
