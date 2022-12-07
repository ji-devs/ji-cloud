use super::super::super::jig::actions as jig_actions;
use super::super::super::spot::state::State as SpotState;
use crate::edit::sidebar::state::{SidebarSpot, SidebarSpotItem};
use dominator::clone;
use shared::{api::endpoints, domain::module::*};
use std::rc::Rc;
use utils::prelude::*;

pub fn edit(state: Rc<SpotState>) {
    let jig_id = *state.sidebar.asset_edit_state.asset_id.unwrap_jig();

    if let SidebarSpotItem::Jig(Some(module)) = &state.module.item {
        let module_id = module.id;
        state
            .sidebar
            .asset_edit_state
            .route
            .set(AssetEditRoute::Jig(jig_id, JigEditRoute::Module(module_id)));
        state.sidebar.collapsed.set(true);

        Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
            jig_id,
            JigEditRoute::Module(module_id),
        ))));
    }
}

pub async fn delete(state: Rc<SpotState>) {
    if let Some(module) = &*state.module.item.unwrap_jig() {
        let req = ModuleDeleteRequest {
            parent_id: state.sidebar.asset_edit_state.asset_id,
        };

        endpoints::module::Delete::api_with_auth_empty(ModuleDeletePath(module.id), Some(req))
            .await
            .unwrap();
    }
}

pub fn assign_kind(state: Rc<SpotState>, kind: ModuleKind) {
    state.sidebar.loader.load(clone!(state => async move {
        let jig_id = *state.sidebar.asset_edit_state.asset_id.unwrap_jig();

        let req = Some(ModuleCreateRequest {
            parent_id: jig_id.into(),
            body: ModuleBody::new(kind),
        });

        match endpoints::module::Create::api_with_auth(
            ModuleCreatePath(),
            req
        ).await {
            Ok(resp) => {
                let id = resp.id;
                let index = state.index;

                let module = SidebarSpot::new_jig_module(Some(LiteModule {
                    id,
                    kind,
                    is_complete: false,
                }));

                {
                    // Instead of replacing the module at the index, we remove the old module and
                    // add the new one. This is slightly less efficient because it fires signals
                    // for the entire list of modules, however, it is necessary so that the modules
                    // before and after this one can have their views updated.
                    let mut modules = state.sidebar.spots.lock_mut();
                    modules.remove(index);
                    modules.insert_cloned(index, module);

                    // Only add a new placeholder module once the above request has completed and
                    // the new module has been added to the list of modules.
                    let placeholder_exists = {
                        match modules.last() {
                            // If the list of modules is not empty and the last module is None, then it is
                            // a placeholder module.
                            Some(module) => module.item.is_none(),
                            // When the list is empty or the last module is not a placeholder module.
                            _ => false,
                        }
                    };

                    // if this is the empty module at the end
                    if !placeholder_exists {
                        modules.push_cloned(Rc::new(SidebarSpot::new_empty(&state.sidebar.asset_edit_state.asset_id)));
                    }
                }

                let req = ModuleUpdateRequest {
                    // id: StableOrUniqueId::Unique(id),
                    parent_id: jig_id.into(),
                    index: Some(index.try_into().unwrap_ji()),
                    body: None,
                    is_complete: None,
                };

                match jig_actions::update_module(&id, req).await {
                    Ok(_) => {
                        state.sidebar.collapsed.set(true);
                        state.sidebar.asset_edit_state.set_route_jig(JigEditRoute::Module(id));
                        Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                            jig_id,
                            JigEditRoute::Module(id)
                        ))));
                    },
                    Err(e) => {
                        log::error!("Error: {:?}", e);
                    },
                }
            },
            Err(e) => {
                log::error!("Error: {:?}", e);
            },
        }
    }));
}

pub async fn update_module_index(state: Rc<SpotState>, module: &LiteModule, index: u16) {
    let req = ModuleUpdateRequest {
        // id: StableOrUniqueId::Unique(module.id),
        parent_id: state.sidebar.asset_edit_state.asset.id(),
        index: Some(index),
        body: None,
        is_complete: None,
    };

    let _ = jig_actions::update_module(&module.id, req).await;
}
