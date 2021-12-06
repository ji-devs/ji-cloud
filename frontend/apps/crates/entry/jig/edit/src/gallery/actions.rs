use super::state::*;
use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{
        self, ApiEndpoint
    },
    domain::{
        jig::*,
        meta::MetadataResponse,
        CreateResponse
    },
    error::{
        EmptyError,
        MetadataNotFound
    }
};
use std::{default, rc::Rc};
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

        let req = JigBrowseQuery {
            is_published,
            author_id: Some(UserOrMe::Me),
            jig_focus: Some(state.focus),
            ..Default::default()
        };

        match api_with_auth::<JigBrowseResponse, EmptyError, _>(
            &endpoints::jig::Browse::PATH,
            endpoints::jig::Browse::METHOD,
            Some(req)
        )
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
            endpoints::meta::Get::PATH,
            endpoints::meta::Get::METHOD,
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

            match api_with_auth::<JigSearchResponse, EmptyError, _>(
                &endpoints::jig::Search::PATH,
                endpoints::jig::Search::METHOD,
                req
            ).await {
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
        state.loader.load(clone!(state => async move {
            let req = Some(JigCreateRequest::default());

            match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(
                &endpoints::jig::Create::PATH,
                endpoints::jig::Create::METHOD,
                req,
            )
            .await
            {
                Ok(resp) => {
                    if state.focus.is_resources() {
                        Self::set_focus_resource(&resp.id).await;
                    }
                    let url: String = Route::Jig(JigRoute::Edit(resp.id, JigEditRoute::Landing)).into();
                    dominator::routing::go_to_url(&url);
                }
                Err(_) => todo!("")
            }
        }));
    }

    async fn set_focus_resource(jig_id: &JigId) {
        let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id.0.to_string());
        let req = JigUpdateDraftDataRequest {
            jig_focus: Some(JigFocus::Resources),
            ..Default::default()
        };
        match api_with_auth_empty::<EmptyError, JigUpdateDraftDataRequest>(
            &path,
            endpoints::jig::UpdateDraftData::METHOD,
            Some(req)
        ).await {
            Ok(_) => {},
            Err(_) => todo!(),
        };
    }

    pub fn copy_jig(self: &Rc<Self>, jig_id: &JigId) {
        let state = Rc::clone(&self);
        let path = endpoints::jig::Clone::PATH.replace("{id}", &jig_id.0.to_string());

        state.loader.load(clone!(state => async move {
            match api_with_auth::<CreateResponse<JigId>, EmptyError, ()>(&path, endpoints::jig::Clone::METHOD, None).await {
                Ok(resp) => {

                    let path = endpoints::jig::GetDraft::PATH.replace("{id}", &resp.id.0.to_string());
                    match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::GetDraft::METHOD, None).await {
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
            let path = endpoints::jig::Delete::PATH.replace("{id}",&jig_id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::Delete::METHOD, None).await {
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
