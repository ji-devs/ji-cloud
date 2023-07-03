use dominator::clone;
use shared::{
    api::endpoints,
    domain::billing::{CreateSetupIntentPath, CreateSetupIntentRequest},
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use super::state::*;
use std::rc::Rc;

impl Subscribe {
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
}
