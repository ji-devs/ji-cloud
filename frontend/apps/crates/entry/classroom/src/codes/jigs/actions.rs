use std::{collections::HashSet, rc::Rc};

use dominator::clone;
use futures::future::join_all;
use shared::{
    api::endpoints::{self, jig},
    domain::jig::{
        codes::{JigCodeListPath, JigCodeListRequest},
        JigGetDraftPath, JigId, JigResponse,
    },
    error::IntoAnyhow,
};
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
        let req = JigCodeListRequest { jig_id: None };
        let res =
            endpoints::jig::codes::JigCodeList::api_with_auth(JigCodeListPath(), Some(req)).await;
        let res = bail_on_err!(res);
        let mut seen = HashSet::new();
        let jigs = res
            .codes
            .into_iter()
            .filter_map(|code| match seen.contains(&code.jig_id) {
                true => None,
                false => {
                    seen.insert(code.jig_id);
                    Some(load_jig(code.jig_id))
                }
            })
            .collect::<Vec<_>>();
        let jigs = join_all(jigs)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>();
        let jigs = bail_on_err!(jigs);
        self.jigs.lock_mut().replace_cloned(jigs);
    }
}

async fn load_jig(jig_id: JigId) -> anyhow::Result<JigResponse> {
    jig::GetDraft::api_with_auth(JigGetDraftPath(jig_id), None)
        .await
        .into_anyhow()
}
