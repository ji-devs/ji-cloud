use std::rc::Rc;

use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::{
        jig::{JigCreateRequest, JigId, JigFocus},
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
    state.loader.load(async {
        let req = Some(JigCreateRequest::default());

        match api_with_auth::<CreateResponse<JigId>, EmptyError, _>(
            &jig::Create::PATH,
            jig::Create::METHOD,
            req,
        )
        .await
        {
            Ok(resp) => {
                let url: String = Route::Jig(JigRoute::Edit(
                    resp.id,
                    JigFocus::Modules, // TODO: might change with resources
                    JigEditRoute::Landing
                )).into();
                dominator::routing::go_to_url(&url);
            }
            Err(_) => {}
        }
    });
}
