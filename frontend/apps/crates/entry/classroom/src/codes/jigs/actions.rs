use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::jig::JigBrowsePath};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::Jigs;

impl Jigs {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            state.load_jigs().await;
        }));
    }

    async fn load_jigs(self: &Rc<Self>) {
        let res = endpoints::jig::Browse::api_with_auth(JigBrowsePath(), None)
            .await
            .toast_on_err();
        let res = bail_on_err!(res);
        self.jigs.lock_mut().replace_cloned(res.jigs);
    }
}