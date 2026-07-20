use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::Mutable;
use gloo_timers::callback::Timeout;
use shared::{
    api::endpoints::jig,
    domain::jig::{codes::JigPlayerSessionCreatePath, JigPlayerSettings, JigShareUrlPath},
};
use utils::{bail_on_err, clipboard, prelude::*};

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

    pub(super) fn refresh_share_url(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.share_url_loading.set(true);
            if let Some((share_url, student_share_url)) = state.signed_share_urls().await {
                state.share_url.set(share_url);
                state.student_share_url.set(student_share_url);
            }
            state.share_url_loading.set(false);
        }));
    }

    pub(super) async fn signed_share_urls(&self) -> Option<(String, String)> {
        match &self.asset {
            shared::domain::asset::Asset::Jig(jig) => {
                let req = shared::domain::jig::JigShareUrlRequest {
                    direction: Some(self.direction.get()),
                    scoring: Some(self.scoring.get()),
                };

                let res = jig::ShareUrl::api_no_auth(JigShareUrlPath(jig.id), Some(req))
                    .await
                    .toast_on_err();
                res.ok().map(|res| (res.share_url, res.student_share_url))
            }
            _ => Some((
                self.share_url.get_cloned(),
                self.student_share_url.get_cloned(),
            )),
        }
    }

    pub(super) fn copy_share_url(&self, copied: Mutable<bool>) {
        clipboard::write_text(&self.share_url.get_cloned());
        ShareAsset::set_copied_mutable(copied);
    }

    pub(super) fn copy_student_share_url(&self, copied: Mutable<bool>) {
        clipboard::write_text(&self.student_share_url.get_cloned());
        ShareAsset::set_copied_mutable(copied);
    }

    pub(super) fn copy_embed_code(self: &Rc<Self>) {
        clipboard::write_text(&self.embed_code());
        ShareAsset::set_copied_mutable(self.copied_embed.clone());
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
