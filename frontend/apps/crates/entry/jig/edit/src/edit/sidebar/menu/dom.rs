use super::state::*;
use crate::edit::sidebar::{
    actions::{duplicate_module, use_module_as},
    copy_paste_module::{copy_module, paste_module},
    module::{
        actions::{self, MoveTarget},
        state::State as ModuleState,
    },
    state::State as SidebarState,
};
use dominator::{clone, html, Dom, EventOptions};
use shared::domain::jig::{module::ModuleId, LiteModule, ModuleKind};
use std::rc::Rc;
use utils::events;

const STR_COPY: &'static str = "Copy to another Jig";
const STR_PASTE: &'static str = "Paste from another JIG";
const STR_DUPLICATE_AS: &'static str = "Duplicate content as:";
// const STR_EDIT_SETTINGS: &'static str = "Edit setting";

pub fn render(state: Rc<State>, items: Vec<Dom>) -> Dom {
    html!("menu-kebab", {
        .property("slot", "menu")
        .child(html!("jig-edit-sidebar-module-menu", {
            .children(items)
        }))
        .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
            e.stop_propagation();
        })
        .after_inserted(clone!(state => move |elem| {
            *state.menu_ref.borrow_mut() = Some(elem);
        }))
    })
}

pub fn item_edit(state: Rc<State>, module: Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "edit")
        .event(clone!(state => move |_:events::Click| {
            actions::edit(module.clone());
            state.close_menu();
        }))
    })
}

pub fn item_move_up(state: Rc<State>, module: Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "move-up")
        .event(clone!(state => move |_:events::Click| {
            actions::move_index(module.clone(), MoveTarget::Up);
            state.close_menu();
        }))
    })
}

pub fn item_move_down(state: Rc<State>, module: Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "move-down")
        .event(clone!(state => move |_:events::Click| {
            actions::move_index(module.clone(), MoveTarget::Down);
            state.close_menu();
        }))
    })
}

pub fn item_duplicate(
    state: Rc<State>,
    sidebar_state: Rc<SidebarState>,
    module_id: ModuleId,
) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "duplicate")
        .event(clone!(state, module_id => move |_:events::Click| {
            state.close_menu();
            duplicate_module(sidebar_state.clone(), &module_id);
        }))
    })
}

pub fn item_delete(state: Rc<State>, module: Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "delete")
        .event(clone!(state => move |_:events::Click| {
            actions::delete(module.clone());
            state.close_menu();
        }))
    })
}

pub fn item_copy(state: Rc<State>, sidebar_state: Rc<SidebarState>, module_id: ModuleId) -> Dom {
    html!("menu-line", {
        .property("slot", "advanced")
        .property("customLabel", STR_COPY)
        .property("icon", "copy")
        .event(clone!(state => move |_:events::Click| {
            state.close_menu();
            copy_module(sidebar_state.clone(), &module_id);
        }))
    })
}

pub fn item_paste(state: Rc<State>, sidebar_state: Rc<SidebarState>) -> Dom {
    html!("menu-line", {
        .property("slot", "advanced")
        .property("customLabel", STR_PASTE)
        .property("icon", "copy")
        .event(clone!(state => move |_:events::Click| {
            state.close_menu();
            paste_module(sidebar_state.clone());
        }))
    })
}

pub fn item_duplicate_as(
    state: Rc<State>,
    sidebar_state: Rc<SidebarState>,
    module: &LiteModule,
) -> Dom {
    let card_kinds = vec![
        ModuleKind::Memory,
        ModuleKind::Flashcards,
        ModuleKind::Matching,
        ModuleKind::CardQuiz,
    ];

    let is_card = card_kinds.contains(&module.kind);

    let card_kinds = card_kinds.into_iter().filter(|kind| &module.kind != kind);

    let module_id = module.id;

    html!("empty-fragment", {
        .property("slot", "advanced")
        .apply_if(is_card, |dom| {
            dom.child(html!("menu-line", {
                .property("customLabel", STR_DUPLICATE_AS)
                .property("icon", "reuse")
                .property_signal("active", state.dup_as_active.signal())
                .event(clone!(state => move |_:events::Click| {
                    let mut dup_as_active = state.dup_as_active.lock_mut();
                    *dup_as_active = !*dup_as_active;
                }))
            }))
            .children(card_kinds.map(|card_kind| {
                html!("menu-line", {
                    .visible_signal(state.dup_as_active.signal())
                    .property("customLabel", String::from("• ") + card_kind.as_str())
                    .event(clone!(state, sidebar_state, module_id => move |_:events::Click| {
                        use_module_as(Rc::clone(&sidebar_state), card_kind, module_id);
                        state.close_menu();
                    }))
                })
            }))
        })
    })
}

// pub fn item_edit_settings(state: Rc<State>, sidebar_state: Rc<SidebarState>) -> Dom {
//     html!("menu-line", {
//         .property("slot", "lines")
//         .property("customLabel", STR_EDIT_SETTINGS)
//         .property("icon", "edit")
//         .event(clone!(state => move |_:events::Click| {
//             state.close_menu();
//             // sidebar_state.settings_shown.set(true);
//             sidebar_state.settings.active_popup.set(Some(ActiveSettingsPopup::Main))
//         }))
//     })
// }
