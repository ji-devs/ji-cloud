use super::super::super::jig::actions as jig_actions;
use super::super::super::spot::state::SpotState;
use crate::edit::sidebar::state::{SidebarSpot, SidebarSpotItem};
use shared::{api::endpoints, domain::module::*};
use std::rc::Rc;
use utils::prelude::*;

pub fn edit(state: Rc<SpotState>) {
    let jig_id = *state.sidebar.asset_edit_state.asset_id.unwrap_jig();

    if let SidebarSpotItem::Jig(Some(module)) = &state.spot.item {
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

pub async fn delete(state: &Rc<SpotState>, module: &Option<Rc<LiteModule>>) {
    if let Some(module) = module {
        let req = ModuleDeleteRequest {
            parent_id: state.sidebar.asset_edit_state.asset_id,
        };

        endpoints::module::Delete::api_with_auth(ModuleDeletePath(module.id), Some(req))
            .await
            .unwrap_ji();
    }
}

pub async fn assign_module_to_empty_spot(
    state: &Rc<SpotState>,
    module_kind: ModuleKind,
) -> Option<Rc<SidebarSpot>> {
    if state.index == 0 && module_kind != ModuleKind::Cover {
        return None;
    }

    // Remove module highlights whenever a new module is added to the list.
    state.sidebar.highlight_modules.set_neq(None);

    let jig_id = *state.sidebar.asset_edit_state.asset_id.unwrap_jig();

    let req = Some(ModuleCreateRequest {
        parent_id: jig_id.into(),
        body: ModuleBody::new(module_kind),
    });

    let resp = endpoints::module::Create::api_with_auth(ModuleCreatePath(), req)
        .await
        .unwrap_ji();

    let id = resp.id;
    let index = state.index;

    let req = ModuleUpdateRequest {
        // id: StableOrUniqueId::Unique(id),
        parent_id: jig_id.into(),
        index: Some(index.try_into().unwrap_ji()),
        body: None,
        is_complete: None,
    };

    jig_actions::update_module(&id, req).await.unwrap_ji();

    state.sidebar.collapsed.set(true);
    state
        .sidebar
        .asset_edit_state
        .set_route_jig(JigEditRoute::Module(id));
    Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
        jig_id,
        JigEditRoute::Module(id),
    ))));

    Some(SidebarSpot::new_jig_module(Some(LiteModule {
        id,
        kind: module_kind,
        is_complete: false,
    })))
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
