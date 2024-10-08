use dominator::{clone, routing::go_to_url};
use shared::{
    api::endpoints,
    domain::{
        billing::{CreateSubscriptionPath, CreateSubscriptionRequest},
        user::GetProfilePath,
    },
};
use utils::{
    bail_on_err,
    error_ext::ErrorExt,
    prelude::{get_school_id, get_user_mutable, ApiEndpointExt},
    routes::{Route, UserRoute},
};
use wasm_bindgen_futures::spawn_local;

use super::state::*;
use std::rc::Rc;
use utils::routes::WelcomeParams;

impl Subscribe2 {
    pub fn subscribe(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let req: CreateSubscriptionRequest = CreateSubscriptionRequest {
                plan_type: state.plan_type,
                setup_intent_id: state.params
                    .as_ref()
                    .and_then(|params| params.setup_intent.clone()),
                promotion_code: state.promo.clone(),
            };
            let res = endpoints::billing::CreateSubscription::api_with_auth(CreateSubscriptionPath(), Some(req)).await.toast_on_err();
            let _ = bail_on_err!(res);

            // Update current user state - UserProfile should include a plan now.
            let res = endpoints::user::Profile::api_with_auth(GetProfilePath(), None).await.toast_on_err();
            let user = bail_on_err!(res);
            get_user_mutable().set(Some(user));

            go_to_url(&get_next_page_url());
        }));
    }
}

fn get_next_page_url() -> String {
    match get_school_id() {
        Some(_) => Route::User(UserRoute::SchoolEnd),
        None => Route::User(UserRoute::Welcome(WelcomeParams { subscribed: true })),
    }
    .to_string()
}
