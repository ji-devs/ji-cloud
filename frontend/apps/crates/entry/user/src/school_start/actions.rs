use dominator::{clone, routing::go_to_url};
use shared::{
    api::endpoints,
    domain::billing::{CreateSchoolAccountPath, CreateSchoolAccountRequest, SchoolNameRequest},
    domain::user::GetProfilePath,
};
use utils::{
    prelude::{get_user_mutable, ApiEndpointExt},
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};

use super::state::*;
use std::rc::Rc;
use utils::prelude::get_user_cloned;

impl SchoolStart {
    pub fn save(self: &Rc<Self>) {
        let state = self;

        // We shouldn't be at this point without a logged-in user - Unwrap is safe.
        let user = get_user_cloned().unwrap_ji();

        state.loader.load(clone!(state => async move {
            let req = CreateSchoolAccountRequest {
                name: SchoolNameRequest::Value(state.name.get_cloned().into()),
                location: state.location.get_cloned(),
                email: user.email,
                description: Default::default(),
                profile_image: Default::default(),
                website: Default::default(),
                organization_type: Default::default()
            };
            endpoints::account::CreateSchoolAccount::api_with_auth(CreateSchoolAccountPath(), Some(req)).await.unwrap_ji();

            // Update current user state - UserProfile should include account summary now.
            let user = endpoints::user::Profile::api_with_auth(GetProfilePath(), None).await.unwrap_ji();
            get_user_mutable().set(Some(user));

            go_to_url(&Route::User(UserRoute::Subscribe1(state.plan_type)).to_string());
        }));
    }
}
