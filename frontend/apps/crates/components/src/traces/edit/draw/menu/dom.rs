use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{ResizeInfo, resize_info_signal}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};

use super::{
    state::*,
    super::state::*
};
/*
    <trace-edit-reshape-menu style="position: absolute; top: 100px; left: 100px">
        <button-icon icon="circle-x-blue" slot="close"></button-icon>
        <trace-edit-reshape-menu-btn kind="path"></trace-edit-reshape-menu-btn>
        <trace-edit-reshape-menu-btn kind="rect"></trace-edit-reshape-menu-btn>
        <trace-edit-reshape-menu-btn kind="ellipse"></trace-edit-reshape-menu-btn>
        <trace-edit-reshape-menu-btn kind="confirm"></trace-edit-reshape-menu-btn>
    </trace-edit-reshape-menu>
    */

pub fn render(state:Rc<Draw>, menu:Menu, resize_info:&ResizeInfo) -> Dom {
    let (x, y) = menu.get_pos(&resize_info);

    html!("trace-edit-reshape-menu", {

        .visible_signal(state.trace.transform.is_transforming.signal().map(|x| !x))
        .style("left", format!("{}px", x))
        .style("top", format!("{}px", y + 24.0 /* nudge a bit for close button */))
        .children(&mut [
              html!("trace-edit-reshape-menu-btn", {
                  .property("kind", "free")
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_free();
                  }))
                  .visible_signal(state.reshape_menu_options_signal())
              }),
              html!("trace-edit-reshape-menu-btn", {
                  .property("kind", "rect")
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_rect();
                  }))
                  .visible_signal(state.reshape_menu_options_signal())
              }),
              html!("trace-edit-reshape-menu-btn", {
                  .property("kind", "ellipse")
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_ellipse();
                  }))
                  .visible_signal(state.reshape_menu_options_signal())
              }),
              html!("trace-edit-reshape-menu-btn", {
                  .property("kind", "confirm")
                  .event(clone!(state => move |evt:events::Click| {
                      state.done();
                  }))
                  .visible_signal(state.reshape_menu_options_signal())
              }),
                        
              html!("button-icon", {
                  .property("icon", "circle-x-blue")
                    .property("slot", "close")
                  .event(clone!(state => move |evt:events::Click| {
                      state.cancel();
                  }))
              }),
        ])
    })


}
