use shared::{
    api::endpoints::{jig::module::*, ApiEndpoint},
    domain::jig::{module::*, JigId},
    error::EmptyError,
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_module_kind(
    jig_id: JigId,
    module_id: ModuleId,
    module_kind: Rc<RefCell<Option<ModuleKind>>>,
) {
    //TODO - API to just get module kind, so no need to load entire body here
    let path = Get::PATH
        .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());

    match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
        Ok(resp) => {
            *module_kind.borrow_mut() = Some(resp.module.body.kind());
        }
        Err(_) => {}
    }
}
