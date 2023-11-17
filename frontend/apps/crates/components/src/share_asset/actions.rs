use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::Mutable;
use gloo_timers::callback::Timeout;
use shared::{
    api::endpoints::jig,
    domain::jig::{codes::JigPlayerSessionCreatePath, JigPlayerSettings},
};
use utils::prelude::*;

use super::state::ShareAsset;

const COPIED_TIMEOUT: u32 = 3_000;

impl ShareAsset {
    pub(super) fn generate_student_code(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = shared::domain::jig::codes::JigPlayerSessionCreateRequest {
                jig_id: state.asset.unwrap_jig().id,
                settings: JigPlayerSettings::default(),
                name: Default::default(),
            };

            match jig::codes::Create::api_with_auth(JigPlayerSessionCreatePath(), Some(req)).await {
                Err(_) => todo!(),
                Ok(res) => {
                    state.student_code.set(Some(res.index.to_string()));
                },
            };
        }));
    }

    pub fn set_copied_mutable(copied: Mutable<bool>) {
        copied.set(true);
        let timeout = Timeout::new(
            COPIED_TIMEOUT,
            clone!(copied => move || {
                copied.set(false);
            }),
        );
        timeout.forget();
    }
}
