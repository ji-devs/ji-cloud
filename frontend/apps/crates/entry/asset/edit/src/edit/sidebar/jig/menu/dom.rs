use super::state::*;
use crate::edit::sidebar::{
    jig::actions as jig_actions,
    jig::copy_paste_module::{copy_module, paste_module},
    spot::actions::{self, MoveTarget},
    spot::jig::actions as jig_spot_actions,
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

impl JigMenu {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("menu-kebab", {
            .prop("slot", "menu")
            .child(html!("jig-edit-sidebar-module-menu", {
                .children(state.menu_items())
            }))
            .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
                e.stop_propagation();
            })
            .after_inserted(move |elem| {
                *state.menu_ref.borrow_mut() = Some(elem);
            })
        })
    }

    fn menu_items(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let module = state.spot_state.spot.item.unwrap_jig();
        state.menu_items_jig(module)
    }

    fn menu_items_jig(self: &Rc<Self>, module: &Option<Rc<LiteModule>>) -> Vec<Dom> {
        let state = self;
        match state.spot_state.index {
            0 => {
                vec![
                    state.item_edit(),
                    // TODO:
                    // item_copy(state.clone()),
                    state.item_paste(&state.spot_state.sidebar),
                ]
            }
            _ => {
                let mut v = vec![];
                if let Some(module) = module {
                    v.push(state.item_edit());
                    if state.spot_state.index > 1 {
                        // We only want to be able to move up if there's somewhere
                        // to move to. Index 0 is occupied by the Cover module, so
                        // anything at 1 cannot go higher.
                        v.push(state.item_move_up());
                    }
                    if state.spot_state.is_last_module() {
                        v.push(state.item_move_down());
                    }
                    v.push(state.item_duplicate(&state.spot_state.sidebar, module.id));
                }
                v.push(state.item_delete());
                if let Some(module) = module {
                    v.push(state.item_copy(&state.spot_state.sidebar, module.id));
                    v.push(state.item_duplicate_as(&state.spot_state.sidebar, module));
                }
                v
            }
        }
    }

    fn item_edit(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "edit")
            .event(clone!(state => move |_:events::Click| {
                jig_spot_actions::edit(state.spot_state.clone());
            }))
        })
    }

    fn item_move_up(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "move-up")
            .event(clone!(state => move |_:events::Click| {
                state.close_menu();
                actions::move_index(state.spot_state.clone(), MoveTarget::Up);
            }))
        })
    }

    fn item_move_down(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "move-down")
            .event(clone!(state => move |_:events::Click| {
                state.close_menu();
                actions::move_index(state.spot_state.clone(), MoveTarget::Down);
            }))
        })
    }

    fn item_duplicate(
        self: &Rc<Self>,
        sidebar_state: &Rc<SidebarState>,
        module_id: ModuleId,
    ) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "duplicate")
            .event(clone!(state, sidebar_state => move |_:events::Click| {
                state.close_menu();
                jig_actions::duplicate_module(sidebar_state.clone(), &module_id);
            }))
        })
    }

    fn item_delete(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "delete")
            .event(clone!(state => move |_:events::Click| {
                state.spot_state.confirm_delete.set_neq(true);
                state.close_menu();
            }))
        })
    }

    fn item_copy(self: &Rc<Self>, sidebar_state: &Rc<SidebarState>, module_id: ModuleId) -> Dom {
        let state = self;
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

    fn item_paste(self: &Rc<Self>, sidebar_state: &Rc<SidebarState>) -> Dom {
        let state = self;
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
        self: &Rc<Self>,
        sidebar_state: &Rc<SidebarState>,
        module: &LiteModule,
    ) -> Dom {
        let state = self;

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
}
