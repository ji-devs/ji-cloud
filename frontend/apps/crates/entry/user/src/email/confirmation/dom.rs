use dominator::{clone, Dom, html};
use std::rc::Rc;
use super::state::*;
use utils::prelude::*;

impl SendEmailConfirmationPage {
    pub fn render(state: Rc<SendEmailConfirmationPage>) -> Dom {
        html!("page-email-send", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .child(html!("button-rect", {
                .property("slot", "change")
                .property("color", "blue")
                .text(crate::strings::STR_CHANGE_EMAIL)
            }))
            .child(html!("button-email-send", {
                .property("slot", "send")
                .property_signal("mode", state.mode_str())
                .event(clone!(state => move |evt:events::Click| {
                    state.resend();
                }))
            }))
        })
    }
}
