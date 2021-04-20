use shared::{
    api::endpoints::{ApiEndpoint, self, jig::module::*},
    error::{EmptyError, MetadataNotFound},
    domain::jig::{*, module::*},
};
use std::rc::Rc;
use std::cell::RefCell;
use utils::prelude::*;
use dominator::clone;

pub async fn load_module_kind(jig_id: JigId, module_id: ModuleId, module_kind: Rc<RefCell<Option<ModuleKind>>>) {
    //TODO - API to just get module kind, so no need to load entire body here
    let path = Get::PATH
        .replace("{id}",&jig_id.0.to_string())
        .replace("{module_id}",&module_id.0.to_string());

    match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
        Ok(resp) => {
            *module_kind.borrow_mut() = resp.module.body.map(|body| body.kind());
        },
        Err(_) => {},
    }
}

