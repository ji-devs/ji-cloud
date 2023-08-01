use dominator::clone;
use shared::{
    api::endpoints,
    domain::billing::{CreateSetupIntentPath, CreateSetupIntentRequest},
};
use utils::{
    prelude::{ApiEndpointExt, SETTINGS},
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};

use super::state::*;
use std::rc::Rc;

impl Subscribe1 {
    pub fn start_intent(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = CreateSetupIntentRequest {
                plan_type: state.plan_type,
            };
            let res = endpoints::billing::CreateSetupIntent::api_with_auth(CreateSetupIntentPath(), Some(req)).await.unwrap_ji();
            state.stripe_client_secret.set(Some(res));
        }));
    }

    pub fn submit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let redirect_url = format!(
                "{}{}",
                SETTINGS.get().unwrap_ji().remote_target.pages_url(),
                Route::User(UserRoute::Subscribe2(state.plan_type, None, state.promo.get_cloned())).to_string()
            );

            let stripe = state.stripe.borrow();
            let stripe = stripe.as_ref().unwrap_ji();
            stripe.submit(&redirect_url).await;
        }));
    }
}
