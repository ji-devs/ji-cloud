use std::rc::Rc;

use shared::{
    api::endpoints::jig,
    domain::jig::{JigCreatePath, JigCreateRequest},
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, JigEditRoute, Route},
};

use super::state::PostPublish;

pub fn create_jig(state: Rc<PostPublish>) {
    state.loader.load(async move {
        let req = JigCreateRequest::default();

        match jig::Create::api_with_auth(JigCreatePath(), Some(req)).await {
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
