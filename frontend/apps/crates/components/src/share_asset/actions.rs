use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::Mutable;
use gloo_timers::callback::Timeout;
use shared::{api::endpoints::jig, domain::jig::JigPlayerSettings};
use utils::prelude::*;

use super::state::ShareAsset;

const COPIED_TIMEOUT: u32 = 3_000;

impl ShareAsset {
    pub(super) fn generate_student_code(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = shared::domain::jig::player::JigPlayerSessionCreateRequest {
                jig_id: *state.asset_id.unwrap_jig(),
                settings: JigPlayerSettings::default(),
            };

            match jig::player::Create::api_with_auth(Some(req)).await {
                Err(_) => todo!(),
                Ok(res) => {
                    let code = format!("{:04}", res.index.0);
                    state.student_code.set(Some(code));
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
