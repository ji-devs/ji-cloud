use super::super::state::Sidebar;
use shared::{
    api::endpoints,
    domain::{pro_dev::{
        unit::{ProDevUnitId, ProDevUnitUpdateRequest, UpdateProDevUnitPath},
        ProDevId, ProDevUpdateDraftDataPath, ProDevUpdateDraftDataRequest,
    }},
};
use std::rc::Rc;
use utils::prelude::*;

#[allow(dead_code)] // TODO: remove once used
pub fn navigate_to_publish(state: Rc<Sidebar>) {
    state.collapsed.set(true);
    state.asset_edit_state.navigate_to_publish();
}

pub async fn update_pro_dev(
    pro_dev_id: &ProDevId,
    req: ProDevUpdateDraftDataRequest,
) -> anyhow::Result<()> {
    endpoints::pro_dev::UpdateDraftData::api_with_auth_empty(
        ProDevUpdateDraftDataPath(pro_dev_id.clone()),
        Some(req),
    )
    .await
}

pub async fn update_unit(
    pro_dev_id: &ProDevId,
    unit_id: &ProDevUnitId,
    req: ProDevUnitUpdateRequest,
) -> anyhow::Result<()> {
    endpoints::pro_dev::unit::Update::api_with_auth_empty(
        UpdateProDevUnitPath(pro_dev_id.clone(), unit_id.clone()),
        Some(req),
    )
    .await
}

pub async fn update_display_name(pro_dev_id: ProDevId, value: String) {
    let req = ProDevUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_pro_dev(&pro_dev_id, req).await;
}


// pub fn _player_settings_change_signal(state: Rc<State>) -> impl Signal<Item = ProDevPlayerSettings> {
//     let sig = map_ref! {
//         let direction = state.settings.direction.signal_cloned(),
//         let display_score = state.settings.display_score.signal(),
//         let track_assessments = state.settings.track_assessments.signal(),
//         let drag_assist = state.settings.drag_assist.signal()
//         => ( *direction, *display_score, *track_assessments, *drag_assist)
//     };

//     sig.map(
//         |(direction, display_score, track_assessments, drag_assist)| ProDevPlayerSettings {
//             direction,
//             display_score,
//             track_assessments,
//             drag_assist,
//         },
//     )
// }

// pub fn get_player_settings(settings_state: Rc<SettingsState>) -> ProDevPlayerOptions {
//     let direction = settings_state.direction.get_cloned();
//     let display_score = settings_state.display_score.get();
//     let track_assessments = settings_state.track_assessments.get();
//     let drag_assist = settings_state.drag_assist.get();

//     ProDevPlayerOptions {
//         direction,
//         display_score,
//         track_assessments,
//         drag_assist,
//         is_student: false,
//         draft: true,
//     }
// }

// fn populate_added_module(state: Rc<State>, module: LiteModule) {
//     // Assumes that the final module in the list is always the placeholder module.
//     let insert_at_idx = state.modules.lock_ref().len() - 1;

//     let module_id = module.id;

//     state
//         .modules
//         .lock_mut()
//         .insert_cloned(insert_at_idx, Rc::new(module.into()));

//     state
//         .pro_dev_edit_state
//         .set_route_pro_dev(ProDevEditRoute::Module(module_id));
// }
