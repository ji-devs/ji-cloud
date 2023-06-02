use shared::domain::course::unit::{
    CourseUnit, CourseUnitCreateRequest, CourseUnitId, CreateCourseUnitPath,
};
use shared::{api::endpoints::course, domain::course::CourseId};
use utils::prelude::ApiEndpointExt;

pub async fn clone_unit(
    orig_unit: &CourseUnit,
    course_id: &CourseId,
) -> anyhow::Result<CourseUnit> {
    let id = create_unit(course_id, orig_unit.clone()).await?;
    Ok(CourseUnit {
        id,
        display_name: orig_unit.display_name.clone(),
        description: orig_unit.description.clone(),
        value: orig_unit.value.clone(),
    })
}

async fn create_unit(
    course_id: &CourseId,
    course_unit: CourseUnit,
) -> anyhow::Result<CourseUnitId> {
    let req = CourseUnitCreateRequest {
        display_name: course_unit.display_name,
        description: course_unit.description,
        value: course_unit.value,
    };

    let res =
        course::unit::Create::api_with_auth(CreateCourseUnitPath(course_id.clone()), Some(req))
            .await?;
    Ok(res.id)
}
