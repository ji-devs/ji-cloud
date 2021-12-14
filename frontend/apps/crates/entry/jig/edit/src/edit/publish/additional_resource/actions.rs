use std::rc::Rc;

use dominator::clone;
use shared::{api::{ApiEndpoint, endpoints}, domain::jig::additional_resource::AdditionalResource, error::EmptyError};
use utils::{prelude::{api_with_auth, api_with_auth_empty}, unwrap::UnwrapJiExt};

use super::state::AdditionalResourceComponent;

impl AdditionalResourceComponent {
    pub fn load_resource(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        state.loader.load(clone!(state => async move {
            let _jig_id = state.publish_state.jig.id.0.to_string();
            let _resource_id = state.id.0.to_string();

            let path = endpoints::jig::additional_resource::GetDraft::PATH
                .replace("{id}", &state.publish_state.jig.id.0.to_string())
                .replace("{additional_resource_id}", &state.id.0.to_string());

            let res = api_with_auth::<AdditionalResource, EmptyError, ()>(
                &path,
                endpoints::jig::additional_resource::GetDraft::METHOD,
                None
            ).await;

            match res {
                Err(_) => {
                    todo!();
                },
                Ok(additional_resource) => {
                    state.additional_resource.set(Some(additional_resource));
                },
            }
        }));
    }

    pub fn delete(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        state.loader.load(clone!(state => async move {
            let _jig_id = state.publish_state.jig.id.0.to_string();
            let _resource_id = state.id.0.to_string();

            let path = endpoints::jig::additional_resource::Delete::PATH
                .replace("{id}", &state.publish_state.jig.id.0.to_string())
                .replace("{additional_resource_id}", &state.id.0.to_string());

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
                    let current_additional_resource_id = state.additional_resource.get_cloned().unwrap_ji().id;
                    let index = additional_resources.iter().position(move |id| {
                        id == &current_additional_resource_id
                    }).unwrap_ji();
                    additional_resources.remove(index);
                },
            }
        }));
    }
}
