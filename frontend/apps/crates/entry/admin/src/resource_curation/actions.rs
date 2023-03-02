use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        asset::{DraftOrLive, OrderBy},
        meta::GetMetadataPath,
        resource::{
            ResourceBrowsePath, ResourceBrowseQuery, ResourceGetDraftPath, ResourceId,
            ResourceResponse, ResourceSearchPath, ResourceSearchQuery,
        },
    },
};
use utils::{
    editable_asset::EditableResource,
    prelude::ApiEndpointExt,
    routes::{AdminResourceCurationRoute, AdminRoute, Route},
    unwrap::UnwrapJiExt,
};

use super::{FetchMode, ResourceCuration};

impl ResourceCuration {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            join!(
                state.load_resources(),
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

    pub async fn load_resources(self: &Rc<Self>) {
        // clone right away to free the lock
        let fetch_mode = self.fetch_mode.borrow().clone();
        let res = match fetch_mode {
            FetchMode::Browse => self.load_resources_browse().await,
            FetchMode::Search(query) => self.load_resources_search(query.clone()).await,
        };

        self.resources.lock_mut().replace_cloned(
            res.resources
                .into_iter()
                .map(|resource| Rc::new(resource.into()))
                .collect(),
        );
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_resources_browse(&self) -> ResourceListResponse {
        let req = ResourceBrowseQuery {
            page: Some(self.active_page.get()),
            draft_or_live: Some(DraftOrLive::Live),
            order_by: Some(self.order_by.get()),
            ..Default::default()
        };

        match endpoints::resource::Browse::api_with_auth(ResourceBrowsePath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => ResourceListResponse {
                resources: res.resources,
                total_pages: res.pages,
            },
        }
    }

    async fn load_resources_search(&self, query: String) -> ResourceListResponse {
        let req = ResourceSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::resource::Search::api_with_auth(ResourceSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => ResourceListResponse {
                resources: res.resources,
                total_pages: res.pages,
            },
        }
    }

    pub fn set_order_by(self: &Rc<Self>, order_by: OrderBy) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.order_by.set(order_by);
            state.load_resources().await;
        }));
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_resources().await;
        }));
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminResourceCurationRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::ResourceCuration(route)).push_state();
    }

    pub async fn get_resource(self: Rc<Self>, resource_id: ResourceId) -> Rc<EditableResource> {
        let resource = self
            .resources
            .lock_ref()
            .iter()
            .find(|resource| resource.id == resource_id)
            .cloned();
        match resource {
            Some(resource) => resource,
            None => Rc::new(self.load_resource(&resource_id).await),
        }
    }

    async fn load_resource(self: &Rc<Self>, resource_id: &ResourceId) -> EditableResource {
        match endpoints::resource::GetDraft::api_with_auth(
            ResourceGetDraftPath(resource_id.clone()),
            None,
        )
        .await
        {
            Ok(resource) => resource.into(),
            Err(_) => {
                todo!()
            }
        }
    }

    pub fn save_and_publish(self: &Rc<Self>, resource: &Rc<EditableResource>) {
        self.loader.load(clone!(resource => async move {
            let (a, b) = join!(
                resource.save_draft(),
                resource.save_admin_data(),
            );
            a.unwrap_ji();
            b.unwrap_ji();
            resource.publish().await.unwrap_ji();
        }))
    }
}

#[derive(Clone, Debug)]
struct ResourceListResponse {
    resources: Vec<ResourceResponse>,
    total_pages: u32,
}
