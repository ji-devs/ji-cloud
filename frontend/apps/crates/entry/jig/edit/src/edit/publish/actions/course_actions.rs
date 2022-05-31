use shared::{
    api::endpoints::{course, ApiEndpoint},
    domain::course::{CourseId, CourseResponse, CourseUpdateDraftDataRequest},
    error::{EmptyError, MetadataNotFound},
};
use utils::{
    prelude::{api_with_auth, api_with_auth_empty},
    routes::{AssetEditRoute, AssetRoute, CourseEditRoute, Route},
};

use crate::edit::publish::editable_assets::EditableAsset;

use super::super::editable_assets::EditableCourse;

pub async fn save_and_publish_course(course: &EditableCourse) -> Result<(), ()> {
    let path = course::UpdateDraftData::PATH.replace("{id}", &course.id.0.to_string());
    let req = course.to_course_update_request();
    api_with_auth_empty::<MetadataNotFound, CourseUpdateDraftDataRequest>(
        &path,
        course::UpdateDraftData::METHOD,
        Some(req),
    )
    .await
    .map_err(|_| ())?;

    let path = course::Publish::PATH.replace("{id}", &course.id.0.to_string());
    api_with_auth_empty::<EmptyError, ()>(&path, course::Publish::METHOD, None)
        .await
        .map_err(|_| ())?;

    let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
        course.id,
        CourseEditRoute::PostPublish,
    )))
    .into();
    log::info!("{}", url);

    /* this will cause a full refresh - but preserves history
    * see the .future in EditPage too
    dominator::routing::go_to_url(&url);
    */

    Ok(())
}

pub async fn load_course(course_id: CourseId) -> Result<EditableAsset, ()> {
    let path = course::GetDraft::PATH.replace("{id}", &course_id.0.to_string());

    api_with_auth::<CourseResponse, EmptyError, ()>(&path, course::GetDraft::METHOD, None)
        .await
        .map(|course| EditableAsset::Course(EditableCourse::new(course)))
        .map_err(|_| ())
}
