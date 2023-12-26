use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::jig::codes::{JigCodeListPath, JigCodeListRequest},
};
use utils::{bail_on_err, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::JigCodes;

impl JigCodes {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            state.load_codes().await;
        }));
    }

    async fn load_codes(self: &Rc<Self>) {
        let req = JigCodeListRequest {
            jig_id: Some(self.jig_id),
        };
        let res =
            endpoints::jig::codes::JigCodeList::api_with_auth(JigCodeListPath(), Some(req)).await;
        let res = bail_on_err!(res);
        self.codes.lock_mut().extend(res.codes);
    }
}
