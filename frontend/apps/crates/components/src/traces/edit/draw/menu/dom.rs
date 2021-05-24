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

    html!("trace-edit-menu", {

        .visible_signal(state.trace.transform.menu_button_visible.signal())
        .style("left", format!("{}px", x))
        .style("top", format!("{}px", y))
        .children(&mut [
              html!("button", {
                  .text("Free")
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_free();
                  }))
              }),
              html!("button", {
                  .text("Rectangle")
                  .event(clone!(state => move |evt:events::Click| {
                      state.shape_rect();
                  }))
              }),
              html!("button", {
                  .text("Ellipse")
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
        ])
    })


}
