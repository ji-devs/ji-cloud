use crate::edit::sidebar::state::SidebarSpot;

use super::super::state::State;
use super::settings::state::State as SettingsState;
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        asset::{Asset, AssetType, DraftOrLive},
        jig::{JigFocus, JigId, JigResponse, JigUpdateDraftDataRequest},
        module::{
            LiteModule, ModuleCreateRequest, ModuleId, ModuleKind, ModuleResponse,
            ModuleUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};
use std::rc::Rc;
use utils::{iframe::ModuleToJigEditorMessage, jig::JigPlayerOptions, prelude::*};

pub async fn load_jig(jig_id: JigId, jig_mutable: Mutable<Option<Asset>>) {
    let path = endpoints::jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(
        &path,
        endpoints::jig::GetDraft::METHOD,
        None,
    )
    .await
    {
        Ok(resp) => {
            assert!(
                resp.jig_focus.is_modules(),
                "only module focused jigs should be here"
            );
            jig_mutable.set(Some(resp.into()));
        }
        Err(_) => {
            todo!();
        }
    }
}

pub fn navigate_to_publish(state: Rc<State>, jig: &JigResponse) {
    state.jig_edit_state.set_route_jig(JigEditRoute::Publish);
    state.collapsed.set(true);

    let jig_id = jig.id;
    Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
        jig_id,
        jig.jig_focus,
        JigEditRoute::Publish,
    ))));
}

pub async fn update_jig(jig_id: &JigId, req: JigUpdateDraftDataRequest) -> Result<(), EmptyError> {
    let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::UpdateDraftData::METHOD, Some(req))
        .await
}

pub async fn update_display_name(jig_id: JigId, value: String) {
    let req = JigUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_jig(&jig_id, req).await;
}

pub fn duplicate_module(state: Rc<State>, module_id: &ModuleId) {
    state.loader.load(clone!(state, module_id => async move {
        let jig_id = state.asset.unwrap_jig().id;
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
    let direction = settings_state.direction.get_cloned();
    let display_score = settings_state.display_score.get();
    let track_assessments = settings_state.track_assessments.get();
    let drag_assist = settings_state.drag_assist.get();

    JigPlayerOptions {
        direction,
        display_score,
        track_assessments,
        drag_assist,
        is_student: false,
        draft_or_live: DraftOrLive::Draft,
    }
}

pub fn on_iframe_message(state: Rc<State>, message: ModuleToJigEditorMessage) {
    match message {
        ModuleToJigEditorMessage::AppendModule(module) => {
            populate_added_module(Rc::clone(&state), module);
        }
        ModuleToJigEditorMessage::Complete(module_id, is_complete) => {
            let modules = state.modules.lock_ref();
            let module = modules.iter().find(|module| {
                // Oh my.
                match &*module.item.unwrap_jig() {
                    Some(module) => module.id == module_id,
                    None => false,
                }
            });

            if let Some(module) = module {
                module.is_incomplete.set_neq(!is_complete);
            }
        }
        ModuleToJigEditorMessage::Next => {
            state.jig_edit_state.set_route_jig(JigEditRoute::Landing);
            let jig_id = state.asset.unwrap_jig().id;
            Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                jig_id,
                JigFocus::Modules,
                JigEditRoute::Landing,
            ))));
        }
        ModuleToJigEditorMessage::Publish => {
            navigate_to_publish(Rc::clone(&state), state.asset.unwrap_jig());
        }
    }
}

fn populate_added_module(state: Rc<State>, module: LiteModule) {
    // Assumes that the final module in the list is always the placeholder module.
    let insert_at_idx = state.modules.lock_ref().len() - 1;

    let module_id = module.id;

    state
        .modules
        .lock_mut()
        .insert_cloned(insert_at_idx, SidebarSpot::new_jig_module(Some(module)));

    state
        .jig_edit_state
        .set_route_jig(JigEditRoute::Module(module_id));
}

pub fn use_module_as(state: Rc<State>, target_kind: ModuleKind, source_module_id: ModuleId) {
    state.loader.load(clone!(state => async move {
        let target_module_id: Result<(ModuleId, bool), EmptyError> = async {
            let asset_type = match state.asset {
                Asset::Jig(_) => AssetType::Jig,
                Asset::Course(_) => AssetType::Course,
            };
            let path = endpoints::module::GetDraft::PATH
                .replace("{asset_type}",asset_type.as_str())
                .replace("{module_id}", &source_module_id.0.to_string());

            let source_module = api_with_auth::<ModuleResponse, EmptyError, ()>(
                &path,
                endpoints::module::GetDraft::METHOD,
                None
            ).await?.module;

            let target_body = source_module.body.convert_to_body(target_kind).unwrap_ji();

            let req = ModuleCreateRequest {
                body: target_body,
                parent_id: state.asset.id(),
            };

            let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
                endpoints::module::Create::PATH,
                endpoints::module::Create::METHOD,
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
) -> Result<(), EmptyError> {
    let path = endpoints::module::Update::PATH
        // .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::module::Update::METHOD, Some(req)).await
}
