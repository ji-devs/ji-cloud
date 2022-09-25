use std::rc::Rc;

use super::state::AdditionalResourceComponent;
use dominator::clone;
use shared::{
    api::endpoints,
    domain::additional_resource::{AssetIdResource, DeleteAssetResourcePath},
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

impl AdditionalResourceComponent {
    pub fn delete(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = AssetIdResource {
                asset_id: Some(state.publish_state.asset.id()),
            };
            let res = endpoints::additional_resource::Delete::api_with_auth_empty(
                DeleteAssetResourcePath(state.additional_resource.id.clone()),
                Some(req)
            ).await;

            match res {
                Err(_) => {
                    todo!();
                },
                Ok(_) => {
                    let mut additional_resources = state.publish_state.asset.additional_resources().lock_mut();
                    let current_additional_resource_id = state.additional_resource.id;
                    let index = additional_resources.iter().position(move |additional_resource| {
                        additional_resource.id == current_additional_resource_id
                    }).unwrap_ji();
                    additional_resources.remove(index);
                },
            }
        }));
    }
}
