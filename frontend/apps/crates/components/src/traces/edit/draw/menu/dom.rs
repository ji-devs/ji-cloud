use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::{prelude::*, resize::ResizeInfo};
use crate::overlay::handle::OverlayHandle;
use futures_signals::signal::SignalExt;

use super::{super::state::*, state::*};
/*
<trace-edit-reshape-menu style="position: absolute; top: 100px; left: 100px">
    <button-icon icon="circle-x-blue" slot="close"></button-icon>
    <trace-edit-reshape-menu-btn kind="path"></trace-edit-reshape-menu-btn>
    <trace-edit-reshape-menu-btn kind="rect"></trace-edit-reshape-menu-btn>
    <trace-edit-reshape-menu-btn kind="ellipse"></trace-edit-reshape-menu-btn>
    <trace-edit-reshape-menu-btn kind="confirm"></trace-edit-reshape-menu-btn>
</trace-edit-reshape-menu>
*/

pub fn render_draw_menu(state: Rc<Draw>, menu: Menu, resize_info: &ResizeInfo) -> Dom {

    let resize_info = resize_info.clone();

    html!("empty-fragment", {
        .apply(OverlayHandle::lifecycle(
        move || {
            html!("overlay-drag", {
                .property("target", menu.get_dom_rect(&resize_info))
                .property("targetAnchor", "bm")
                .property("contentAnchor", "tl")
                .property("marginY", 24.0)
                .child(html!("trace-edit-reshape-menu", {

                    .property_signal("noGap", state.reshape_menu_options_signal().map(|x| !x))
                    .visible_signal(state.trace.transform.is_transforming.signal().map(|x| !x))
                    .children(&mut [
                        html!("trace-edit-reshape-menu-btn", {
                            .property("kind", "free")
                            .event(clone!(state => move |_evt:events::Click| {
                                state.shape_free();
                            }))
                            .visible_signal(state.reshape_menu_options_signal())
                        }),
                        html!("trace-edit-reshape-menu-btn", {
                            .property("kind", "rect")
                            .event(clone!(state => move |_evt:events::Click| {
                                state.shape_rect();
                            }))
                            .visible_signal(state.reshape_menu_options_signal())
                        }),
                        html!("trace-edit-reshape-menu-btn", {
                            .property("kind", "ellipse")
                            .event(clone!(state => move |_evt:events::Click| {
                                state.shape_ellipse();
                            }))
                            .visible_signal(state.reshape_menu_options_signal())
                        }),
                        html!("trace-edit-reshape-menu-btn", {
                            .property("kind", "confirm")
                            .property_signal("bothSidesRounded", state.reshape_menu_options_signal().map(|x| !x))
                            .event(clone!(state => move |_evt:events::Click| {
                                state.done();
                            }))
                            //.visible_signal(state.reshape_menu_options_signal())
                        }),

                        html!("button-icon", {
                            .property("icon", "circle-x-blue")
                            .property("slot", "close")
                            .event(clone!(state => move |_evt:events::Click| {
                                state.cancel();
                            }))
                            .visible_signal(state.reshape_menu_options_signal())
                        }),
                    ])
                }))
            })
        }))
    })
}
