use std::rc::Rc;

use shared::{
    api::endpoints::{self, course},
    domain::{
        course::{CourseGetDraftPath, CourseId, CourseResponse},
        jig::{JigGetLivePath, JigId, JigResponse},
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub async fn load_course(course_id: CourseId) -> anyhow::Result<CourseResponse> {
    course::GetDraft::api_with_auth(CourseGetDraftPath(course_id.clone()), None).await
}

impl AssetEditState {
    pub async fn get_course_spots(&self, course: &CourseResponse) {
        let mut items = vec![SidebarSpot::new_course_cover(
            course.course_data.cover.clone().unwrap(),
        )];
        for jig_id in &course.course_data.items {
            let jig = get_jig(jig_id).await;

            items.push(SidebarSpot::new_course_item(jig));
        }

        // add empty at the end
        items.push(Rc::new(SidebarSpot::new_empty(&course.id.into())));

        let mut spots = self.sidebar_spots.lock_mut();
        for item in items {
            spots.push_cloned(item);
        }
    }
}

async fn get_jig(jig_id: &JigId) -> JigResponse {
    endpoints::jig::GetLive::api_with_auth(JigGetLivePath(jig_id.clone()), None)
        .await
        .unwrap_ji()
}
