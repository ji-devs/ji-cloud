use std::rc::Rc;

use super::Welcome;
use dominator::{html, Dom};
use shared::domain::billing::PlanType;
use utils::routes::{HomePricingRoute, UserRoute};
use utils::{
    prelude::{get_plan_type, get_school_id, get_user_email, get_user_mutable},
    routes::{AssetRoute, HomeRoute, Route},
    unwrap::UnwrapJiExt,
};

fn get_add_teacher_form_link() -> String {
    let user = get_user_mutable();
    let user = user.lock_ref();
    let user = user.as_ref();
    let user = user.unwrap_ji();

    let name_of_school = user.school_or_organization().clone().unwrap_or_default();
    let email = user.email.clone();
    let first_name = user.given_name.clone();
    let last_name = user.family_name.clone();
    format!("https://docs.google.com/forms/d/e/1FAIpQLSeYmxx5Dhe0yS5KuLFfet-VyP7fE3JVxP7xYnvGteTp5bw2CA/viewform?usp=pp_url&entry.751628773={name_of_school}&entry.1836812708={email}&entry.998789064={first_name}&entry.328136787={last_name}")
}
const STR_TITLE: &str = "Welcome to your Jigzi ";

impl Welcome {
    fn page_title(self: &Rc<Self>, plan_kind: Option<PlanType>) -> String {
        let end = match plan_kind {
            None => "family!",
            Some(PlanType::IndividualBasicMonthly | PlanType::IndividualBasicAnnually) => "Basic!",
            Some(PlanType::IndividualProMonthly | PlanType::IndividualProAnnually) => "Pro!",
            Some(
                PlanType::SchoolLevel1Monthly
                | PlanType::SchoolLevel2Monthly
                | PlanType::SchoolLevel3Monthly
                | PlanType::SchoolLevel4Monthly
                | PlanType::SchoolUnlimitedMonthly
                | PlanType::SchoolLevel1Annually
                | PlanType::SchoolLevel2Annually
                | PlanType::SchoolLevel3Annually
                | PlanType::SchoolLevel4Annually
                | PlanType::SchoolUnlimitedAnnually,
            ) => "School plan!",
        };
        format!("{} {}", STR_TITLE, end)
    }
    pub fn render(self: &Rc<Self>) -> Dom {
        let plan = get_plan_type();
        let email = get_user_email().unwrap_or_default();
        let title = self.page_title(plan);

        let is_school = get_school_id().is_some();
        html!("page-register-complete", {
            .child(html!("h1", {
                .prop("slot", "headings")
                .text(&title)
            }))
            .apply_if(is_school, |dom| {
                dom.child(html!("h2", {
                    .prop("slot", "headings")
                    .text("Thank you for signing up. The next step is to send us the emails of your team members so we can upgrade these accounts to Pro.")
                }))
                .child(html!("h2", {
                    .prop("slot", "headings")
                    .text(&format!("Look out for an email we will send to {email} once everyone is processed."))
                }))
                .child(html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "filled")

                    .prop("href", get_add_teacher_form_link())
                    .prop("target", "_blank")
                    .text("Next step...")
                }))
                .child(html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "filled")

                    .prop("href", "/")
                    .text("Done")
                }))
                .child(html!("p", {
                    .prop("slot", "help")
                    .text("Need help? Contact us at: ")
                    .child(html!("button-rect", {
                        .prop("kind", "text")
                        .prop("color", "blue")

                        .prop("href", "mailto:schools@jigzi.org")
                        .text("schools@jigzi.org")
                    }))
                }))
            })
            .apply_if(!is_school, |dom| {
                let route = if self.subscribed {
                    format!("{}#plan", Route::User(UserRoute::Settings))
                } else {
                    format!("{}", Route::Home(HomeRoute::Pricing(HomePricingRoute::default())))
                };

                dom.child(html!("h2", {
                    .prop("slot", "headings")
                    .text("You can now create, play, and share your content.")
                }))
                .child(html!("h2", {
                    .prop("slot", "headings")
                    .text("We are here to help.")
                }))
                .child(html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "filled")
                    .prop("href", Route::Home(HomeRoute::Search(None)).to_string())
                    .text("Start exploring")
                }))
                .child(html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "filled")
                    .prop("href", Route::Asset(AssetRoute::Studio).to_string())
                    .text("Start creating")
                }))
                .child(html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "filled")
                    .prop("href", route)
                    .text("Upgrade account")
                }))
            })
        })
    }
}
