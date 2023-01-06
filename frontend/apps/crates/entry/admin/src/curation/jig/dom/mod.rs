use super::state::CurationJig;
use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{asset::DraftOrLive, jig::JigRating};
use std::rc::Rc;
use utils::{events, routes::AdminCurationRoute, unwrap::UnwrapJiExt};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

mod affiliation;
mod age;
mod language;

impl CurationJig {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-jig-details", {
            .prop("slot", "jig-details")
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.jig.loader.is_loading())
            }))
            .children(&mut [
                html!("button-rect", {
                    .prop("slot", "back")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text("Back")
                    .event(clone!(state => move |_: events::Click| {
                        let route = AdminCurationRoute::Table;
                        state.curation_state.navigate_to(route);
                    }))
                }),
                html!("star-rating", {
                    .prop("slot", "rating")
                    .prop_signal("rating", state.jig.rating.signal_cloned().map(|rating| {
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
                    .prop("slot", "buttons")
                    .children(&mut [
                        html!("button-rect", {
                            .prop("kind", "text")
                            .prop("color", "blue")
                            .text("Cancel")
                            .event(clone!(state => move |_: events::Click| {
                                state.curation_state.navigate_to(AdminCurationRoute::Table);
                            }))
                        }),
                        html!("button-rect", {
                            .prop("kind", "filled")
                            .prop("color", "blue")
                            .text("Save and republish")
                            .event(clone!(state => move |_: events::Click| {
                                state.jig.save_and_publish();
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .prop("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .prop("label", "JIG name")
                            .children(&mut [
                                html!("input" => HtmlInputElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.jig.display_name.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.display_name.set(value);
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
                                    .prop("value", &state.jig.author_name)
                                }),
                            ])
                        }),
                        state.render_languages(),
                        state.render_ages(),
                        state.render_affiliations(),
                        html!("input-wrapper", {
                            .prop("label", "JIG teacher's description")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.jig.description.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.description.set(value);
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
                                        .prop_signal("value", state.jig.other_keywords.signal_cloned())
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
            .child(
                ModuleThumbnail::new(
                    state.jig_id.into(),
                    state.jig.modules.get(0).cloned(),
                    ThumbnailFallback::Asset,
                    DraftOrLive::Live,
                ).render(Some("player"))
            )
            .child(html!("fa-button", {
                .prop("slot", "player")
                .prop("icon", "fa-duotone fa-circle-play")
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
                        Some(PlayerPopup::new_default_player_options(
                            state.jig_id.into(),
                            PreviewPopupCallbacks::new(Box::new(on_close)),
                        ).render(Some("player")))
                    }
                }
            })))
            .child(html!("fa-button", {
                .prop("slot", "block")
                .style_signal("color", state.jig.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "red",
                        false => "green",
                    }
                }))
                .prop_signal("icon", state.jig.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "fa-solid fa-eye-slash",
                        false => "fa-solid fa-eye",
                    }
                }))
                .prop_signal("title", state.jig.blocked.signal().map(|blocked| {
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
