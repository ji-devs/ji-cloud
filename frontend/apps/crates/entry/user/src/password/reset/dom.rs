use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::{strings, register::{
    state::Step,
    components::footer::Footer
}};

impl PasswordResetPage {
    pub fn render(state: Rc<PasswordResetPage>) -> Dom {
        html!("page-password-reset", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .property_signal("passwordStrength", state.password.get_strength())
            .child(
                    html!("input-password", {
                        .property("slot", "password")
                        .property("label", strings::STR_PASSWORD_LABEL)
                        .property("placeholder", strings::STR_PASSWORD_PLACEHOLDER)
                        .property("autocomplete", "new-password")
                        .property_signal("error", state.password.error().map(|err| {
                            !err.is_empty()
                        }))
                        .property_signal("hint", state.password.error())
                        .event(clone!(state => move |evt:events::CustomInput| {
                            state.password.clear_status();
                            *state.password.value.borrow_mut() = evt.value();
                            state.password.update_strength();
                        }))
                    })
            )
            .child(
                    html!("button-rect", {
                        .property("slot", "submit")
                        .property("color", "red")
                        .property("size", "medium")
                        .text(strings::STR_SUBMIT)
                        .event(clone!(state => move |evt:events::Click| {
                            actions::change_password(state.clone())
                        }))
                    })
            )
        })
    }
}
