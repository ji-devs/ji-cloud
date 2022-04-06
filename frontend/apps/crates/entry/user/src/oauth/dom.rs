use dominator::{html, Dom};

use utils::routes::*;
use wasm_bindgen_futures::spawn_local;

use super::actions;
use shared::domain::session::*;
pub struct OauthPage {}

impl OauthPage {
    pub fn render(data: OauthData, redirect_kind: OAuthUrlKind) -> Dom {
        spawn_local(async move {
            actions::finalize(data, redirect_kind).await;
        });

        html!("div")
    }
}
