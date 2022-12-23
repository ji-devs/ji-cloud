use shared::{
    api::endpoints::course,
    domain::course::{
        CourseGetDraftPath, CoursePublishPath, CourseResponse, CourseUpdateDraftDataPath,
    },
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditableCourse;

pub async fn save_and_publish_course(course: &EditableCourse) -> anyhow::Result<CourseResponse> {
    let req = course.to_course_update_request();
    course::UpdateDraftData::api_with_auth_empty(CourseUpdateDraftDataPath(course.id), Some(req))
        .await?;

    course::Publish::api_with_auth_empty(CoursePublishPath(course.id), None).await?;

    let course = course::GetDraft::api_with_auth(CourseGetDraftPath(course.id), None).await?;

    Ok(course)
}
