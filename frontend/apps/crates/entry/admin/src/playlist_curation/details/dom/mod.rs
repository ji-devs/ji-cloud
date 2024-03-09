use super::state::PlaylistDetails;
use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::playlist::PlaylistRating;
use std::rc::Rc;
use utils::{events, routes::AdminPlaylistCurationRoute, unwrap::UnwrapJiExt};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

mod affiliation;
mod age;
mod language;

impl PlaylistDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-playlist-details", {
            .prop("slot", "playlist-details")
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
                        let route = AdminPlaylistCurationRoute::Table;
                        state.curation_state.navigate_to(route);
                    }))
                }),
                html!("star-rating", {
                    .prop("slot", "rating")
                    .prop_signal("rating", state.playlist.rating.signal_cloned().map(|rating| {
                        rating.map(|rating| {
                            rating as u8
                        })
                    }))
                    .event(clone!(state => move |e: events::CustomNumber| {
                        let rating = e.number();
                        let rating = rating.map(|rating| {
                            PlaylistRating::try_from(rating as u8).unwrap_ji()
                        });
                        state.playlist.rating.set(rating);
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
                                state.curation_state.navigate_to(AdminPlaylistCurationRoute::Table);
                            }))
                        }),
                        html!("button-rect", {
                            .prop("kind", "filled")
                            .prop("color", "blue")
                            .text("Save and republish")
                            .event(clone!(state => move |_: events::Click| {
                                state.curation_state.save_and_publish(&state.playlist);
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .prop("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .prop("label", "Playlist name")
                            .children(&mut [
                                html!("input" => HtmlInputElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.playlist.display_name.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.playlist.display_name.set(value);
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
                                    .prop("value", &state.playlist.author_name.clone().unwrap_or_default())
                                }),
                            ])
                        }),
                        state.render_languages(),
                        state.render_ages(),
                        state.render_affiliations(),
                        html!("input-wrapper", {
                            .prop("label", "Playlist teacher's description")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .prop_signal("value", state.playlist.description.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.playlist.description.set(value);
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
                                        .prop_signal("value", state.playlist.other_keywords.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.playlist.other_keywords.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                    ])
                }),
            ])
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
                            state.playlist_id.into(),
                            PreviewPopupCallbacks::new(Box::new(on_close)),
                        ).render(Some("player")))
                    }
                }
            })))
            .child(html!("fa-button", {
                .prop("slot", "block")
                .style_signal("color", state.playlist.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "red",
                        false => "green",
                    }
                }))
                .prop_signal("icon", state.playlist.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "fa-solid fa-eye-slash",
                        false => "fa-solid fa-eye",
                    }
                }))
                .prop_signal("title", state.playlist.blocked.signal().map(|blocked| {
                    match blocked {
                        true => "Blocked",
                        false => "Visible",
                    }
                }))
                .event(clone!(state => move |_: events::Click| {
                    let mut blocked = state.playlist.blocked.lock_mut();
                    *blocked = !*blocked;
                }))
            }))
        })
    }
}
