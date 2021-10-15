use dominator::{clone, html, Dom};

use std::rc::Rc;

use futures_signals::signal_vec::SignalVecExt;
use utils::events;

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
                .event(clone!(state => move |_evt:events::Click| {
                    Self::sync_all(state.clone());
                }))
            }))
            .child(html!("h2", {
                .text("Current tags:")
            }))
            .child(html!("ul", {
                .children_signal_vec(state.list.signal_vec_cloned().map(|item| {
                    html!("li", {
                        .text(&item.display_name)
                    })
                }))
            }))
        })
    }
}
