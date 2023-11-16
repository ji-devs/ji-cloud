use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::jig::codes::JigCodeListPath};
use utils::{bail_on_err, fetch::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::Codes;

impl Codes {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            state.load_codes().await;
        }));
    }

    async fn load_codes(self: &Rc<Self>) {
        let res = endpoints::jig::codes::JigCodeList::api_with_auth(JigCodeListPath(), None).await;
        let res = bail_on_err!(res);
        self.codes.lock_mut().extend(res.codes);
    }
}
