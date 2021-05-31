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


pub fn render(state:Rc<Draw>, menu:Menu, resize_info:&ResizeInfo) -> Dom {
    let (x, y) = menu.get_pos(&resize_info);

    html!("trace-edit-reshape-menu", {

        .visible_signal(state.trace.transform.is_transforming.signal().map(|x| !x))
        .style("left", format!("{}px", x))
        .style("top", format!("{}px", y))
        .children(&mut [
              html!("button", {
                  .text("Free")
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_free();
                  }))
                    .visible_signal(state.reshape_menu_options_signal())
              }),
              html!("button", {
                  .text("Rectangle")
                    .visible_signal(state.reshape_menu_options_signal())
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_rect();
                  }))
              }),
              html!("button", {
                  .text("Ellipse")
                    .visible_signal(state.reshape_menu_options_signal())
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_ellipse();
                  }))
              }),
              html!("button", {
                  .text("Done")
                  .event(clone!(state => move |evt:events::Click| {
                      state.done();
                  }))
              }),
              html!("button", {
                  .text("Cancel")
                  .event(clone!(state => move |evt:events::Click| {
                      state.cancel();
                  }))
              }),
        ])
    })


}
