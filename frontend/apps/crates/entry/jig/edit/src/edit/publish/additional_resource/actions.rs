use std::rc::Rc;

use dominator::clone;
use shared::{api::{ApiEndpoint, endpoints}, error::EmptyError};
use utils::{prelude::api_with_auth_empty, unwrap::UnwrapJiExt};

use super::state::AdditionalResourceComponent;

impl AdditionalResourceComponent {
    pub fn delete(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        state.loader.load(clone!(state => async move {
            let jig_id = state.publish_state.jig.id.0.to_string();
            let resource_id = state.additional_resource.id.0.to_string();

            let path = endpoints::jig::additional_resource::Delete::PATH
                .replace("{id}", &jig_id)
                .replace("{additional_resource_id}", &resource_id);

            let res = api_with_auth_empty::<EmptyError, ()>(
                &path,
                endpoints::jig::additional_resource::Delete::METHOD,
                None
            ).await;

            match res {
                Err(_) => {
                    todo!();
                },
                Ok(_) => {
                    let mut additional_resources = state.publish_state.jig.additional_resources.lock_mut();
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
