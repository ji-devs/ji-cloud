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

const CARD_KINDS: [ModuleKind; 4] = [
    ModuleKind::Memory,
    ModuleKind::Flashcards,
    ModuleKind::Matching,
    ModuleKind::CardQuiz,
];

pub fn render(module_state: &Rc<ModuleState>) -> Dom {
    let state = Rc::new(State::new());

    html!("menu-kebab", {
        .property("slot", "menu")
        .child(html!("jig-edit-sidebar-module-menu", {
            .children(menu_items(&state, module_state))
        }))
        .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
            e.stop_propagation();
        })
        .after_inserted(move |elem| {
            *state.menu_ref.borrow_mut() = Some(elem);
        })
    })
}

fn menu_items(state: &Rc<State>, module_state: &Rc<ModuleState>) -> Vec<Dom> {
    match module_state.index {
        0 => {
            vec![
                item_edit(state, module_state),
                // TODO:
                // item_copy(state.clone()),
                item_paste(state, &module_state.sidebar),
            ]
        },
        _ => {
            let mut v = vec![];
            if let Some(module) = &*module_state.module {
                v.push(item_edit(state, module_state));
                if module_state.index > 1 {
                    // We only want to be able to move up if there's somewhere
                    // to move to. Index 0 is occupied by the Cover module, so
                    // anything at 1 cannot go higher.
                    v.push(item_move_up(state, module_state));
                }
                if module_state.is_last_module() {
                    v.push(item_move_down(state, module_state));
                }
                v.push(item_duplicate(state, &module_state.sidebar, module.id));
            }
            v.push(item_delete(state, module_state));
            if let Some(module) = &*module_state.module {
                v.push(item_copy(state, &module_state.sidebar, module.id));
                v.push(item_duplicate_as(state, &module_state.sidebar, module));
            }
            v
        }
    }
}

fn item_edit(state: &Rc<State>, module: &Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "edit")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::edit(module.clone());
        }))
    })
}

fn item_move_up(state: &Rc<State>, module: &Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "move-up")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::move_index(module.clone(), MoveTarget::Up);
        }))
    })
}

fn item_move_down(state: &Rc<State>, module: &Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "move-down")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::move_index(module.clone(), MoveTarget::Down);
        }))
    })
}

fn item_duplicate(
    state: &Rc<State>,
    sidebar_state: &Rc<SidebarState>,
    module_id: ModuleId,
) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "duplicate")
        .event(clone!(state, sidebar_state => move |_:events::Click| {
            state.close_menu();
            duplicate_module(sidebar_state.clone(), &module_id);
        }))
    })
}

fn item_delete(state: &Rc<State>, module: &Rc<ModuleState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "delete")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::delete(module.clone());
        }))
    })
}

fn item_copy(state: &Rc<State>, sidebar_state: &Rc<SidebarState>, module_id: ModuleId) -> Dom {
    html!("menu-line", {
        .property("slot", "advanced")
        .property("customLabel", STR_COPY)
        .property("icon", "copy")
        .event(clone!(state, sidebar_state => move |_:events::Click| {
            state.close_menu();
            copy_module(sidebar_state.clone(), &module_id);
        }))
    })
}

fn item_paste(state: &Rc<State>, sidebar_state: &Rc<SidebarState>) -> Dom {
    html!("menu-line", {
        .property("slot", "advanced")
        .property("customLabel", STR_PASTE)
        .property("icon", "copy")
        .event(clone!(state, sidebar_state => move |_:events::Click| {
            state.close_menu();
            paste_module(sidebar_state.clone());
        }))
    })
}

fn item_duplicate_as(
    state: &Rc<State>,
    sidebar_state: &Rc<SidebarState>,
    module: &LiteModule,
) -> Dom {
    let is_card = CARD_KINDS.contains(&module.kind);

    html!("empty-fragment", {
        .property("slot", "advanced")
        .apply_if(is_card, |dom| {
            let card_kinds = CARD_KINDS.into_iter().filter(|kind| &module.kind != kind);
            let module_id = module.id;

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

// fn item_edit_settings(state: Rc<State>, sidebar_state: Rc<SidebarState>) -> Dom {
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

