use super::state::ResourceDetails;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::resource::ResourceRating;
use std::rc::Rc;
use utils::{events, routes::AdminResourceCurationRoute, unwrap::UnwrapJiExt};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

mod affiliation;
mod age;
mod language;

impl ResourceDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-resource-details", {
            .prop("slot", "resource-details")
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.loader.is_loading())
            }))
            .children(&mut [
                html!("button-rect", {
                    .prop("slot", "back")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text("Back")
                    .event(clone!(state => move |_: events::Click| {
                        let route = AdminResourceCurationRoute::Table;
                        state.curation_state.navigate_to(route);
                    }))
                }),
                html!("star-rating", {
                    .prop("slot", "rating")
                    .prop_signal("rating", state.resource.rating.signal_cloned().map(|rating| {
                        rating.map(|rating| {
                            rating as u8
                        })
                    }))
                    .event(clone!(state => move |e: events::CustomRatingChange| {
                        let rating = e.rating();
                        let rating = rating.map(|rating| {
                            ResourceRating::try_from(rating).unwrap_ji()
                        });
                        state.resource.rating.set(rating);
                    }))
                }),
                html!("div", {
                    .prop("slot", "buttons")
                    .children(&mut [
                        html!("button-rect", {
                            .prop("kind", "text")
                            .prop("color", "blue")
                            .text("Cancel")
                            .event(clone!(state => move |_: events::Click| {
                                state.curation_state.navigate_to(AdminResourceCurationRoute::Table);
                            }))
                        }),
                        html!("button-rect", {
                            .prop("kind", "filled")
                            .prop("color", "blue")
                            .text("Save and republish")
                            .event(clone!(state => move |_: events::Click| {
                                state.curation_state.save_and_publish(&state.resource)
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .prop("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .prop("label", "RESOURCE name")
                            .children(&mut [
                                html!("input" => HtmlInputElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.resource.display_name.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.resource.display_name.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .prop("label", "Author name")
                            .children(&mut [
                                html!("input", {
                                    .prop("readOnly", true)
                                    .prop("value", &state.resource.author_name.clone().unwrap_or_default())
                                }),
                            ])
                        }),
                        state.render_languages(),
                        state.render_ages(),
                        state.render_affiliations(),
                        html!("input-wrapper", {
                            .prop("label", "RESOURCE teacher's description")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.resource.description.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.resource.description.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .prop("label", "Additional keywords")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.resource.other_keywords.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.resource.other_keywords.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                    ])
                }),
            ])
            .child(html!("fa-button", {
                .prop("slot", "block")
                .style_signal("color", state.resource.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "red",
                        false => "green",
                    }
                }))
                .prop_signal("icon", state.resource.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "fa-solid fa-eye-slash",
                        false => "fa-solid fa-eye",
                    }
                }))
                .prop_signal("title", state.resource.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "Blocked",
                        false => "Visible",
                    }
                }))
                .event(clone!(state => move |_: events::Click| {
                    let mut blocked = state.resource.blocked.lock_mut();
                    *blocked = !*blocked;
                }))
            }))
        })
    }
}
