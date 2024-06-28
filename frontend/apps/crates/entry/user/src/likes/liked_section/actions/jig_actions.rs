use std::rc::Rc;

use futures_signals::signal_vec::MutableVec;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        jig::{JigId, JigUnlikePath, ListLikedPath, ListLikedRequest},
    },
};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::likes::liked_section::LikedSection;

impl LikedSection {
    pub async fn load_jigs(self: &Rc<Self>) {
        let req = ListLikedRequest {
            page: Some(self.next_page.get()),
            ..Default::default()
        };
        let res = endpoints::jig::ListLiked::api_with_auth(ListLikedPath(), Some(req))
            .await
            .toast_on_err();
        let res = bail_on_err!(res);

        let mut jigs = self.list.lock_mut();
        res.jigs.into_iter().for_each(|jig| {
            jigs.push_cloned(Rc::new(jig.into()));
        });

        self.total.set(res.total_jig_count);

        let mut last_page_loaded = self.next_page.lock_mut();
        *last_page_loaded += 1;
    }

    pub(super) async fn unlike_jig(self: &Rc<Self>, jig_id: JigId) -> Rc<MutableVec<Rc<Asset>>> {
        endpoints::jig::Unlike::api_with_auth(JigUnlikePath(jig_id), None)
            .await
            .unwrap_ji();
        Rc::clone(&self.list)
    }
}
