use super::state::*;
use crate::edit::sidebar::{
    jig::actions as jig_actions,
    jig::copy_paste_module::{copy_module, paste_module},
    spot::jig::actions as jig_spot_actions,
    spot::{
        actions::{self, MoveTarget},
        state::State as SpotState,
    },
    state::Sidebar as SidebarState,
};
use dominator::{clone, html, Dom, EventOptions};
use shared::domain::module::{LiteModule, ModuleId, ModuleKind};
use std::rc::Rc;
use utils::events;

const STR_COPY: &str = "Copy to another Jig";
const STR_PASTE: &str = "Paste from another JIG";
const STR_DUPLICATE_AS: &str = "Duplicate content as:";
// const STR_EDIT_SETTINGS: &str = "Edit setting";

const CARD_KINDS: [ModuleKind; 4] = [
    ModuleKind::Memory,
    ModuleKind::Flashcards,
    ModuleKind::Matching,
    ModuleKind::CardQuiz,
];

pub fn render(module_state: &Rc<SpotState>) -> Dom {
    let state = Rc::new(State::new());

    html!("menu-kebab", {
        .prop("slot", "menu")
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

fn menu_items(state: &Rc<State>, module_state: &Rc<SpotState>) -> Vec<Dom> {
    let module = module_state.module.item.unwrap_jig();
    menu_items_jig(state, module_state, module)
}

fn menu_items_jig(
    state: &Rc<State>,
    module_state: &Rc<SpotState>,
    module: &Option<Rc<LiteModule>>,
) -> Vec<Dom> {
    match module_state.index {
        0 => {
            vec![
                item_edit(state, module_state),
                // TODO:
                // item_copy(state.clone()),
                item_paste(state, &module_state.sidebar),
            ]
        }
        _ => {
            let mut v = vec![];
            if let Some(module) = module {
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
            if let Some(module) = module {
                v.push(item_copy(state, &module_state.sidebar, module.id));
                v.push(item_duplicate_as(state, &module_state.sidebar, module));
            }
            v
        }
    }
}

fn item_edit(_: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .prop("slot", "lines")
        .prop("icon", "edit")
        .event(clone!(module => move |_:events::Click| {
            jig_spot_actions::edit(module.clone());
        }))
    })
}

fn item_move_up(state: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .prop("slot", "lines")
        .prop("icon", "move-up")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::move_index(module.clone(), MoveTarget::Up);
        }))
    })
}

fn item_move_down(state: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .prop("slot", "lines")
        .prop("icon", "move-down")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::move_index(module.clone(), MoveTarget::Down);
        }))
    })
}

fn item_duplicate(state: &Rc<State>, sidebar_state: &Rc<SidebarState>, module_id: ModuleId) -> Dom {
    html!("menu-line", {
        .prop("slot", "lines")
        .prop("icon", "duplicate")
        .event(clone!(state, sidebar_state => move |_:events::Click| {
            state.close_menu();
            jig_actions::duplicate_module(sidebar_state.clone(), &module_id);
        }))
    })
}

fn item_delete(state: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .prop("slot", "lines")
        .prop("icon", "delete")
        .event(clone!(state, module => move |_:events::Click| {
            module.confirm_delete.set_neq(true);
            state.close_menu();
        }))
    })
}

fn item_copy(state: &Rc<State>, sidebar_state: &Rc<SidebarState>, module_id: ModuleId) -> Dom {
    html!("menu-line", {
        .prop("slot", "advanced")
        .prop("customLabel", STR_COPY)
        .prop("icon", "copy")
        .event(clone!(state, sidebar_state => move |_:events::Click| {
            state.close_menu();
            copy_module(sidebar_state.clone(), &module_id);
        }))
    })
}

fn item_paste(state: &Rc<State>, sidebar_state: &Rc<SidebarState>) -> Dom {
    html!("menu-line", {
        .prop("slot", "advanced")
        .prop("customLabel", STR_PASTE)
        .prop("icon", "copy")
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
        .prop("slot", "advanced")
        .apply_if(is_card, |dom| {
            let card_kinds = CARD_KINDS.into_iter().filter(|kind| &module.kind != kind);
            let module_id = module.id;

            dom.child(html!("menu-line", {
                .prop("customLabel", STR_DUPLICATE_AS)
                .prop("icon", "reuse")
                .prop_signal("active", state.dup_as_active.signal())
                .event(clone!(state => move |_:events::Click| {
                    let mut dup_as_active = state.dup_as_active.lock_mut();
                    *dup_as_active = !*dup_as_active;
                }))
            }))
            .children(card_kinds.map(|card_kind| {
                html!("menu-line", {
                    .visible_signal(state.dup_as_active.signal())
                    .prop("customLabel", String::from("• ") + card_kind.as_str())
                    .event(clone!(state, sidebar_state, module_id => move |_:events::Click| {
                        jig_actions::use_module_as(Rc::clone(&sidebar_state), card_kind, module_id);
                        state.close_menu();
                    }))
                })
            }))
        })
    })
}

// fn item_edit_settings(state: Rc<State>, sidebar_state: Rc<SidebarState>) -> Dom {
//     html!("menu-line", {
//         .prop("slot", "lines")
//         .prop("customLabel", STR_EDIT_SETTINGS)
//         .prop("icon", "edit")
//         .event(clone!(state => move |_:events::Click| {
//             state.close_menu();
//             // sidebar_state.settings_shown.set(true);
//             sidebar_state.settings.active_popup.set(Some(ActiveSettingsPopup::Main))
//         }))
//     })
// }
