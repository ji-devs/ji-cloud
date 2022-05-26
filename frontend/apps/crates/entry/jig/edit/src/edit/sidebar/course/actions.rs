use super::super::state::State;
use futures_signals::signal::Mutable;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        asset::Asset,
        course::{CourseId, CourseResponse, CourseUpdateDraftDataRequest},
    },
    error::EmptyError,
};
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_course(course_id: CourseId, jig_mutable: Mutable<Option<Asset>>) {
    let path = endpoints::course::GetDraft::PATH.replace("{id}", &course_id.0.to_string());

    match api_with_auth::<CourseResponse, EmptyError, ()>(
        &path,
        endpoints::course::GetDraft::METHOD,
        None,
    )
    .await
    {
        Ok(resp) => {
            jig_mutable.set(Some(resp.into()));
        }
        Err(_) => {
            todo!();
        }
    }
}

#[allow(dead_code)] // TODO: remove once used
pub fn navigate_to_publish(state: Rc<State>, course: &CourseResponse) {
    state
        .jig_edit_state
        .set_route_course(CourseEditRoute::Publish);
    state.collapsed.set(true);

    let course_id = course.id;
    Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
        course_id,
        CourseEditRoute::Publish,
    ))));
}

pub async fn update_course(
    course_id: &CourseId,
    req: CourseUpdateDraftDataRequest,
) -> Result<(), EmptyError> {
    let path = endpoints::course::UpdateDraftData::PATH.replace("{id}", &course_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(
        &path,
        endpoints::course::UpdateDraftData::METHOD,
        Some(req),
    )
    .await
}

#[allow(dead_code)] // TODO: remove once used
pub async fn update_display_name(course_id: CourseId, value: String) {
    let req = CourseUpdateDraftDataRequest {
        display_name: Some(value),
        ..Default::default()
    };

    let _ = update_course(&course_id, req).await;
}

// pub fn _player_settings_change_signal(state: Rc<State>) -> impl Signal<Item = CoursePlayerSettings> {
//     let sig = map_ref! {
//         let direction = state.settings.direction.signal_cloned(),
//         let display_score = state.settings.display_score.signal(),
//         let track_assessments = state.settings.track_assessments.signal(),
//         let drag_assist = state.settings.drag_assist.signal()
//         => ( *direction, *display_score, *track_assessments, *drag_assist)
//     };

//     sig.map(
//         |(direction, display_score, track_assessments, drag_assist)| CoursePlayerSettings {
//             direction,
//             display_score,
//             track_assessments,
//             drag_assist,
//         },
//     )
// }

// pub fn get_player_settings(settings_state: Rc<SettingsState>) -> CoursePlayerOptions {
//     let direction = settings_state.direction.get_cloned();
//     let display_score = settings_state.display_score.get();
//     let track_assessments = settings_state.track_assessments.get();
//     let drag_assist = settings_state.drag_assist.get();

//     CoursePlayerOptions {
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
//         .course_edit_state
//         .set_route_course(CourseEditRoute::Module(module_id));
// }
