use std::rc::Rc;

use futures_signals::signal_vec::MutableVec;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        resource::{ListLikedPath, ListLikedRequest, ResourceId, ResourceUnlikePath},
    },
};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::likes::liked_section::LikedSection;

impl LikedSection {
    pub async fn load_resources(self: &Rc<Self>) {
        let req = ListLikedRequest {
            page: Some(self.next_page.get()),
            ..Default::default()
        };
        let res = endpoints::resource::ListLiked::api_with_auth(ListLikedPath(), Some(req))
            .await
            .toast_on_err();
        let res = bail_on_err!(res);

        let mut resources = self.list.lock_mut();
        res.resources.into_iter().for_each(|resource| {
            resources.push_cloned(Rc::new(resource.into()));
        });

        self.total.set(res.total_resource_count);

        let mut last_page_loaded = self.next_page.lock_mut();
        *last_page_loaded += 1;
    }

    pub(super) async fn unlike_resource(
        self: &Rc<Self>,
        resource_id: ResourceId,
    ) -> Rc<MutableVec<Rc<Asset>>> {
        endpoints::resource::Unlike::api_with_auth(ResourceUnlikePath(resource_id), None)
            .await
            .unwrap_ji();
        Rc::clone(&self.list)
    }
}
