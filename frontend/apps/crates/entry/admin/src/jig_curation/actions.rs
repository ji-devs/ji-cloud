use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        asset::{DraftOrLive, OrderBy},
        jig::{
            JigBrowsePath, JigBrowseQuery, JigGetDraftPath, JigId, JigResponse, JigSearchPath,
            JigSearchQuery,
        },
        meta::GetMetadataPath,
    },
};
use utils::{
    editable_asset::EditableJig,
    prelude::ApiEndpointExt,
    routes::{AdminJigCurationRoute, AdminRoute, Route},
    unwrap::UnwrapJiExt,
};

use super::{FetchMode, JigCuration};

impl JigCuration {
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
        match endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.ages.set(meta.age_ranges);
                self.affiliations.set(meta.affiliations);
            }
        };
    }

    pub async fn load_jigs(self: &Rc<Self>) {
        // clone right away to free the lock
        let fetch_mode = self.fetch_mode.borrow().clone();
        let res = match fetch_mode {
            FetchMode::Browse => self.load_jigs_browse().await,
            FetchMode::Search(query) => self.load_jigs_search(query.clone()).await,
        };

        self.jigs.lock_mut().replace_cloned(
            res.jigs
                .into_iter()
                .map(|jig| Rc::new(jig.into()))
                .collect(),
        );
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_jigs_browse(&self) -> JigListResponse {
        let req = JigBrowseQuery {
            page: Some(self.active_page.get()),
            draft_or_live: Some(DraftOrLive::Live),
            order_by: Some(self.order_by.get()),
            ..Default::default()
        };

        match endpoints::jig::Browse::api_with_auth(JigBrowsePath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => JigListResponse {
                jigs: res.jigs,
                total_pages: res.pages,
            },
        }
    }

    async fn load_jigs_search(&self, query: String) -> JigListResponse {
        let req = JigSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::jig::Search::api_with_auth(JigSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => JigListResponse {
                jigs: res.jigs,
                total_pages: res.pages,
            },
        }
    }

    pub fn set_order_by(self: &Rc<Self>, order_by: OrderBy) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.order_by.set(order_by);
            state.load_jigs().await;
        }));
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_jigs().await;
        }));
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminJigCurationRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::JigCuration(route)).push_state();
    }

    pub async fn get_jig(self: Rc<Self>, jig_id: JigId) -> Rc<EditableJig> {
        let jig = self
            .jigs
            .lock_ref()
            .iter()
            .find(|jig| jig.id == jig_id)
            .cloned();
        match jig {
            Some(jig) => jig,
            None => Rc::new(self.load_jig(&jig_id).await),
        }
    }

    async fn load_jig(self: &Rc<Self>, jig_id: &JigId) -> EditableJig {
        match endpoints::jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id.clone()), None).await {
            Ok(jig) => jig.into(),
            Err(_) => {
                todo!()
            }
        }
    }

    pub fn save_and_publish(self: &Rc<Self>, jig: &Rc<EditableJig>) {
        self.loader.load(clone!(jig => async move {
            let (a, b) = join!(
                jig.save_draft(),
                jig.save_admin_data(),
            );
            a.unwrap_ji();
            b.unwrap_ji();
            jig.publish().await.unwrap_ji();
        }))
    }

    pub fn save_admin_data(self: &Rc<Self>, jig: &Rc<EditableJig>) {
        self.loader.load(clone!(jig => async move {
            jig.save_admin_data().await.unwrap_ji();
        }))
    }
}

#[derive(Clone, Debug)]
struct JigListResponse {
    jigs: Vec<JigResponse>,
    total_pages: u32,
}
