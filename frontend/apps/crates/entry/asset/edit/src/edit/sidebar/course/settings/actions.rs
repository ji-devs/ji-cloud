use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::course::CourseUpdateDraftDataPath};
use utils::prelude::ApiEndpointExt;

use super::state::CourseSettings;

impl CourseSettings {
    pub fn update_course_settings(self: &Rc<Self>) {
        let state = self;
        let req = state.get_course_update_req();

        state.loader.load(clone!(state => async move {
            let _ = endpoints::course::UpdateDraftData::api_with_auth(
                CourseUpdateDraftDataPath(state.course.id),
                Some(req),
            )
            .await;
        }));
    }
}
