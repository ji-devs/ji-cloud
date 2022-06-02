use std::rc::Rc;

use dominator::clone;
use shared::{api::{endpoints, ApiEndpoint}, domain::badge::Badge, error::EmptyError};
use utils::prelude::api_no_auth;

use super::BadgeDetails;

impl BadgeDetails {
    pub fn load_badge(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let path = endpoints::badge::Get::PATH.replace("{id}", &state.badge_id.0.to_string());
            match api_no_auth::<Badge, EmptyError, ()>(
                &path,
                endpoints::badge::Get::METHOD,
                None
            ).await {
                Ok(badge) => {
                    state.badge.set(Some(badge));
                },
                Err(_) => todo!(),
            }
        }));
    }
}
