use std::rc::Rc;

use dominator::{clone, html, Dom};
use utils::events;

use crate::{profile::state::ActivePopup, strings::profile::*};

use super::super::state::State;

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
                .property("label", STR_CURRENT_PASSWORD_LABEL)
                .property("placeholder", STR_PASSWORD_PLACEHOLDER)
            }),
            html!("button-rect", {
                .property("slot", "inputs")
                .property("kind", "text")
                .property("color", "blue")
                .text(STR_FORGOT_PASSWORD)
            }),
            html!("input-password", {
                .property("slot", "inputs")
                .property("label", STR_NEW_PASSWORD_LABEL)
                .property("placeholder", STR_PASSWORD_PLACEHOLDER)
            }),
            html!("input-password", {
                .property("slot", "inputs")
                .property("label", STR_RETYPE_NEW_PASSWORD_LABEL)
                .property("placeholder", STR_PASSWORD_PLACEHOLDER)
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
