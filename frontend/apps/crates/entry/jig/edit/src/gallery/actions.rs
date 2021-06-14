use dominator::clone;
use futures::join;
use shared::{api::endpoints::{ApiEndpoint, jig::*, meta}, domain::{CreateResponse, jig::*, meta::MetadataResponse}, error::{EmptyError, MetadataNotFound}};
use std::rc::Rc;
use super::state::*;
use utils::prelude::*;


pub fn load_data(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        join!(
            load_jigs(Rc::clone(&state)),
            load_ages(Rc::clone(&state)),
        );
    }));
}

async fn load_jigs(state: Rc<State>) {
    let is_published = match *state.visible_jigs.lock_ref() {
        VisibleJigs::All => None,
        VisibleJigs::Published => Some(true),
        VisibleJigs::Draft => Some(false),
    };

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
}

async fn load_ages(state: Rc<State>) {
    match api_with_auth::<MetadataResponse, EmptyError, ()>(meta::Get::PATH, meta::Get::METHOD, None).await {
        Err(e) => {},
        Ok(res) => {
            state.age_ranges.set(res.age_ranges);
        },
    }
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

pub fn load_jigs_regular(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        load_jigs(Rc::clone(&state)).await
    }));
}

pub fn create_jig(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        
        let req = Some(JigCreateRequest::default());

        match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(&Create::PATH, Create::METHOD, req).await {
            Ok(resp) => {
                let url:String = Route::Jig(JigRoute::Edit(resp.id, JigEditRoute::Landing)).into();
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
