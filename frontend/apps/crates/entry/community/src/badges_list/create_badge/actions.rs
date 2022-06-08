use std::rc::Rc;

use dominator::clone;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::badge::{Badge, BadgeCreateRequest},
    error::EmptyError,
};
use utils::prelude::{api_with_auth, ApiEndpointExt};

use super::CreateBadge;

impl CreateBadge {
    pub fn save_badges(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            match state.save_badge_async().await {
                Ok(badge) => {
                    state.badge_list_state.badges.lock_mut().insert_cloned(0, badge);
                    state.badge_list_state.create_popup_open.set(false);
                },
                Err(_) => todo!(),
            }
        }));
    }

    async fn save_badge_async(self: &Rc<Self>) -> anyhow::Result<Badge> {
        let state = self;

        let req = BadgeCreateRequest {
            display_name: state.name.borrow().clone(),
            description: state.description.borrow().clone(),
            thumbnail: url::Url::parse(
                "data:image/gif;base64,R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==",
            )
            .unwrap(),
        };

        let id = endpoints::badge::Create::api_with_auth(Some(req)).await?.id;

        let path = endpoints::badge::Get::PATH.replace("{id}", &id.0.to_string());
        let badge =
            api_with_auth::<Badge, EmptyError, ()>(&path, endpoints::badge::Get::METHOD, None)
                .await?;

        Ok(badge)
    }
}
