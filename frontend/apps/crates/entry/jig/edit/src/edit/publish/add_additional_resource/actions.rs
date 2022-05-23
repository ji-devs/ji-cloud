use std::rc::Rc;

use shared::domain::additional_resource::AdditionalResource;
use shared::domain::additional_resource::ResourceContent;
use shared::domain::meta::ResourceTypeId;
use shared::{api::endpoints, domain::additional_resource::AdditionalResourceCreateRequest};

use utils::prelude::ApiEndpointExt;

use super::state::AddAdditionalResource;

impl AddAdditionalResource {
    pub(super) async fn save_additional_resource(
        self: &Rc<Self>,
        resource_content: ResourceContent,
        display_name: String,
        resource_type_id: ResourceTypeId,
    ) {
        let state = Rc::clone(self);

        let req = AdditionalResourceCreateRequest {
            asset_id: state.publish_state.asset.id(),
            display_name: display_name.clone(),
            resource_type_id,
            resource_content: resource_content.clone(),
        };

        let res = endpoints::additional_resource::Create::api_with_auth(Some(req)).await;

        match res {
            Ok(res) => {
                let resource = AdditionalResource {
                    id: res.id,
                    display_name,
                    resource_type_id,
                    resource_content,
                };
                state
                    .publish_state
                    .asset
                    .additional_resources()
                    .lock_mut()
                    .push_cloned(resource);
            }
            Err(_e) => todo!(),
        };
    }
}
