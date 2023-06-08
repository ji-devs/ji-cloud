use std::{rc::Rc, str::FromStr};

use dominator::clone;
use shared::domain::module::ModuleId;
use utils::{storage::get_local_storage, unwrap::UnwrapJiExt};

use crate::edit::sidebar::state::SidebarSpot;

use super::super::state::Sidebar;

pub const COPY_MODULE_KEY: &str = "COPY_MODULE";
pub fn copy_module(state: Rc<Sidebar>, module_id: &ModuleId) {
    let value = format!(
        "{},{}",
        &state.asset_edit_state.asset_id.uuid(),
        &module_id.0
    );

    let local_storage = get_local_storage().unwrap_ji();

    local_storage.set(COPY_MODULE_KEY, &value).unwrap_ji();
}
fn get_module_to_paste() -> Option<ModuleId> {
    let value = get_local_storage()
        .unwrap_ji()
        .get(COPY_MODULE_KEY)
        .unwrap_ji();

    match value {
        None => None,
        Some(value) => {
            let value: Vec<&str> = value.split(',').collect();
            // let jig_id = JigId(Uuid::from_str(value[0]).unwrap_ji());
            let module_id = ModuleId::from_str(value[1]).unwrap_ji();
            // value
            log::info!("{:?}{:?}", value, 90);

            Some(module_id)
        }
    }
}
pub fn paste_module(state: Rc<Sidebar>) {
    match get_module_to_paste() {
        None => log::warn!("No module to paste"),
        Some(module_id) => {
            state.loader.load(clone!(state => async move {
                let module = super::module_cloner::clone_module(&module_id, state.asset_edit_state.asset_id.unwrap_jig()).await.unwrap_ji();
                let mut modules = state.asset_edit_state.sidebar_spots.lock_mut();
                modules.insert_cloned(modules.len() - 2, SidebarSpot::new_jig_module(Some(module)));
            }));
        }
    }
}
