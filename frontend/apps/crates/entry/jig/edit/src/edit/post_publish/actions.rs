use std::rc::Rc;

use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::{
        jig::{JigCreateRequest, JigId},
        CreateResponse,
    },
    error::EmptyError,
};
use utils::{
    prelude::api_with_auth,
    routes::{JigEditRoute, JigRoute, Route},
};

use super::state::State;

pub fn create_jig(state: Rc<State>) {
    let jig_focus = state.jig_edit_state.jig_focus;

    state.loader.load(async move {
        let req = JigCreateRequest {
            jig_focus,
            ..Default::default()
        };

        match api_with_auth::<CreateResponse<JigId>, EmptyError, _>(
            jig::Create::PATH,
            jig::Create::METHOD,
            Some(req),
        )
        .await
        {
            Ok(resp) => {
                let url: String =
                    Route::Jig(JigRoute::Edit(resp.id, jig_focus, JigEditRoute::Landing)).into();
                dominator::routing::go_to_url(&url);
            }
            Err(_) => {
                todo!();
            }
        }
    });
}
