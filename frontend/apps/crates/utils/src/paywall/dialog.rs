use gloo::utils::document;

use crate::{
    dialog, events, gap,
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

fn show_dialog(msg: &str, img: &str, show_no_thanks: bool) {
    let id = js_sys::Math::random().to_string();
    dominator::append_dom(
        &document().body().unwrap_ji(),
        dialog! {
            .prop("id", &id)
            .style("padding-block", "72px 45px")
            .style("padding-inline", "16px")
            .style("display", "grid")
            .style("width", "700px")
            .style("max-width", "90vw")
            .children(&mut [
                html!("img-ui", {
                    .style("height", "190px")
                    .style("display", "grid")
                    .style("place-content", "center")
                    .prop("path", img)
                }),
                gap!(40),
                html!("div", {
                    .children(msg.lines().map(|line| {
                        html!("p", {
                            .style("margin", "0")
                            .style("font-size", "18px")
                            .style("font-weight", "500")
                            .style("color", "var(--dark-blue-5)")
                            .style("text-align", "center")
                            .text(line.trim())
                        })
                    }))
                }),
                gap!(75),
                html!("div", {
                    .style("display", "grid")
                    .style("grid-template-columns", "auto auto")
                    .style("justify-content", "center")
                    .style("gap", "24px")
                    .apply_if(show_no_thanks, |dom| {
                        dom.child(html!("button-rect", {
                            .prop("kind", "text")
                            .prop("color", "blue")
                            .text("No, thanks")
                            .event(move |_: events::Click| {
                                document().get_element_by_id(&id).unwrap_ji().remove();
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
        },
    );
}
