use dominator::clone;
use shared::{
    api::endpoints::{ApiEndpoint, jig::*,},
    error::{EmptyError, MetadataNotFound},
    domain::{CreateResponse, jig::*},
};
use std::rc::Rc;
use super::state::*;
use utils::prelude::*;

pub fn load_jigs(state: Rc<State>) {
    let is_published = match *state.visible_jigs.lock_ref() {
        VisibleJigs::All => None,
        VisibleJigs::Published => Some(true),
        VisibleJigs::Draft => Some(false),
    };
    state.loader.load(clone!(state => async move {
        let req = Some(JigBrowseQuery {
            is_published,
            author_id: Some(UserOrMe::Me),
            page: None,
        });

        match api_with_auth::<JigBrowseResponse, EmptyError, _>(&Browse::PATH, Browse::METHOD, req).await {
            Ok(resp) => {
                state.jigs.lock_mut().replace_cloned(resp.jigs);
            },
            Err(_) => {},
        }
    }));
}

pub fn search_jigs(state: Rc<State>, q: String) {
    state.loader.load(clone!(state => async move {
        let req = Some(JigSearchQuery {
            q,
            ..Default::default()
        });

        match api_with_auth::<JigSearchResponse, EmptyError, _>(&Search::PATH, Search::METHOD, req).await {
            Ok(resp) => {
                state.jigs.lock_mut().replace_cloned(
                    resp.jigs
                        .into_iter()
                        .map(|jr| jr.jig)
                        .collect()
                    );
            },
            Err(_) => {},
        }
    }));
}

pub fn create_jig(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        
        let req = Some(JigCreateRequest::default());

        match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(&Create::PATH, Create::METHOD, req).await {
            Ok(resp) => {
                let url:String = Route::Jig(JigRoute::Edit(resp.id, None)).into();
                dominator::routing::go_to_url(&url);
            },
            Err(_) => {},
        }
    }));

}

pub fn copy_jig(state: Rc<State>, jig_id: &JigId) {
    let path = Clone::PATH.replace("{id}", &jig_id.0.to_string());

    state.loader.load(clone!(state => async move {
        match api_with_auth::<CreateResponse<JigId>, EmptyError, ()>(&path, Clone::METHOD, None).await {
            Ok(resp) => {

                let path = Get::PATH.replace("{id}", &resp.id.0.to_string());
                match api_with_auth::<JigResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
                    Ok(resp) => {
                        state.jigs.lock_mut().push_cloned(resp.jig);
                    },
                    Err(_) => {},
                };

            },
            Err(_) => {},
        };
    }));
}


pub fn delete_jig(state: Rc<State>, jig_id: JigId) {
    state.loader.load(clone!(state => async move {
        let path = Delete::PATH.replace("{id}",&jig_id.0.to_string());
        match api_with_auth_empty::<EmptyError, ()>(&path, Delete::METHOD, None).await {
            Ok(_) => {
                state.jigs.lock_mut().retain(|jig| {
                    jig.id != jig_id
                });
            },
            Err(_) => {}
        }
    }));

}
