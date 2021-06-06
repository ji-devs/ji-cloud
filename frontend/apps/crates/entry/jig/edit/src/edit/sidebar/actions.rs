use shared::{api::endpoints::{ApiEndpoint, self}, domain::jig::{Jig, JigId, JigResponse, JigUpdateRequest, module::ModuleId}, error::EmptyError};
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
        state.name.set(value.clone());

        let req = JigUpdateRequest {
            display_name: Some(value),
            ..JigUpdateRequest::default()
        };

        match update_jig(&state.jig.id, req).await {
            Ok(_) => {},
            Err(_) => {},
        }
    }));
}

pub fn duplicate_module(state: Rc<State>, module_id: &ModuleId) {
    state.loader.load(clone!(state, module_id => async move {
        let module = super::module_cloner::clone_module(&state.jig.id, &module_id, &state.jig.id).await.unwrap_ji();
        state.modules.lock_mut().push_cloned(Rc::new(module));
    }));
}
