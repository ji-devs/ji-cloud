use std::rc::Rc;

use dominator::clone;
use shared::{domain::course::{CourseUpdateDraftDataRequest, CourseResponse}, api::{endpoints, ApiEndpoint}, error::EmptyError};
use utils::prelude::{api_with_auth_empty, api_with_auth};

use super::state::JigSelection;

impl JigSelection {
    pub fn load_course(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let req = CourseUpdateDraftDataRequest {
                items: Some(state.jigs.lock_ref().to_vec()),
                ..Default::default()
            };

            let path = endpoints::course::GetDraft::PATH.replace(
                "{id}",
                &state.course_id.0.to_string()
            );

            let res = api_with_auth::<CourseResponse, EmptyError, _>(
                &path,
                endpoints::course::GetDraft::METHOD,
                Some(req),
            )
            .await;

            match res {
                Ok(course) => {
                    let items = course.course_data.items;
                    state.jigs.lock_mut().replace(items);
                },
                Err(_) => todo!(),
            }
        }));
    }

    pub fn save_course(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let req = CourseUpdateDraftDataRequest {
                items: Some(state.jigs.lock_ref().to_vec()),
                ..Default::default()
            };

            let path = endpoints::course::UpdateDraftData::PATH.replace(
                "{id}",
                &state.course_id.0.to_string()
            );

            let _ = api_with_auth_empty::<EmptyError, _>(
                &path,
                endpoints::course::UpdateDraftData::METHOD,
                Some(req),
            )
            .await;
        }));
    }
}
