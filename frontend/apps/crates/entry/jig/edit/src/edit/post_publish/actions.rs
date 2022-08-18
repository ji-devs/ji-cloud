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
    routes::{AssetEditRoute, AssetRoute, JigEditRoute, Route},
};

use super::state::PostPublish;

pub fn create_jig(state: Rc<PostPublish>) {
    state.loader.load(async move {
        let req = JigCreateRequest::default();

        match api_with_auth::<CreateResponse<JigId>, EmptyError, _>(
            jig::Create::PATH,
            jig::Create::METHOD,
            Some(req),
        )
        .await
        {
            Ok(resp) => {
                let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                    resp.id,
                    JigEditRoute::Landing,
                )))
                .into();
                dominator::routing::go_to_url(&url);
            }
            Err(_) => {
                todo!();
            }
        }
    });
}
