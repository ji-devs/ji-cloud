use super::state::*;
use crate::edit::sidebar::{
    actions::duplicate_module,
    copy_paste_module::{copy_module, paste_module},
    module::{
        actions::{self, MoveTarget},
        state::State as ModuleState,
    },
    settings::state::ActiveSettingsPopup,
    state::State as SidebarState,
};
use dominator::{clone, html, Dom};
use shared::domain::jig::module::ModuleId;
use std::rc::Rc;
use utils::events;

const STR_COPY: &'static str = "Copy to another Jig";
const STR_PASTE: &'static str = "Paste from another JIG";
const STR_DUPLICATE_AS: &'static str = "Duplicate content as:";
const STR_EDIT_SETTINGS: &'static str = "Edit setting";

pub fn render(state: Rc<State>, items: Vec<Dom>) -> Dom {
    html!("menu-kebab", {
        .property("slot", "menu")
        .child(html!("jig-edit-sidebar-module-menu", {
            .children(items)
        }))
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

pub fn item_duplicate_as(state: Rc<State>) -> Dom {
    html!("menu-line", {
        .property("slot", "advanced")
        .property("customLabel", STR_DUPLICATE_AS)
        .property("icon", "reuse")
        .event(clone!(state => move |_:events::Click| {
            state.close_menu();
        }))
    })
}

pub fn item_edit_settings(state: Rc<State>, sidebar_state: Rc<SidebarState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("customLabel", STR_EDIT_SETTINGS)
        .property("icon", "edit")
        .event(clone!(state => move |_:events::Click| {
            state.close_menu();
            // sidebar_state.settings_shown.set(true);
            sidebar_state.settings.active_popup.set(Some(ActiveSettingsPopup::Main))
        }))
    })
}
