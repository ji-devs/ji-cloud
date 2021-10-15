use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;

impl Sidebar {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("admin-sidebar", {
            .children_signal_vec(
                state.item_signal_vec()
                    .map(SidebarItem::render)
            )
        })
    }
}

impl SidebarItem {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("admin-sidebar-item", {
            .property("id", state.id)
            .property("locked", state.locked)
            .property("selected", state.selected)
            .event(clone!(state => move |_evt:events::Click| {
                state.on_click();
            }))
        })
    }
}
