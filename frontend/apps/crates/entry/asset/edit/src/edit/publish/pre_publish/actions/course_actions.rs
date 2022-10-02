use shared::{
    api::endpoints::course,
    domain::course::{CoursePublishPath, CourseUpdateDraftDataPath},
};
use utils::prelude::ApiEndpointExt;

use super::super::editable_assets::EditableCourse;

pub async fn save_and_publish_course(course: &EditableCourse) -> anyhow::Result<()> {
    let req = course.to_course_update_request();
    course::UpdateDraftData::api_with_auth_empty(
        CourseUpdateDraftDataPath(course.id.clone()),
        Some(req),
    )
    .await?;

    course::Publish::api_with_auth_empty(CoursePublishPath(course.id.clone()), None).await?;

    Ok(())
}
