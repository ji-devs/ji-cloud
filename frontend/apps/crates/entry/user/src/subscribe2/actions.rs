use dominator::{clone, routing::go_to_url};
use shared::{
    api::endpoints,
    domain::billing::{CreateSubscriptionPath, CreateSubscriptionRequest},
};
use utils::{
    prelude::{get_school_id, ApiEndpointExt},
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;

use super::state::*;
use std::rc::Rc;

impl Subscribe2 {
    pub fn subscribe(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let req: CreateSubscriptionRequest = CreateSubscriptionRequest {
                plan_type: state.plan_type,
                setup_intent_id: Default::default(),
                promotion_code: Default::default()
            };
            endpoints::billing::CreateSubscription::api_with_auth(CreateSubscriptionPath(), Some(req)).await.unwrap_ji();
            go_to_url(&get_next_page_url());
        }));
    }
}

fn get_next_page_url() -> String {
    match get_school_id() {
        Some(_) => Route::User(UserRoute::SchoolEnd),
        None => Route::User(UserRoute::Welcome),
    }
    .to_string()
}
