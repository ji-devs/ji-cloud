use shared::{
    api::endpoints::{ApiEndpoint, self},
    error::{EmptyError, MetadataNotFound},
    domain::{CreateResponse, jig::*},
};
use std::rc::Rc;
use std::cell::RefCell;
use dominator::clone;
use super::state::*;
use utils::prelude::*;

pub async fn load_jig(jig_id: JigId, jig_cell: Rc<RefCell<Option<Jig>>>) {

    let path = endpoints::jig::Get::PATH.replace("{id}",&jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::Get::METHOD, None).await {
        Ok(resp) => {
            *jig_cell.borrow_mut() = Some(resp.jig);
        },
        Err(_) => {},
    }

}

pub async fn update_jig(jig_id: &JigId, req: JigUpdateRequest) -> Result<(), EmptyError> {
    let path = endpoints::jig::Update::PATH
        .replace("{id}", &jig_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::Update::METHOD, Some(req)).await
}

pub fn update_display_name(state: Rc<State>, value: String) {
    state.loader.load(clone!(state => async move {
        let value = if value.is_empty() {
            None
        } else {
            Some(value)
        };

        state.name.set(value.clone());

        let req = JigUpdateRequest {
            display_name: value,
            ..JigUpdateRequest::default()
        };

        match update_jig(&state.jig.id, req).await {
            Ok(_) => {},
            Err(_) => {},
        }
    }));
}
