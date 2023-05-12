use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use std::rc::Rc;

use components::{
    page_footer,
    page_header::{PageHeader, PageHeaderConfig},
};
use shared::api::endpoints::billing::GetSubscriptionPlans;
use shared::domain::billing::SubscriptionPlanPath;
use utils::prelude::*;

pub mod actions;
pub mod dom;
pub mod state;

use state::Subscribe;

pub fn render_subscribe(redirect: Option<SubscribeRedirect>) -> Dom {
    let plans = Mutable::new(None);

    html!("div", {
        .future(clone!(plans => async move {
            // TODO match for failure and set a failure state
            plans.set(
                GetSubscriptionPlans::api_no_auth(SubscriptionPlanPath(), None)
                    .await
                    .ok()
                    .map(Rc::new)
            );
        }))
        .child(PageHeader::new(PageHeaderConfig::default()).render())
        .child(Subscribe::new(plans, redirect).render())
        .child(page_footer::dom::render(None))
    })
}
