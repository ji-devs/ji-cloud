use std::rc::Rc;

use super::JigCodes;
use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::jig::{
        codes::{JigCode, JigCodeListPath, JigCodeListRequest, JigCodePath, JigCodeUpdateRequest},
        JigGetLivePath,
    },
};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

impl JigCodes {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            join!(
                state.load_codes(),
                state.load_jig(),
            );
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

    async fn load_jig(self: &Rc<Self>) {
        let jig = endpoints::jig::GetLive::api_with_auth(JigGetLivePath(self.jig_id.clone()), None)
            .await
            .toast_on_err();
        let jig = bail_on_err!(jig);
        self.jig.set(Some(jig));
    }

    pub fn save_name(self: &Rc<Self>, code: JigCode, new_name: String) {
        spawn_local(async move {
            let req = JigCodeUpdateRequest {
                name: Some(Some(new_name)),
                settings: None,
            };
            let _ = endpoints::jig::codes::Update::api_with_auth(JigCodePath(code), Some(req))
                .await
                .toast_on_err();
        });
    }
}
