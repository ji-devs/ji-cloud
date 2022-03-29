use super::state::CurationJig;
use components::{
    module::_common::thumbnail::ModuleThumbnail,
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::JigRating;
use std::rc::Rc;
use utils::{events, jig::JigPlayerOptions, routes::AdminCurationRoute, unwrap::UnwrapJiExt};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

mod affiliation;
mod age;
mod language;

impl CurationJig {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-curation-jig-details", {
            .property("slot", "jig-details")
            .child(html!("window-loader-block", {
                .property("slot", "loader")
                .property_signal("visible", state.jig.loader.is_loading())
            }))
            .children(&mut [
                html!("button-rect", {
                    .property("slot", "back")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text("Back")
                    .event(clone!(state => move |_: events::Click| {
                        let route = AdminCurationRoute::Table;
                        state.curation_state.navigate_to(route);
                    }))
                }),
                html!("star-rating", {
                    .property("slot", "rating")
                    .property_signal("rating", state.jig.rating.signal_cloned().map(|rating| {
                        rating.map(|rating| {
                            rating as u8
                        })
                    }))
                    .event(clone!(state => move |e: events::CustomRatingChange| {
                        let rating = e.rating();
                        let rating = rating.map(|rating| {
                            JigRating::try_from(rating).unwrap_ji()
                        });
                        state.jig.rating.set(rating);
                    }))
                }),
                html!("div", {
                    .property("slot", "buttons")
                    .children(&mut [
                        html!("button-rect", {
                            .property("kind", "text")
                            .property("color", "blue")
                            .text("Cancel")
                            .event(clone!(state => move |_: events::Click| {
                                state.curation_state.navigate_to(AdminCurationRoute::Table);
                            }))
                        }),
                        html!("button-rect", {
                            .property("kind", "filled")
                            .property("color", "blue")
                            .text("Save and republish")
                            .event(clone!(state => move |_: events::Click| {
                                state.jig.save_and_publish();
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .property("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .property("label", "JIG name")
                            .children(&mut [
                                html!("input" => HtmlInputElement, {
                                    .with_node!(elem => {
                                        .property_signal("value", state.jig.display_name.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.display_name.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .property("label", "Author name")
                            .children(&mut [
                                html!("input", {
                                    .property("readOnly", true)
                                    .property("value", &state.jig.author_name)
                                }),
                            ])
                        }),
                        state.render_languages(),
                        state.render_ages(),
                        state.render_affiliations(),
                        html!("input-wrapper", {
                            .property("label", "JIG teacher's description")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .property_signal("value", state.jig.description.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.description.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .property("label", "Additional keywords")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .property_signal("value", state.jig.other_keywords.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.other_keywords.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                    ])
                }),
            ])
            .child(ModuleThumbnail::render(
                Rc::new(ModuleThumbnail {
                    jig_id: state.jig_id,
                    module: state.jig.modules.get(0).cloned(),
                    is_jig_fallback: true,
                }),
                Some("player")
            ))
            .child(html!("fa-button", {
                .property("slot", "player")
                .property("icon", "fa-duotone fa-circle-play")
                .event(clone!(state => move |_: events::Click| {
                    state.player_open.set(true);
                }))
            }))
            .child_signal(state.player_open.signal().map(clone!(state => move |player_open| {
                match player_open {
                    false => None,
                    true => {
                        let on_close = clone!(state => move|| {
                            state.player_open.set(false);
                        });
                        Some(PlayerPopup::new(
                            state.jig_id,
                            JigPlayerOptions::default(),
                            PreviewPopupCallbacks::new(Box::new(on_close)),
                        ).render(Some("player")))
                    }
                }
            })))
            .child(html!("fa-button", {
                .property("slot", "block")
                .style_signal("color", state.jig.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "red",
                        false => "green",
                    }
                }))
                .property_signal("icon", state.jig.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "fa-solid fa-eye-slash",
                        false => "fa-solid fa-eye",
                    }
                }))
                .property_signal("title", state.jig.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "Blocked",
                        false => "Visible",
                    }
                }))
                .event(clone!(state => move |_: events::Click| {
                    let mut blocked = state.jig.blocked.lock_mut();
                    *blocked = !*blocked;
                }))
            }))
        })
    }
}
