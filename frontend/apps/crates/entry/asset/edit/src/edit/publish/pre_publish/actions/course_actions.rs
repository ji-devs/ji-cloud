use shared::{
    api::endpoints::course,
    domain::course::{
        CourseGetDraftPath, CourseId, CoursePublishPath, CourseResponse, CourseUpdateDraftDataPath,
    },
    error::IntoAnyhow,
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableCourse;

pub async fn save_course(course: &EditableCourse) -> anyhow::Result<()> {
    let req = course.to_course_update_request();

    course::UpdateDraftData::api_with_auth(CourseUpdateDraftDataPath(course.id), Some(req))
        .await
        .into_anyhow()
}

pub async fn publish_course(course_id: CourseId) -> anyhow::Result<CourseResponse> {
    course::Publish::api_with_auth(CoursePublishPath(course_id), None).await?;

    let course = course::GetDraft::api_with_auth(CourseGetDraftPath(course_id), None).await?;

    Ok(course)
}
