use std::rc::Rc;

use dominator::{html, Dom};
use utils::prelude::{get_plan_type, get_school_id};

use crate::strings::register::complete::*;

use super::Welcome;

fn get_email_body() -> String {
    let body = r#"
        Please fill out names and emails
        [name]: [email]
        [name]: [email]
    "#;
    body.replace("\n", "%0A")
}

fn get_email_link() -> String {
    let email_address = "someone@yoursite.com";
    let subject = "Big%20News";
    let body = get_email_body();
    format!("mailto:{email_address}?subject={subject}&body={body}")
}

impl Welcome {
    pub fn render(self: &Rc<Self>) -> Dom {
        let plan = get_plan_type();
        let plan_str = plan.map(|plan| plan.as_str()).unwrap_or("Family");
        let is_school = get_school_id().is_some();
        html!("page-register-complete", {
            .prop("plan", plan_str)
            .child(html!("button-rect", {
                .prop("slot", "actions")
                .prop("color", "red")
                .prop("href", "/")
                .prop("size", "regular")
                .text(STR_SUBMIT)
            }))
            .apply_if(is_school, |dom| {
                dom.child(html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "red")
                    .prop("href", get_email_link())
                    .prop("target", "_BLANK")
                    .prop("size", "regular")
                    .text("Send list of teachers")
                }))
            })
        })
    }
}
