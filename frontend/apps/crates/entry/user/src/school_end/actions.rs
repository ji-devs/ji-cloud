use dominator::clone;
use shared::{
    api::endpoints,
    domain::billing::{SchoolAccountPath, UpdateSchoolAccountRequest},
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};

use super::state::*;
use std::rc::Rc;

impl SchoolEnd {
    pub fn save(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = UpdateSchoolAccountRequest {
                website: state.website.get_cloned().into(),
                profile_image: state.profile_image.get_cloned().into(),
                description: state.description.get_cloned().into(),
                organization_type: state.organization_type.get_cloned().into(),
                ..Default::default()
            };
            endpoints::account::UpdateSchoolAccount::api_with_auth_empty(SchoolAccountPath(state.school_id), Some(req)).await.unwrap_ji();
            dominator::routing::go_to_url(&Route::User(UserRoute::Welcome).to_string());
        }));
    }
}
