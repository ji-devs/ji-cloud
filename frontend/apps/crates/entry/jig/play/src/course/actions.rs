use std::rc::Rc;

use dominator::clone;
use futures::future::try_join_all;
use shared::{api::{endpoints, ApiEndpoint}, domain::{course::CourseResponse, jig::{JigId, JigResponse}}, error::EmptyError};
use utils::{prelude::api_no_auth, unwrap::UnwrapJiExt};

use super::state::CoursePlayer;

impl CoursePlayer {
    pub fn load_course(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            // let course = match state.player_options.draft {
            //     false => {
            //         let path = endpoints::course::GetLive::PATH.replace("{id}", &state.course_id.0.to_string());
            //         api_no_auth::<CourseResponse, EmptyError, ()>(&path, endpoints::course::GetLive::METHOD, None).await
            //     },
            //     true => {
            //         let path = endpoints::course::GetDraft::PATH.replace("{id}", &state.course_id.0.to_string());
            //         api_no_auth::<CourseResponse, EmptyError, ()>(&path, endpoints::course::GetDraft::METHOD, None).await
            //     },
            // };
            let path = endpoints::course::GetDraft::PATH.replace("{id}", &state.course_id.0.to_string());
            let course = api_no_auth::<CourseResponse, EmptyError, ()>(&path, endpoints::course::GetDraft::METHOD, None).await;

            match course {
                Ok(course) => {
                    let jig_ids = course.course_data.items.clone();
                    state.course.set(Some(course));
                    state.load_jigs(jig_ids).await;
                },
                Err(_) => {
                    todo!();
                },
            }
        }));
    }


    async fn load_jigs(self: &Rc<Self>, jig_ids: Vec<JigId>) {
        let jigs = try_join_all(
            jig_ids.iter().map(|jig_id| self.load_jig(jig_id))
        )
            .await
            .unwrap_ji();

        self.jigs.set(jigs);
    }

    async fn load_jig(self: &Rc<Self>, jig_id: &JigId) -> Result<JigResponse, ()> {
        let path = endpoints::jig::GetLive::PATH.replace("{id}", &jig_id.0.to_string());
        api_no_auth::<JigResponse, EmptyError, ()>(
            &path,
            endpoints::jig::GetLive::METHOD,
            None
        )
            .await
            .map_err(|_| ())
    }
}
