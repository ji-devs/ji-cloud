use super::state::*;

use crate::base::styles;
use crate::config::HINT_TIME_MS;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{
    prelude::*,
    resize::{resize_info_signal, ResizeInfo},
};

use components::overlay::handle::OverlayHandle;
use gloo_timers::future::TimeoutFuture;

impl TalkType {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .class(&*styles::FULL_STAGE)
            .children_signal_vec(
                resize_info_signal().map(clone!(state => move |resize_info| {
                    state.items
                        .iter()
                        .map(|item| item.clone().render_text_input(state.clone(), &resize_info))
                        .collect()
                }))
                .to_signal_vec()
            )
        })
    }
}

impl TalkTypeItem {
    pub fn render_text_input(
        self: Rc<Self>,
        parent: Rc<TalkType>,
        resize_info: &ResizeInfo,
    ) -> Dom {
        let state = self;
        let bounds = state.bounds.denormalize(resize_info);
        let mut abs_bounds = bounds.clone();
        abs_bounds.x += resize_info.x;
        abs_bounds.y += resize_info.y;

        html!("legacy-input-fit", {

            .future(state.phase.signal().for_each(clone!(state => move |phase| {
                clone!(state => async move {
                    if phase == TalkTypeItemPhase::Wrong {
                        TimeoutFuture::new(HINT_TIME_MS).await;
                        state.phase.set_neq(TalkTypeItemPhase::Input);
                    }
                })
            })))
            .property("y", bounds.y)
            .property("x", bounds.x)
            .property("width", bounds.width)
            .property("height", bounds.height)
            .property_signal("value", state.value.signal_cloned())
            .property_signal("color", state.phase.signal().map(|phase| {
                match phase {
                    TalkTypeItemPhase::Wrong => "red",
                    TalkTypeItemPhase::Correct => "green",
                    _ => ""
                }
            }))
            .event(clone!(state => move |_evt:events::Focus| {
                state.play_audio();
            }))
            .event(clone!(state => move |evt:events::CustomInput| {
                state.phase.set_neq(TalkTypeItemPhase::Input);
                state.value.set_neq(evt.value())
            }))
            .event(clone!(state, parent => move |_evt:events::Enter| {
                state.clone().evaluate(parent.clone())
            }))
            .with_node!(_elem => {
                .apply_if(parent.raw.show_hints, OverlayHandle::lifecycle(clone!(state => move || {
                    html!("empty-fragment", {
                        .child_signal(state.phase.signal().map(clone!(state => move |phase| {
                            match phase {
                                TalkTypeItemPhase::Wrong => {
                                    Some(html!("overlay-tooltip-bubble", {
                                        .text(&state.hint_letters.borrow().to_string())
                                        .property("target", web_sys::DomRect::from(abs_bounds))
                                        .property("targetAnchor", "bm")
                                        .property("contentAnchor", "oppositeV")
                                    }))
                                },
                                _ => None
                            }
                        })))
                    })
                })))
            })
        })
    }
}
