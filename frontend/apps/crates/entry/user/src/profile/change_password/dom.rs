use std::rc::Rc;

use dominator::{Dom, clone, html};
use utils::events;

use crate::profile::state::ActivePopup;

use super::super::state::State;

const STR_FORGOT_PASSWORD: &'static str = "Forgot your password?";
const STR_SAVE: &'static str = "Save";
const STR_CANCEL: &'static str = "Cancel";

pub fn render(state: Rc<State>) -> Dom {
    html!("user-profile-reset-password-popup", {
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                }))
            }),
            html!("input-password", {
                .property("slot", "inputs")
                .property("label", "Current password")
                .property("placeholder", "Type your password")
            }),
            html!("button-rect", {
                .property("slot", "inputs")
                .property("kind", "text")
                .property("color", "blue")
                .text(STR_FORGOT_PASSWORD)
            }),
            html!("input-password", {
                .property("slot", "inputs")
                .property("label", "New password")
                .property("placeholder", "Type your password")
            }),
            html!("input-password", {
                .property("slot", "inputs")
                .property("label", "Retype new password")
                .property("placeholder", "Type your password")
            }),
            html!("button-text", {
                .property("slot", "cancel")
                .text(STR_CANCEL)
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                }))
            }),
            html!("button-rect", {
                .property("slot", "save")
                .property("color", "blue")
                .text(STR_SAVE)
            }),
        ])
    })
}
