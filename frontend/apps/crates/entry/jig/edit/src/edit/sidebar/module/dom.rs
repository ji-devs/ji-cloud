use dominator::{clone, html, Dom};

use super::super::menu::{dom as MenuDom, state::State as MenuState};
use super::{actions, state::*};
use crate::edit::sidebar::state::State as SidebarState;
use futures_signals::signal::SignalExt;
use shared::domain::jig::{LiteModule, ModuleKind};
use std::rc::Rc;
use std::str::FromStr;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use components::module::_common::thumbnail::ModuleThumbnail;
pub struct ModuleDom {}

impl ModuleDom {
    pub fn render(
        sidebar_state: Rc<SidebarState>,
        index: usize,
        drag_target_index: Option<usize>,
        total_len: usize,
        module: Rc<Option<LiteModule>>,
    ) -> Dom {
        let state = Rc::new(State::new(
            sidebar_state.clone(),
            index,
            total_len,
            module.clone(),
        ));

        let is_filler = Some(index) == drag_target_index;

        html!("empty-fragment", {
            .property("slot", if index == 0 { "cover-module" } else { "modules" })
            .child(html!("jig-edit-sidebar-filler", {
                .style("display", {
                    if is_filler { "block" } else {"none"}
                })
            }))
            .child( html!("jig-sidebar-module", {
                .future(State::drag_overlap_signal(state.clone()).for_each(clone!(state => move |overlap| {
                    if overlap {
                        state.sidebar.drag_target_index.set(Some(state.index));
                    }
                        //Doing this here instead of immediately on mousemove
                        //gives us a nice separation of concerns
                        //e.g. to throttle
                        //actions::update_index(state.clone(), pos.x, pos.y);
                    async {}
                })))
                .style("display", {
                    if is_filler { "none" } else {"block"}
                })
                .property("module", state.kind_str())
                .property("index", index as u32)
                .property_signal("collapsed", state.sidebar.collapsed.signal())
                .property_signal("selected", state.sidebar.route.signal_ref(clone!(module => move |route| {
                    match (&*module, route) {
                        (Some(module), JigEditRoute::Module(module_id)) if module_id == &module.id => true,
                        _ => false,
                    }
                })))
                .property("lastBottomDecoration", index == total_len-1)
                .event(clone!(state => move |_evt:events::MouseDown| {
                    // TODO:
                    // actions::mouse_down(state.clone(), evt.x(), evt.y());
                }))
                .child(html!("jig-edit-sidebar-module-window", {
                    .property("slot", "window")
                    .property_signal("state", State::window_state_signal(Rc::clone(&state)))
                    .property("activeModuleKind", state.kind_str())
                    .event_preventable(clone!(state => move |evt:events::DragOver| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            if data_transfer.types().index_of(&JsValue::from_str("module_kind"), 0) != -1 {
                                if state.module.is_none() {
                                    evt.prevent_default();
                                }
                            }
                        }

                    }))
                    .event(clone!(state => move |evt:events::Drop| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            if let Some(module_kind) = data_transfer.get_data("module_kind").ok() {
                                let kind:ModuleKind = ModuleKind::from_str(&module_kind).unwrap_ji();
                                actions::assign_kind(state.clone(), kind);
                            }
                        }
                    }))
                    .event(clone!(state => move |_evt:events::Click| {
                        actions::edit(state.clone());
                    }))
                    .child_signal(state.sidebar.route.signal_ref(clone!(state, module => move |route| {
                        match (&*module, route) {
                            (Some(module), JigEditRoute::Module(module_id)) if module_id == &module.id => None,
                            (Some(module), _) => {
                                Some(ModuleThumbnail::render(
                                        Rc::new(ModuleThumbnail {
                                            jig_id: state.sidebar.jig.id.clone(),
                                            module: module.clone(), 
                                            is_jig_fallback: false,
                                        }),
                                        Some("thumbnail")
                                ))
                            },
                            _ => None,
                        }
                    })))
                }))
                .after_inserted(clone!(state => move |dom| {
                    *state.elem.borrow_mut() = Some(dom);
                    //*state.sidebar.drag_targets.borrow_mut().
                }))
                .after_removed(clone!(state => move |_dom| {
                    *state.elem.borrow_mut() = None;
                }))
                .apply(clone!(state, sidebar_state, module => move |dom| {
                    let menu_state = Rc::new(MenuState::new());
                    let menu_items = match index {
                        0 => {
                            vec![
                                MenuDom::item_edit(menu_state.clone(), state.clone()),
                                // TODO:
                                // MenuDom::item_copy(menu_state.clone()),
                                MenuDom::item_paste(menu_state.clone(), sidebar_state.clone()),
                            ]
                        },
                        _ => {
                            let mut v = vec![];
                            if let Some(module) = &*module {
                                v.push(MenuDom::item_edit(menu_state.clone(), state.clone()));
                                v.push(MenuDom::item_move_up(menu_state.clone(), state.clone()));
                                v.push(MenuDom::item_move_down(menu_state.clone(), state.clone()));
                                v.push(MenuDom::item_duplicate(menu_state.clone(), sidebar_state.clone(), module.id));
                            }
                            v.push(MenuDom::item_delete(menu_state.clone(), state.clone()));
                            if let Some(module) = &*module {
                                v.push(MenuDom::item_copy(menu_state.clone(), sidebar_state.clone(), module.id));
                                v.push(MenuDom::item_duplicate_as(menu_state.clone(), sidebar_state.clone(), module));
                            }
                            v
                        }
                    };

                    dom.child(MenuDom::render(
                        menu_state.clone(),
                        menu_items
                    ))
                }))
                .child(html!("button-icon", {
                    .property("icon", "gears")
                    .property("slot", "add")
                    .event(clone!(state => move |_evt:events::Click| {
                        actions::add_empty_module_after(state.clone())
                    }))
                }))
            }))
        })
    }
}
