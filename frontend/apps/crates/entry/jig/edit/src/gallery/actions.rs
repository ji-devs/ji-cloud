use dominator::clone;
use shared::{
    api::endpoints::{ApiEndpoint, jig::*,},
    error::{EmptyError, MetadataNotFound},
    domain::{CreateResponse, jig::*},
};
use std::rc::Rc;
use std::cell::RefCell;
use super::state::*;
use utils::prelude::*;

pub fn load_jigs(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let req = Some(JigBrowseQuery {
            is_published: None,
            author_id: Some(UserOrMe::Me),
            page: None,
        });

        match api_with_auth::<JigBrowseResponse, EmptyError, _>(&Browse::PATH, Browse::METHOD, req).await {
            Ok(resp) => {
                state.jigs.lock_mut().replace_cloned(
                    resp.jigs
                        .into_iter()
                        .map(|jig| {
                            (jig.id, jig.display_name)
                        })
                        .collect()
                );
            },
            Err(_) => {},
        }
    }));

}

pub fn create_jig(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        
        let req = Some(JigCreateRequest {
            display_name: None,
            modules: Vec::new(),
            content_types: Vec::new(),
            publish_at: None,
        });

        match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(&Create::PATH, Create::METHOD, req).await {
            Ok(resp) => {
                state.jigs.lock_mut().push_cloned((resp.id, None));
            },
            Err(_) => {},
        }
    }));

}


pub fn delete_jig(state: Rc<State>, jig_id: JigId) {
    state.loader.load(clone!(state => async move {
        let path = Delete::PATH.replace("{id}",&jig_id.0.to_string());
        match api_with_auth_empty::<EmptyError, ()>(&path, Delete::METHOD, None).await {
            Ok(_) => {
                state.jigs.lock_mut().retain(|(id, _)| {
                    *id != jig_id
                });
            },
            Err(_) => {}
        }
    }));

}
