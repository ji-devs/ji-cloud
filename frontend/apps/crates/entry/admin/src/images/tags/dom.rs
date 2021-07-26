use dominator::{html, clone, Dom};
use shared::domain::image::{ImageId, ImageSearchQuery};
use std::rc::Rc;
use std::cell::RefCell;
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use utils::{routes::*, events};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use super::state::*;


impl ImageTags {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .style("margin", "20px")
            .child(
                html!("window-loader-block", {
                    .property_signal("visible", state.loader.is_loading())
                })
            )
            .child(html!("button", {
                .text("SYNC")
                .style("font-size", "xx-large")
                .event(clone!(state => move |evt:events::Click| {
                    Self::sync_all(state.clone());
                }))
            }))
            .child(html!("h2", {
                .text("Current tags:")
            }))
            .child(html!("ul", {
                .children_signal_vec(state.list.signal_vec_cloned().map(clone!(state => move |item| {
                    html!("li", {
                        .text(&item.display_name)
                    })
                })))
            }))
        })
    }
}
