use super::state::*;
use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{jig::*, meta, ApiEndpoint},
    domain::{jig::*, meta::MetadataResponse, CreateResponse},
    error::{EmptyError, MetadataNotFound},
};
use std::rc::Rc;
use utils::prelude::*;

impl JigGallery {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        state.loader.load(clone!(state => async move {
            join!(
                state.load_jigs(),
                state.load_ages(),
            );
        }));
    }

    async fn load_jigs(self: &Rc<Self>) {
        let state = Rc::clone(&self);
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

        match api_with_auth::<JigBrowseResponse, EmptyError, _>(&Browse::PATH, Browse::METHOD, req)
            .await
        {
            Ok(resp) => {
                state.jigs.lock_mut().replace_cloned(resp.jigs);
            }
            Err(_) => {}
        }
    }

    async fn load_ages(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        match api_with_auth::<MetadataResponse, EmptyError, ()>(
            meta::Get::PATH,
            meta::Get::METHOD,
            None,
        )
        .await
        {
            Err(_e) => {}
            Ok(res) => {
                state.age_ranges.set(res.age_ranges);
            }
        }
    }

    pub fn search_jigs(self: &Rc<Self>, q: String) {
        let state = Rc::clone(&self);
        state.loader.load(clone!(state => async move {
            let is_published = match *state.visible_jigs.lock_ref() {
                VisibleJigs::All => None,
                VisibleJigs::Published => Some(true),
                VisibleJigs::Draft => Some(false),
            };

            let req = Some(JigSearchQuery {
                q,
                is_published,
                ..Default::default()
            });

            match api_with_auth::<JigSearchResponse, EmptyError, _>(&Search::PATH, Search::METHOD, req).await {
                Ok(resp) => {
                    state.jigs.lock_mut().replace_cloned(resp.jigs);
                },
                Err(_) => {},
            }
        }));
    }

    pub fn load_jigs_regular(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        state.loader.load(clone!(state => async move {
            state.load_jigs().await
        }));
    }

    pub fn create_jig(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        state.loader.load(async {
            let req = Some(JigCreateRequest::default());

            match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(
                &Create::PATH,
                Create::METHOD,
                req,
            )
            .await
            {
                Ok(resp) => {
                    let url: String = Route::Jig(JigRoute::Edit(resp.id, JigEditRoute::Landing)).into();
                    dominator::routing::go_to_url(&url);
                }
                Err(_) => {}
            }
        });
    }

    pub fn copy_jig(self: &Rc<Self>, jig_id: &JigId) {
        let state = Rc::clone(&self);
        let path = Clone::PATH.replace("{id}", &jig_id.0.to_string());

        state.loader.load(clone!(state => async move {
            match api_with_auth::<CreateResponse<JigId>, EmptyError, ()>(&path, Clone::METHOD, None).await {
                Ok(resp) => {

                    let path = GetDraft::PATH.replace("{id}", &resp.id.0.to_string());
                    match api_with_auth::<JigResponse, EmptyError, ()>(&path, GetDraft::METHOD, None).await {
                        Ok(resp) => {
                            state.jigs.lock_mut().push_cloned(resp);
                        },
                        Err(_) => {},
                    };

                },
                Err(_) => {},
            };
        }));
    }

    pub fn delete_jig(self: &Rc<Self>, jig_id: JigId) {
        let state = Rc::clone(&self);
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
}
