use shared::{
    api::endpoints::{ApiEndpoint, jig::*,},
    error::{EmptyError, MetadataNotFound},
    domain::{CreateResponse, jig::*},
};
use std::rc::Rc;
use std::cell::RefCell;
use super::state::*;
use utils::prelude::*;

pub async fn load_jig(jig_id: JigId, jig_cell: Rc<RefCell<Option<Jig>>>) {

    let path = Get::PATH.replace("{id}",&jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
        Ok(resp) => {
            *jig_cell.borrow_mut() = Some(resp.jig);
        },
        Err(_) => {},
    }

}

