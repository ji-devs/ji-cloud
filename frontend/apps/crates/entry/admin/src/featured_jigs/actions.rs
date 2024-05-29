use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::jig::{JigFeaturedPath, JigFeaturedUpdateRequest, JigSearchPath},
};
use utils::{bail_on_err, error_ext::ErrorExt, fetch::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::FeaturedJigs;

impl FeaturedJigs {
    pub fn search(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let mut req = state.search_bar.search_selected.to_jig_search_request();

            state.active_query.set(req.q.clone());

            req.page = Some(state.next_page.get());

            let res = endpoints::jig::Search::api_no_auth(JigSearchPath(), Some(req)).await.toast_on_err();
            let res = bail_on_err!(res);

            let mut jigs = state.search_results.lock_mut();
            res.jigs.into_iter().for_each(|jig| {
                jigs.push_cloned(Rc::new(jig.into()));
            });

            state.total_jig_count.set(res.total_jig_count as u32);

            state.next_page.replace_with(|next_page| *next_page + 1);
        }));
    }

    pub fn load_featured(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let res = endpoints::jig::Featured::api_with_auth(JigFeaturedPath(), None).await.toast_on_err();
            let res = bail_on_err!(res);
            state.featured_jigs.lock_mut().extend(res.jigs);
        }));
    }

    pub fn update_featured(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let req = JigFeaturedUpdateRequest {
                jigs: state.featured_jigs.lock_ref().iter().map(|jig| jig.id).collect(),
            };
            let _ = endpoints::jig::FeaturedUpdate::api_with_auth(JigFeaturedPath(), Some(req)).await.toast_on_err();
        }));
    }
}
