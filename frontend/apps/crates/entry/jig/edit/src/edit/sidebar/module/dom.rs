use dominator::{html, Dom, clone};
use shared::domain::jig::Jig;
use std::rc::Rc;
use crate::edit::sidebar::state::State as SidebarState;
use super::{
    state::*,
    menu::dom::MenuDom,
    actions
};
use futures_signals::signal::SignalExt;
use utils::events;
use shared::domain::jig::{ModuleKind, LiteModule};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub struct ModuleDom {
}

impl ModuleDom {
    pub fn render(sidebar_state: Rc<SidebarState>, index: usize, total_len:usize, module: Rc<Module>) -> Dom {
        let state = Rc::new(State::new(sidebar_state, index, total_len, module));


        html!("jig-edit-sidebar-module", {
            .property_signal("module", state.kind_signal().map(|kind| {
                match kind {
                    Some(kind) => kind.as_str(),
                    None => ""
                }
            }))
            .property("index", index as u32)
            .property("slot", if index == 0 { "cover-module" } else { "modules" })
            .property("lastBottomDecoration", index == total_len-1)
            .child(html!("jig-edit-sidebar-module-window", {
                .property("slot", "window")
                .property_signal("state", state.kind_signal().map(|kind| {
                    match kind {
                        Some(kind) => "draft",
                        None => "empty"
                    }
                }))
                .event_preventable(clone!(state => move |evt:events::DragOver| {
                    if let Some(data_transfer) = evt.data_transfer() {
                        if data_transfer.types().index_of(&JsValue::from_str("module_kind"), 0) != -1 {
                            if state.module.kind.get().is_none() {
                                evt.prevent_default();
                            } 
                        }                     }

                }))
                .event(clone!(state => move |evt:events::Drop| {
                    if let Some(data_transfer) = evt.data_transfer() {
                        if let Some(module_kind) = data_transfer.get_data("module_kind").ok() { 
                            let kind:ModuleKind = ModuleKind::from_str(&module_kind).unwrap_throw();
                            actions::assign_kind(state.clone(), kind);
                        }
                    }
                }))
            }))
            .child(MenuDom::render(state.clone()))
            .apply_if(index < total_len-1, |dom| {
                dom.child(html!("button-icon", {
                    .property("icon", "gears")
                    .property("slot", "add")
                    .event(clone!(state => move |evt:events::Click| {
                        actions::add_module_after(state.clone())
                    }))
                }))
            })
        })
    }
}
