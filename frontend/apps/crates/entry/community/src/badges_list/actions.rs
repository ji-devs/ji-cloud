use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::badge::BadgeBrowseQuery};
use utils::prelude::ApiEndpointExt;

use super::BadgesList;

impl BadgesList {
    pub fn load_badges(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let req = BadgeBrowseQuery {
                page: None,
                ..Default::default()
            };

            match endpoints::badge::Browse::api_no_auth(Some(req)).await {
                Ok(badge) => {
                    state.badges.lock_mut().extend(badge.badges);
                },
                Err(_) => todo!(),
            }
        }));
    }
}
