use std::rc::Rc;

use components::overlay::handle::OverlayHandle;
use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal::{not, SignalExt};
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use super::state::SendResetLink;

const STR_DISMISS: &str = "Dismiss";
const STR_RESET_PASSWORD: &str = "Reset Password";

// NOTE: using lots of inline styles since this page was never formally defined

impl SendResetLink {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("empty-fragment" => HtmlElement, {
            .apply(OverlayHandle::lifecycle(
                move || {
                    html!("overlay-content", {
                        .property("flowContentAnchor", "mm")
                        .property("contentAnchor", "mm")
                        .child(html!("dialog-backdrop" => HtmlElement, {
                            .child(html!("popup-body", {
                                .children(&mut [
                                    html!("fa-button", {
                                        .property("slot", "close")
                                        .property("icon", "fa-light fa-xmark")
                                        .event(clone!(state => move |_: events::Click| {
                                            (state.callbacks.on_close)();
                                        }))
                                    }),
                                    html!("h3", {
                                        .property("slot", "heading")
                                        .text(STR_RESET_PASSWORD)
                                    }),
                                ])
                                .child_signal(state.reset_sent.signal().map(clone!(state => move|reset_sent| {
                                    Some(match reset_sent {
                                        true => state.render_sent(),
                                        false => state.render_form()
                                    })
                                })))
                            }))
                        }))
                    })
                }
            ))
        })
    }

    fn render_form(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("form", {
            .style("padding", "30px")
            .style("display", "grid")
            .style("row-gap", "50px")
            .style("justify-items", "center")
            .property("slot", "body")
            .event_with_options(
                &EventOptions::preventable(),
                clone!(state => move |evt: events::Submit| {
                    evt.prevent_default();
                    state.submit();
                })
            )
            .children(&mut [
                html!("input-wrapper", {
                    .property("slot", "email")
                    .property("label", crate::strings::STR_EMAIL_LABEL)
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property("name", "email")
                            .property("type", "email")
                            .property("required", true)
                            .attribute("autocomplete", "email")
                            .event(clone!(state => move |_:events::Input| {
                                state.email.update_value(elem.value());
                            }))
                        })
                    }))
                }),
                html!("button-rect", {
                    .property("submit", true)
                    .text("Reset password")
                    .property_signal("disabled", not(state.email.email_acceptable_signal()))
                    .event(clone!(state => move |_:events::Click| {
                        state.submit();
                    }))
                })
            ])
        })
    }

    fn render_sent(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .style("padding", "30px")
            .property("slot", "body")
            .children(&mut [
                html!("h2", {
                    .text("Check you inbox.")
                }),
                html!("p", {
                    .text("We send you a reset password link to your inbox if there's an account associated with this email address.")
                }),
                html!("p", {
                    .text("Please make sure to check your spam folder if you can't find it.")
                }),
                html!("button-rect", {
                    .text(STR_DISMISS)
                    .event(clone!(state => move |_: events::Click| {
                        (state.callbacks.on_close)();
                    }))
                }),
            ])
        })
    }
}
