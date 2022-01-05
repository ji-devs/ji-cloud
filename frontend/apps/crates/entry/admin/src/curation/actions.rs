use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{domain::jig::{JigBrowseQuery, JigId, JigResponse, JigSearchQuery}, api::{endpoints, ApiEndpoint}, error::EmptyError};
use utils::{prelude::{ApiEndpointExt, api_with_auth}, routes::{AdminCurationRoute, Route, AdminRoute}};

use super::{Curation, FetchMode};

impl Curation {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            join!(
                state.load_jigs(),
                state.load_meta()
            );
        }));
    }

    async fn load_meta(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.ages.set(meta.age_ranges);
                self.goals.set(meta.goals);
                self.affiliations.set(meta.affiliations);
            }
        };
    }

    pub async fn load_jigs(self: &Rc<Self>) {
        let res = match &*self.fetch_mode.borrow() {
            FetchMode::Browse => {
                self.load_jigs_browse().await
            },
            FetchMode::Search(query) => {
                self.load_jigs_search(query.clone()).await
            },
        };

        self.jigs.lock_mut().replace_cloned(res.jigs);
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_jigs_browse(&self) -> JigListResponse {
        let req = JigBrowseQuery {
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::jig::Browse::api_with_auth(Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => {
                JigListResponse {
                    jigs: res.jigs,
                    total_pages: res.pages,
                }
            }
        }
    }

    async fn load_jigs_search(&self, query: String) -> JigListResponse {
        let req = JigSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::jig::Search::api_with_auth(Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => {
                JigListResponse {
                    jigs: res.jigs,
                    total_pages: res.pages,
                }
            }
        }
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_jigs().await;
        }));
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminCurationRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::Curation(route)).push_state();
    }

    pub async fn get_jig(self: Rc<Self>, jig_id: JigId) -> JigResponse {
        let jig = self
            .jigs
            .lock_ref()
            .iter()
            .find(|jig| jig.id == jig_id)
            .cloned();
        match jig {
            Some(jig) => {
                jig
            }
            None => {
                self.load_jig(&jig_id).await
            },
        }
    }

    async fn load_jig(self: &Rc<Self>, jig_id: &JigId) -> JigResponse {
        let path = endpoints::jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string());

        match api_with_auth::<JigResponse, EmptyError, ()>(
            &path,
            endpoints::jig::GetDraft::METHOD,
            None,
        )
        .await
        {
            Ok(jig) => {
                jig
            }
            Err(_) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct JigListResponse {
    jigs: Vec<JigResponse>,
    total_pages: u32,
}
