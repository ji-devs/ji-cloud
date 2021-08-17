use dominator::{html, Dom, clone};
use utils::prelude::*;
use super::state::*;
use std::rc::Rc;
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt
};

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
            .event(clone!(state => move |evt:events::Click| {
                state.on_click();
            }))
        })
    }
}
