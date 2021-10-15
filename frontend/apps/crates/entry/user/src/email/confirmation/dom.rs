use super::state::*;
use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

impl SendEmailConfirmationPage {
    pub fn render(state: Rc<SendEmailConfirmationPage>) -> Dom {
        html!("page-email-send", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            //this doesn't actually make sense here... the *only* thing we have is their email
            //it's equivilent to just creating a new account which is simpler
            /*
            .child(html!("button-rect", {
                .property("slot", "change")
                .property("color", "blue")
                .text(crate::strings::STR_CHANGE_EMAIL)
            }))
            */
            .child(html!("button-email-send", {
                .property("slot", "send")
                .property_signal("mode", state.mode_str())
                .event(clone!(state => move |_:events::Click| {
                    state.resend();
                }))
            }))
        })
    }
}
