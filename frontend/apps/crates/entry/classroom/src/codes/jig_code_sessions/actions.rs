use std::rc::Rc;

use dominator::clone;
use futures::{future::try_join_all, join};
use shared::{
    api::endpoints,
    domain::{
        asset::AssetType,
        jig::{codes::JigCodeSessionsPath, JigGetLivePath},
        module::{ModuleGetLivePath, ModuleResponse},
    },
};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::{CodeSessions, JigWithModules};

impl CodeSessions {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            join!(
                state.load_jig(),
                state.load_report(),
            );
        }));
    }

    async fn load_jig(self: &Rc<Self>) {
        let jig = endpoints::jig::GetLive::api_with_auth(JigGetLivePath(self.jig_id.clone()), None)
            .await
            .toast_on_err();
        let jig = bail_on_err!(jig);
        let modules = try_join_all(jig.jig_data.modules.iter().map(|module| {
            endpoints::module::GetLive::api_with_auth(
                ModuleGetLivePath(AssetType::Jig, module.id.clone()),
                None,
            )
        }))
        .await
        .toast_on_err();
        let modules: Vec<ModuleResponse> = bail_on_err!(modules);
        let modules = modules
            .into_iter()
            .map(|module| (module.module.stable_id, module))
            .collect();

        self.jig.set(Some(JigWithModules { jig, modules }));
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
