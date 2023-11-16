use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{asset::AssetType, jig::codes::JigCodeSessionsPath, module::ModuleGetLivePath},
};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::CodeSessions;

impl CodeSessions {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            join!(
                state.load_module(),
                state.load_report(),
            );
        }));
    }

    async fn load_module(self: &Rc<Self>) {
        let res = endpoints::module::GetLive::api_with_auth(
            ModuleGetLivePath(AssetType::Jig, self.module_id.clone()),
            None,
        )
        .await
        .toast_on_err();
        let res = bail_on_err!(res);
        self.module.set(Some(res));
    }

    async fn load_report(self: &Rc<Self>) {
        let res = endpoints::jig::codes::JigCodeSessions::api_with_auth(
            JigCodeSessionsPath(self.code),
            None,
        )
        .await;
        let res = bail_on_err!(res);
        self.infos.lock_mut().extend(res.sessions);
    }
}
