use shared::{
    api::endpoints::course,
    domain::course::{CourseGetDraftPath, CourseId, CoursePublishPath, CourseUpdateDraftDataPath},
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, CourseEditRoute, Route},
};

use crate::edit::publish::editable_assets::EditableAsset;

use super::super::editable_assets::EditableCourse;

pub async fn save_and_publish_course(course: &EditableCourse) -> anyhow::Result<()> {
    let req = course.to_course_update_request();
    course::UpdateDraftData::api_with_auth_empty(
        CourseUpdateDraftDataPath(course.id.clone()),
        Some(req),
    )
    .await?;

    course::Publish::api_with_auth_empty(CoursePublishPath(course.id.clone()), None).await?;

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

pub async fn load_course(course_id: CourseId) -> anyhow::Result<EditableAsset> {
    course::GetDraft::api_with_auth(CourseGetDraftPath(course_id.clone()), None)
        .await
        .map(|course| EditableAsset::Course(EditableCourse::new(course)))
}
