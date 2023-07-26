use shared::{
    api::endpoints::{self},
    domain::course::{CourseGetDraftPath, CourseId, CourseResponse},
    error::IntoAnyhow,
};
use utils::prelude::{ApiEndpointExt, UnwrapJiExt};

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub(crate) async fn load_course(course_id: &CourseId) -> anyhow::Result<CourseResponse> {
    endpoints::course::GetDraft::api_with_auth(CourseGetDraftPath(course_id.clone()), None)
        .await
        .into_anyhow()
}

impl AssetEditState {
    pub async fn get_course_spots(&self, course: &CourseResponse) {
        let mut items = vec![SidebarSpot::new_course_cover(
            course.course_data.cover.clone().unwrap_ji(),
        )];

        for unit in &course.course_data.units {
            // let unit = get_unit(&course.id, &unit.id).await;

            let unit = unit.clone();

            items.push(SidebarSpot::new_course_unit(unit));
        }

        let mut spots = self.sidebar_spots.lock_mut();

        for item in items {
            spots.push_cloned(item);
        }
    }
}
