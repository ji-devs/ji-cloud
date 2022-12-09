use shared::{
    api::endpoints::course,
    domain::course::{CourseGetDraftPath, CourseId, CourseResponse},
};
use utils::prelude::ApiEndpointExt;

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub async fn load_course(course_id: CourseId) -> anyhow::Result<CourseResponse> {
    course::GetDraft::api_with_auth(CourseGetDraftPath(course_id.clone()), None).await
}

impl AssetEditState {
    pub fn get_course_spots(&self, course: &CourseResponse) {
        let mut items = vec![SidebarSpot::new_course_cover(
            course.course_data.cover.clone().unwrap(),
        )];
        for item in &course.course_data.items {
            items.push(SidebarSpot::new_course_item(*item));
        }

        let mut spots = self.sidebar_spots.lock_mut();
        for item in items {
            spots.push_cloned(item);
        }
    }
}
