use shared::{
    api::endpoints::course,
    domain::course::{
        CourseGetDraftPath, CourseId, CoursePublishPath, CourseResponse, CourseUpdateDraftDataPath,
    },
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableCourse;

pub async fn save_course(course: &EditableCourse) -> anyhow::Result<()> {
    let req = course.to_course_update_request();

    course::UpdateDraftData::api_with_auth_empty(CourseUpdateDraftDataPath(course.id), Some(req))
        .await
}

pub async fn publish_course(course_id: CourseId) -> anyhow::Result<CourseResponse> {
    course::Publish::api_with_auth_empty(CoursePublishPath(course_id), None).await?;

    let course = course::GetDraft::api_with_auth(CourseGetDraftPath(course_id), None).await?;

    Ok(course)
}
