use shared::{
    api::endpoints::course,
    domain::course::{CourseGetDraftPath, CourseId, CourseResponse},
};
use utils::prelude::ApiEndpointExt;

pub async fn load_course(course_id: CourseId) -> anyhow::Result<CourseResponse> {
    course::GetDraft::api_with_auth(CourseGetDraftPath(course_id.clone()), None).await
}
