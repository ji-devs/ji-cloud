use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::Mutable;
use gloo_timers::callback::Timeout;
use shared::{
    api::endpoints::jig,
    domain::jig::{codes::JigPlayerSessionCreatePath, JigPlayerSettings},
};
use utils::{bail_on_err, prelude::*};

use crate::qr_dialog::{QrDialog, QrDialogCallbacks};

use super::state::ShareAsset;

const COPIED_TIMEOUT: u32 = 3_000;

impl ShareAsset {
    pub(super) fn generate_student_code(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = shared::domain::jig::codes::JigPlayerSessionCreateRequest {
                jig_id: state.asset.unwrap_jig().id,
                name: state.code_name.get_cloned(),
                settings: JigPlayerSettings {
                    direction: state.direction.get(),
                    scoring: state.scoring.get(),
                    ..Default::default()
                },
            };

            let res = jig::codes::Create::api_with_auth(JigPlayerSessionCreatePath(), Some(req)).await.toast_on_err();
            let res = bail_on_err!(res);
            state.student_code.set(Some(res.index));
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

    pub fn show_qr_code(self: &Rc<Self>) {
        let state = self;
        let qr_dialog = QrDialog::new_jig_code(
            state.student_code.get().unwrap_ji(),
            state.asset.unwrap_jig().jig_data.display_name.clone(),
            state.code_name.get_cloned(),
            QrDialogCallbacks::new(clone!(state => move || {
                state.qr_dialog.set(None);
            })),
        );
        self.qr_dialog.set(Some(qr_dialog));
        self.active_popup.set(None);
    }
}
