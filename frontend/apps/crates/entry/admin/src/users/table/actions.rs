use std::rc::Rc;

use dominator::clone;
use shared::domain::admin::SetAccountTierOverridePath;
use shared::domain::billing::PlanTier;
use shared::domain::UpdateNullable;
use shared::{
    api::endpoints,
    domain::user::{PatchProfileAdminDataPath, PatchProfileAdminDataRequest},
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::users::{EditableUser, FetchMode};

use super::state::UsersTable;

impl UsersTable {
    pub fn search_users(self: &Rc<Self>, query: String) {
        let state = self;
        let mut fetch_mode = state.users_state.fetch_mode.lock_mut();
        if query.is_empty() {
            *fetch_mode = FetchMode::Browse;
        } else {
            *fetch_mode = FetchMode::Search(query);
        }

        state.users_state.active_page.set(0);

        state.loader.load(clone!(state => async move {
            state.users_state.load_users().await;
        }));
    }

    pub fn save_admin_data(self: &Rc<Self>, user: &Rc<EditableUser>) {
        self.loader.load(clone!(user => async move {
            let req = PatchProfileAdminDataRequest {
                badge: Some(user.badge.get()),
            };
            endpoints::user::PatchProfileAdminData::api_with_auth(
                PatchProfileAdminDataPath(user.id),
                Some(req),
            )
                .await
                .unwrap_ji();
        }))
    }

    pub fn set_tier_override(self: &Rc<Self>, user: &Rc<EditableUser>, tier: Option<PlanTier>) {
        self.loader.load(clone!(user => async move {
            let req = UpdateNullable::from(tier);

            endpoints::admin::SetAccountTierOverride::api_with_auth(
                SetAccountTierOverridePath(user.id),
                Some(req),
            )
            .await
            .unwrap_ji();
        }))
    }
}
