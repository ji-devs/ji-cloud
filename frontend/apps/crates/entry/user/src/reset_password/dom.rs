use super::state::PasswordResetPage;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use crate::strings;
use utils::events;

impl PasswordResetPage {
    pub fn render(state: Rc<PasswordResetPage>) -> Dom {
        html!("page-password-reset", {
            .child(html!("window-loader-block", {
                .prop_signal("visible", state.loader.is_loading())
            }))
            .prop_signal("passwordStrength", state.password.strength_signal())
            .child(html!("input-password", {
                .prop("slot", "password")
                .prop("label", strings::STR_PASSWORD_CREATE_LABEL)
                .prop("placeholder", strings::STR_PASSWORD_PLACEHOLDER)
                .prop("autocomplete", "new-password")
                .prop_signal("error", state.show_error_signal().map(|err| {
                    err.is_some()
                }))
                .prop_signal("hint", state.show_error_signal())
                .event(clone!(state => move |evt:events::CustomInput| {
                    state.password.update_value(evt.value());
                }))
            }))
            .child(html!("button-rect", {
                .prop("slot", "submit")
                .prop("color", "red")
                .prop("size", "medium")
                .text(strings::STR_SUBMIT)
                .event(clone!(state => move |_evt:events::Click| {
                    state.change_password();
                }))
            }))
        })
    }
}
