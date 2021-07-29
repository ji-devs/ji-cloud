use dominator::{html, Dom, clone};
use shared::domain::jig::Jig;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::HtmlElement;
use utils::events;
use crate::edit::sidebar::{
    state::State as SidebarState,
    module::{
        state::State as ModuleState,
        actions::{self, MoveTarget},
    }
};
use super::state::*;


const STR_CUSTOM_COPY:&'static str = "Copy to another Jig";

pub struct MenuDom {
}

impl MenuDom {
    pub fn render(module: Rc<ModuleState>) -> Dom {
        let state = Rc::new(State::new(module));

        let mut children = vec![
            html!("menu-line", {
                .property("slot", "lines")
                .property("icon", "edit")
                .event(clone!(state => move |evt:events::Click| {
                    actions::edit(state.module.clone());
                    state.close_menu();
                }))
            })
        ];

        if state.index() != 0 {
            children.append(&mut vec![
                html!("menu-line", {
                    .property("slot", "lines")
                    .property("icon", "move-up")
                    .event(clone!(state => move |evt:events::Click| {
                        actions::move_index(state.module.clone(), MoveTarget::Up);
                        state.close_menu();
                    }))
                }),
                html!("menu-line", {
                    .property("slot", "lines")
                    .property("icon", "move-down")
                    .event(clone!(state => move |evt:events::Click| {
                        actions::move_index(state.module.clone(), MoveTarget::Down);
                        state.close_menu();
                    }))
                }),
                html!("menu-line", {
                    .property("slot", "lines")
                    .property("icon", "duplicate")
                    .event(clone!(state => move |evt:events::Click| {
                        state.close_menu();
                    }))
                }),
                html!("menu-line", {
                    .property("slot", "advanced")
                    .property("customLabel", STR_CUSTOM_COPY)
                    .property("icon", "copy")
                    .event(clone!(state => move |evt:events::Click| {
                        state.close_menu();
                    }))
                }),
                html!("menu-line", {
                    .property("slot", "lines")
                    .property("icon", "delete")
                    .event(clone!(state => move |evt:events::Click| {
                        actions::delete(state.module.clone());
                        state.close_menu();
                    }))
                })
            ]);
        }
        html!("menu-kebab", {
            .property("slot", "menu")
            .child(html!("jig-edit-sidebar-module-menu", {
                .property("hideAdvancedSection", state.index() == 0)
                .children(children)
            }))
            .after_inserted(clone!(state => move |elem| {
                *state.menu_ref.borrow_mut() = Some(elem);
            }))
        })
    }
}
