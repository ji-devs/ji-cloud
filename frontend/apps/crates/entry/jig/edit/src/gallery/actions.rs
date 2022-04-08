use super::state::*;
use components::module::_common::prelude::ModuleId;
use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        jig::{
            module::{ModuleBody, ModuleCreateRequest},
            DraftOrLive, JigBrowseQuery, JigBrowseResponse, JigCreateRequest, JigId, JigResponse,
            JigSearchQuery, JigSearchResponse, ModuleKind, UserOrMe,
        },
        meta::MetadataResponse,
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};
use std::rc::Rc;
use utils::prelude::*;

impl JigGallery {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_jigs(),
                state.load_ages(),
            );
        }));
    }

    async fn load_jigs(self: &Rc<Self>) {
        let state = self;
        let is_published = match *state.visible_jigs.lock_ref() {
            VisibleJigs::All => None,
            VisibleJigs::Published => Some(true),
            VisibleJigs::Draft => Some(false),
        };

        let req = JigBrowseQuery {
            page: Some(*self.next_page.lock_ref()),
            is_published,
            author_id: Some(UserOrMe::Me),
            jig_focus: Some(state.focus),
            draft_or_live: Some(DraftOrLive::Draft),
            ..Default::default()
        };

        match api_with_auth::<JigBrowseResponse, EmptyError, _>(
            endpoints::jig::Browse::PATH,
            endpoints::jig::Browse::METHOD,
            Some(req),
        )
        .await
        {
            Ok(mut resp) => {
                // Update the total count and increment the next page so that a future call will
                // call the correct page.
                state.total_jig_count.set(Some(resp.total_jig_count));
                *state.next_page.lock_mut() += 1;

                // Append results to the current list.
                let mut new_list = state.jigs.lock_ref().to_vec();
                new_list.append(&mut resp.jigs);

                // Update the list with the new list.
                state.jigs.lock_mut().replace_cloned(new_list);
            }
            Err(_) => {
                todo!();
            }
        }
    }

    async fn load_ages(self: &Rc<Self>) {
        let state = Rc::clone(self);
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
        let state = self;
        state.loader.load(clone!(state => async move {
            let is_published = match *state.visible_jigs.lock_ref() {
                VisibleJigs::All => None,
                VisibleJigs::Published => Some(true),
                VisibleJigs::Draft => Some(false),
            };

            let req = Some(JigSearchQuery {
                q,
                is_published,
                author_id: Some(UserOrMe::Me),
                jig_focus: Some(state.focus),
                ..Default::default()
            });

            match api_with_auth::<JigSearchResponse, EmptyError, _>(
                endpoints::jig::Search::PATH,
                endpoints::jig::Search::METHOD,
                req
            ).await {
                Ok(resp) => {
                    state.jigs.lock_mut().replace_cloned(resp.jigs);
                },
                Err(_) => {
                    todo!();
                },
            }
        }));
    }

    pub fn load_jigs_regular(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.load_jigs().await
        }));
    }

    pub fn create_jig(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = JigCreateRequest {
                jig_focus: state.focus,
                ..Default::default()
            };

            match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(
                endpoints::jig::Create::PATH,
                endpoints::jig::Create::METHOD,
                Some(req),
            )
            .await
            {
                Ok(resp) => {
                    if state.focus.is_resources() {
                        Self::add_resource_cover(&resp.id).await;
                    }
                    let url: String = Route::Jig(JigRoute::Edit(
                        resp.id,
                        state.focus,
                        JigEditRoute::Landing
                    )).into();
                    dominator::routing::go_to_url(&url);
                }
                Err(_) => todo!("")
            }
        }));
    }

    async fn add_resource_cover(jig_id: &JigId) {
        let req = ModuleCreateRequest {
            body: ModuleBody::new(ModuleKind::ResourceCover),
        };

        let path = endpoints::jig::module::Create::PATH.replace("{id}", &jig_id.0.to_string());

        match api_with_auth::<CreateResponse<ModuleId>, EmptyError, _>(
            &path,
            endpoints::jig::module::Create::METHOD,
            Some(req),
        )
        .await
        {
            Ok(_) => {}
            Err(_) => {
                todo!()
            }
        }
    }

    pub fn copy_jig(self: &Rc<Self>, jig_id: &JigId) {
        let state = self;
        let path = endpoints::jig::Clone::PATH.replace("{id}", &jig_id.0.to_string());

        state.loader.load(clone!(state => async move {
            match api_with_auth::<CreateResponse<JigId>, EmptyError, ()>(&path, endpoints::jig::Clone::METHOD, None).await {
                Ok(resp) => {

                    let path = endpoints::jig::GetDraft::PATH.replace("{id}", &resp.id.0.to_string());
                    match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::GetDraft::METHOD, None).await {
                        Ok(resp) => {
                            state.jigs.lock_mut().push_cloned(resp);
                        },
                        Err(_) => {
                            todo!();
                        },
                    };

                },
                Err(_) => {
                    todo!();
                },
            };
        }));
    }

    pub fn delete_jig(self: &Rc<Self>, jig_id: JigId) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let path = endpoints::jig::Delete::PATH.replace("{id}",&jig_id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::Delete::METHOD, None).await {
                Ok(_) => {
                    state.jigs.lock_mut().retain(|jig| {
                        jig.id != jig_id
                    });
                },
                Err(_) => {
                    todo!();
                }
            }
        }));
    }
}
