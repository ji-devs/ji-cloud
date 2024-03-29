use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::jig::codes::JigsWithCodesPath};
use utils::{bail_on_err, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::Jigs;

impl Jigs {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            state.load_codes().await;
        }));
    }

    async fn load_codes(self: &Rc<Self>) {
        let res =
            endpoints::jig::codes::JigsWithCodes::api_with_auth(JigsWithCodesPath(), None).await;
        let res = bail_on_err!(res);
        self.jigs.set(Some(res.jigs));
    }
}
