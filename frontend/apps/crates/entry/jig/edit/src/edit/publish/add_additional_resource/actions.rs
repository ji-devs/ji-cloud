use std::rc::Rc;

use shared::api::ApiEndpoint;
use shared::domain::jig::additional_resource::{AdditionalResourceId, ResourceContent};
use shared::domain::jig::AdditionalResource;
use shared::domain::meta::ResourceTypeId;
use shared::domain::CreateResponse;
use shared::error::EmptyError;
use shared::{api::endpoints, domain::jig::additional_resource::AdditionalResourceCreateRequest};

use utils::prelude::api_with_auth;

use super::state::AddAdditionalResource;

impl AddAdditionalResource {
    pub(super) async fn save_additional_resource(
        self: &Rc<Self>,
        resource_content: ResourceContent,
        display_name: String,
        resource_type_id: ResourceTypeId,
    ) {
        let state = Rc::clone(&self);

        let req = AdditionalResourceCreateRequest {
            display_name: display_name.clone(),
            resource_type_id: resource_type_id.clone(),
            resource_content: resource_content.clone(),
        };

        let path = endpoints::jig::additional_resource::Create::PATH
            .replace("{id}", &self.publish_state.jig.id.0.to_string());
        let res = api_with_auth::<CreateResponse<AdditionalResourceId>, EmptyError, _>(
            &path,
            endpoints::jig::additional_resource::Create::METHOD,
            Some(req),
        )
        .await;

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
                    .jig
                    .additional_resources
                    .lock_mut()
                    .push_cloned(resource);
            }
            Err(_e) => todo!(),
        };
    }
}
