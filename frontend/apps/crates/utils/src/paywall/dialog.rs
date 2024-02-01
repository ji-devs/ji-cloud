use dominator::DomBuilder;
use gloo::utils::document;
use web_sys::{HtmlElement, ShadowRoot};

use crate::{
    component::Component,
    dialog, events,
    routes::{HomePricingRoute, HomeRoute, Route},
    unwrap::UnwrapJiExt,
};

pub fn dialog_play(msg: &str) {
    show_dialog(msg, "paywall-popups/illustration-play.webp", false)
}

pub fn dialog_limit(msg: &str) {
    show_dialog(msg, "paywall-popups/illustration-limit.webp", true)
}

pub fn dialog_image_theme(msg: &str) {
    show_dialog(msg, "paywall-popups/illustration-image-theme.webp", true)
}

struct PaywallDialog {
    el_id: String,
    msg: String,
    img: String,
    show_no_thanks: bool,
}
impl Component<PaywallDialog> for PaywallDialog {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, host: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        host.prop("id", &self.el_id)
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let el_id = self.el_id.clone();
        dom
            .child(dialog! {
                .child(html!("div", {
                    .class("dialog")
                    .children(&mut [
                        html!("img-ui", {
                            .prop("path", &self.img)
                        }),
                        html!("div", {
                            .children(self.msg.lines().map(|line| {
                                html!("p", {
                                    .text(line.trim())
                                })
                            }))
                        }),
                        html!("div", {
                            .class("actions")
                            .apply_if(self.show_no_thanks, |dom| {
                                dom.child(html!("button-rect", {
                                    .prop("kind", "text")
                                    .prop("color", "blue")
                                    .text("No, thanks")
                                    .event(move |_: events::Click| {
                                        document().get_element_by_id(&el_id).unwrap_ji().remove();
                                    })
                                }))
                            })
                            .child(html!("button-rect", {
                                .prop("kind", "filled")
                                .prop("color", "blue")
                                .prop("href", Route::Home(HomeRoute::Pricing(HomePricingRoute::Individual)).to_string())
                                .text("See our plans")
                            }))
                        })
                    ])
                }))
            })
    }
}

fn show_dialog(msg: &str, img: &str, show_no_thanks: bool) {
    let paywall_dialog = PaywallDialog {
        el_id: js_sys::Math::random().to_string(),
        msg: msg.to_owned(),
        img: img.to_owned(),
        show_no_thanks,
    };
    dominator::append_dom(&document().body().unwrap_ji(), paywall_dialog.render());
}
