use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::user::public_user::PublicUserGetPath};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt};
use wasm_bindgen_futures::spawn_local;

use super::AssetSearchBar;

impl AssetSearchBar {
    pub(super) fn load_data(self: &Rc<Self>) {
        let state = self;
        if let Some(user_id) = state.search_selected.user_id.get() {
            spawn_local(clone!(state => async move {
                let user = endpoints::user::GetPublicUser::api_with_auth(PublicUserGetPath(user_id), None).await.toast_on_err();
                let user = bail_on_err!(user);
                state.selected_user.set(Some(user));
            }));
        };
    }
}
