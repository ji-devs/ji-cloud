use std::rc::Rc;

use dominator::clone;
use itertools::Itertools;
use shared::{
    api::endpoints,
    domain::{
        course::{CourseGetDraftPath, CourseUpdateDraftDataPath, CourseUpdateDraftDataRequest},
        jig::{JigGetLivePath, JigId, JigResponse, JigSearchPath, JigSearchQuery},
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use super::state::JigSelection;

impl JigSelection {
    pub fn load_course(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let res = endpoints::course::GetDraft::api_with_auth(
                CourseGetDraftPath(state.course_id),
                None,
            )
            .await;

            match res {
                Ok(course) => {
                    let mut items = Vec::with_capacity(course.course_data.items.len());
                    for jig_id in course.course_data.items {
                        let jig = state.get_jig(&jig_id).await;
                        items.push(Rc::new(jig));
                    }
                    state.jigs.lock_mut().replace_cloned(items);
                },
                Err(_) => todo!(),
            }
        }));
    }

    pub fn save_course(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let items = state
                .jigs
                .lock_ref()
                .iter()
                .map(|jig| jig.id)
                .collect_vec();
            let req = CourseUpdateDraftDataRequest {
                items: Some(items),
                ..Default::default()
            };

            let _ = endpoints::course::UpdateDraftData::api_with_auth_empty(
                CourseUpdateDraftDataPath(state.course_id.clone()),
                Some(req),
            )
            .await;
        }));
    }

    pub fn add_jig(self: &Rc<Self>, jig: Rc<JigResponse>) {
        self.jigs.lock_mut().push_cloned(jig);
        self.save_course();
    }

    pub fn remove_jig(self: &Rc<Self>, to_remove: &JigId) {
        self.jigs.lock_mut().retain(|jig| &jig.id != to_remove);
        self.save_course();
    }

    pub fn move_up_jig(self: &Rc<Self>, jig_id: &JigId) {
        let mut jigs = self.jigs.lock_mut();
        let pos = jigs.iter().position(|jig| &jig.id == jig_id).unwrap();
        jigs.move_from_to(pos, pos - 1);
        self.save_course();
    }

    pub fn move_down_jig(self: &Rc<Self>, jig_id: &JigId) {
        let mut jigs = self.jigs.lock_mut();
        let pos = jigs.iter().position(|jig| &jig.id == jig_id).unwrap();
        jigs.move_from_to(pos, pos + 1);
        self.save_course();
    }

    async fn get_jig(self: &Rc<Self>, jig_id: &JigId) -> JigResponse {
        endpoints::jig::GetLive::api_with_auth(JigGetLivePath(jig_id.clone()), None)
            .await
            .unwrap_ji()
    }

    pub fn search(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let req = JigSearchQuery {
                q: String::from(state.input.borrow().clone()),
                ..Default::default()
            };

            match endpoints::jig::Search::api_no_auth(JigSearchPath(), Some(req)).await {
                Err(_) => todo!(),
                Ok(res) => {
                    let jigs = res
                        .jigs
                        .into_iter()
                        .map(|jig| Rc::new(jig))
                        .collect_vec();
                    state.search_results.lock_mut().replace_cloned(jigs);
                }
            };

        }));
    }
}
