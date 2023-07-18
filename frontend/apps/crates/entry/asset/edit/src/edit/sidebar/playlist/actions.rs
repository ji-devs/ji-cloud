use super::super::state::Sidebar;
use shared::{
    api::endpoints,
    domain::playlist::{PlaylistId, PlaylistUpdateDraftDataPath, PlaylistUpdateDraftDataRequest},
};
use std::rc::Rc;
use utils::prelude::*;

#[allow(dead_code)] // TODO: remove once used
pub fn navigate_to_publish(state: Rc<Sidebar>) {
    state.collapsed.set(true);
    state.asset_edit_state.navigate_to_publish();
}

pub async fn update_playlist(
    playlist_id: &PlaylistId,
    req: PlaylistUpdateDraftDataRequest,
) -> anyhow::Result<()> {
    endpoints::playlist::UpdateDraftData::api_with_auth(
        PlaylistUpdateDraftDataPath(playlist_id.clone()),
        Some(req),
    )
    .await
    .into_anyhow()
}

pub async fn update_display_name(playlist_id: PlaylistId, value: String) {
    let req = PlaylistUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_playlist(&playlist_id, req).await;
}

// pub fn _player_settings_change_signal(state: Rc<State>) -> impl Signal<Item = PlaylistPlayerSettings> {
//     let sig = map_ref! {
//         let direction = state.settings.direction.signal_cloned(),
//         let display_score = state.settings.display_score.signal(),
//         let track_assessments = state.settings.track_assessments.signal(),
//         let drag_assist = state.settings.drag_assist.signal()
//         => ( *direction, *display_score, *track_assessments, *drag_assist)
//     };

//     sig.map(
//         |(direction, display_score, track_assessments, drag_assist)| PlaylistPlayerSettings {
//             direction,
//             display_score,
//             track_assessments,
//             drag_assist,
//         },
//     )
// }

// pub fn get_player_settings(settings_state: Rc<SettingsState>) -> PlaylistPlayerOptions {
//     let direction = settings_state.direction.get_cloned();
//     let display_score = settings_state.display_score.get();
//     let track_assessments = settings_state.track_assessments.get();
//     let drag_assist = settings_state.drag_assist.get();

//     PlaylistPlayerOptions {
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
//         .playlist_edit_state
//         .set_route_playlist(PlaylistEditRoute::Module(module_id));
// }
