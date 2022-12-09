use crate::edit::sidebar::state::{CourseSpot, SidebarSpot, SidebarSpotItem};

use super::super::state::Sidebar;
use super::settings::state::State as SettingsState;
use dominator::clone;
use shared::{
    api::endpoints::{self},
    domain::{
        asset::{AssetType, DraftOrLive},
        jig::{JigId, JigUpdateDraftDataPath, JigUpdateDraftDataRequest},
        module::{
            LiteModule, ModuleCreatePath, ModuleCreateRequest, ModuleGetDraftPath, ModuleId,
            ModuleKind, ModuleUpdateRequest, ModuleUploadPath,
        },
    },
};
use std::rc::Rc;
use utils::{asset::JigPlayerOptions, iframe::ModuleToJigEditorMessage, prelude::*};

pub fn navigate_to_publish(state: Rc<Sidebar>) {
    state.collapsed.set(true);
    state.asset_edit_state.navigate_to_publish();
}

pub async fn update_jig(jig_id: &JigId, req: JigUpdateDraftDataRequest) -> anyhow::Result<()> {
    endpoints::jig::UpdateDraftData::api_with_auth_empty(
        JigUpdateDraftDataPath(jig_id.clone()),
        Some(req),
    )
    .await
}

pub async fn update_display_name(jig_id: JigId, value: String) {
    let req = JigUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_jig(&jig_id, req).await;
}

pub fn duplicate_module(state: Rc<Sidebar>, module_id: &ModuleId) {
    state.loader.load(clone!(state, module_id => async move {
        let jig_id = state.asset_edit_state.asset_id.unwrap_jig();
        let module = super::module_cloner::clone_module(&module_id, &jig_id).await.unwrap_ji();
        populate_added_module(state, module);
    }));
}

// pub fn _player_settings_change_signal(state: Rc<State>) -> impl Signal<Item = JigPlayerSettings> {
//     let sig = map_ref! {
//         let direction = state.settings.direction.signal_cloned(),
//         let display_score = state.settings.display_score.signal(),
//         let track_assessments = state.settings.track_assessments.signal(),
//         let drag_assist = state.settings.drag_assist.signal()
//         => ( *direction, *display_score, *track_assessments, *drag_assist)
//     };

//     sig.map(
//         |(direction, display_score, track_assessments, drag_assist)| JigPlayerSettings {
//             direction,
//             display_score,
//             track_assessments,
//             drag_assist,
//         },
//     )
// }

pub fn get_player_settings(settings_state: Rc<SettingsState>) -> JigPlayerOptions {
    let direction = settings_state.jig.direction.get_cloned();
    let display_score = settings_state.jig.display_score.get();
    let track_assessments = settings_state.jig.track_assessments.get();
    let drag_assist = settings_state.jig.drag_assist.get();

    JigPlayerOptions {
        direction,
        display_score,
        track_assessments,
        drag_assist,
        is_student: false,
        draft_or_live: DraftOrLive::Draft,
    }
}

// TODO: move out of jig dir
pub fn on_iframe_message(state: Rc<Sidebar>, message: ModuleToJigEditorMessage) {
    match message {
        ModuleToJigEditorMessage::AppendModule(module) => {
            populate_added_module(Rc::clone(&state), module);
        }
        ModuleToJigEditorMessage::Complete(complete_module_id, is_complete) => {
            let modules = state.asset_edit_state.sidebar_spots.lock_ref();
            let module = modules.iter().find(|module| {
                // Oh my.
                // only modules should be here, either jig.modules or any asset cover
                let current_module_id = match &module.item {
                    SidebarSpotItem::Jig(module) => module.as_ref().map(|module| module.id),
                    SidebarSpotItem::Course(item) => item.as_ref().map(|item| match &**item {
                        CourseSpot::Cover(module) => module.id,
                        CourseSpot::Item(_) => unreachable!("Only modules should be here"),
                    }),
                };
                match current_module_id {
                    Some(current_module_id) => current_module_id == complete_module_id,
                    None => false,
                }
            });

            if let Some(module) = module {
                module.is_incomplete.set_neq(!is_complete);
            }
        }
        ModuleToJigEditorMessage::Next => {
            state.asset_edit_state.set_route_jig(JigEditRoute::Landing);
            let jig_id = *state.asset_edit_state.asset_id.unwrap_jig();
            Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                jig_id,
                JigEditRoute::Landing,
            ))));
        }
    }
}

fn populate_added_module(state: Rc<Sidebar>, module: LiteModule) {
    // Assumes that the final module in the list is always the placeholder module.
    let insert_at_idx = state.asset_edit_state.sidebar_spots.lock_ref().len() - 1;

    let module_id = module.id;

    state
        .asset_edit_state
        .sidebar_spots
        .lock_mut()
        .insert_cloned(insert_at_idx, SidebarSpot::new_jig_module(Some(module)));

    state
        .asset_edit_state
        .set_route_jig(JigEditRoute::Module(module_id));
}

pub fn use_module_as(state: Rc<Sidebar>, target_kind: ModuleKind, source_module_id: ModuleId) {
    state.loader.load(clone!(state => async move {
        let target_module_id: anyhow::Result<(ModuleId, bool)> = async {
            let asset_type: AssetType = (&state.asset_edit_state.asset_id).into();
            let source_module = endpoints::module::GetDraft::api_with_auth(
                ModuleGetDraftPath(asset_type, source_module_id.clone()),
                None
            ).await?.module;

            let target_body = source_module.body.convert_to_body(target_kind).unwrap_ji();

            let req = ModuleCreateRequest {
                body: target_body,
                parent_id: state.asset_edit_state.asset.id(),
            };

            let res = endpoints::module::Create::api_with_auth(
                ModuleCreatePath(),
                Some(req),
            )
            .await?;

            Ok((res.id, source_module.is_complete))
        }.await;

        match target_module_id {
            Err(_) => {
                log::error!("request to create module failed!");
            },
            Ok((target_module_id, is_complete)) => {
                let lite_module = LiteModule {
                    id: target_module_id,
                    kind: target_kind,
                    is_complete,
                };
                populate_added_module(Rc::clone(&state), lite_module);
            },
        };
    }));
}

pub async fn update_module(
    // jig_id: &JigId,
    module_id: &ModuleId,
    req: ModuleUpdateRequest,
) -> anyhow::Result<()> {
    endpoints::module::Update::api_with_auth_empty(ModuleUploadPath(module_id.clone()), Some(req))
        .await
}
