use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::{asset::DraftOrLive, jig::JigResponse, meta::ResourceTypeId};
use utils::{
    ages::AgeRangeVecExt,
    asset::{published_at_string, PlaylistPlayerOptions, ResourceContentExt},
    events,
    routes::{AssetPlayRoute, AssetRoute, Route},
};

use super::{super::state::State, report, track_action};

pub fn render(state: Rc<State>) -> Dom {
    html!("anchored-overlay", {
        .prop("positionY", "bottom-out")
        .prop("positionX", "left-in")
        .prop("styled", true)
        .prop("slot", "actions")
        .prop_signal("open", info_open_signal(Rc::clone(&state)))
        .event(clone!(state => move |_: events::Close| {
            state.info_popup_active.set(false);
        }))
        .child(html!("jig-play-sidebar-action", {
            .prop("slot", "anchor")
            .prop("kind", "info")
            .prop_signal("active", info_open_signal(Rc::clone(&state)))
            .event(clone!(state => move |_: events::Click| {
                let mut info_popup_active = state.info_popup_active.lock_mut();
                *info_popup_active = !*info_popup_active;
                track_action("Information Click", state.clone());
            }))
        }))
        .child_signal({
            (map_ref!{
                let info_popup_active = state.info_popup_active.signal_cloned(),
                let jig = state.player_state.jig.signal_cloned() => {
                    (*info_popup_active, jig.clone())
                }
            }).map(move|(info_popup_active, jig)| {
                match (info_popup_active, jig) {
                    (true, Some(jig)) => {
                        Some(render_jig_info(Rc::clone(&state), &jig))
                    },
                    _ => None,
                }
            })
        })
    })
}

fn info_open_signal(state: Rc<State>) -> impl Signal<Item = bool> {
    state.info_popup_active.signal_cloned()
}

fn render_jig_info(state: Rc<State>, jig: &JigResponse) -> Dom {
    html!("jig-play-sidebar-jig-info", {
        .prop("slot", "overlay")
        .prop("name", &jig.jig_data.display_name)
        .prop("playedCount", jig.plays as usize)
        .prop("likedCount", jig.likes as usize)
        .prop("language", &jig.jig_data.language)
        .prop("author", jig.author_name.clone())
        .prop("publishedAt", {
            match jig.published_at {
                Some(publish_at) => published_at_string(publish_at, false),
                None => String::new(),
            }
        })
        .prop("description", &jig.jig_data.description)
        .child_signal(state.all_ages.signal_cloned().map(clone!(jig => move |age_ranges| {
            let range = age_ranges.range(&jig.jig_data.age_ranges);
            Some(html!("age-range", {
                .prop("slot", "ages")
                .prop("icon", "entry/jig/play/sidebar/age.svg")
                .prop("from", range.0)
                .prop("to", range.1)
            }))
        })))
        .child(html!("button-empty", {
            .prop("slot", "close")
            .text("×")
            .event(clone!(state => move |_: events::Click| {
                state.info_popup_active.set(false);
            }))
        }))
        .children(jig.jig_data.categories.iter().map(|category_id| {
            html!("pill-close", {
                .prop("slot", "categories")
                .prop("label", &category_id.0.to_string())
            })
        }))
        .apply_if(!jig.jig_data.additional_resources.is_empty(),|dom| {
            dom.prop("showResources", true)
            .children(jig.jig_data.additional_resources.iter().map(|resource| {
                html!("a", {
                    .prop("slot", "additional-resources")
                    .prop("target", "_BLANK")
                    .prop("title", &resource.display_name)
                    .prop("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-file")
                    }))
                    .text(format!(" {}  ", &resource.display_name).as_str())
                    .text_signal(resource_type_name_signal(Rc::clone(&state), resource.resource_type_id))
                })
            }))
        })
        .apply_if(!state.player_state.playlists.get_cloned().is_empty() ,|dom| {
            dom.prop("showPlaylists", true)
            .children_signal_vec(state.player_state.playlists.signal_cloned().map(clone!(state => move |playlist| {
                playlist.into_iter().map(|playlist| {
                    html!("a", {
                        .prop("slot", "playlists")
                        .prop("target", "_BLANK")
                        .prop("title", &playlist.playlist_data.display_name)
                        .prop("href",  Route::Asset(
                                            AssetRoute::Play(AssetPlayRoute::Playlist(playlist.id,
                                                    PlaylistPlayerOptions {
                                                        draft_or_live: DraftOrLive::Live,
                                                        is_student: state.player_state.player_options.is_student
                                                    }))).to_string()
                                )
                        .text(format!(" {}  ", &playlist.playlist_data.display_name).as_str())
                    })
                }).collect()
            })).to_signal_vec())
        })
        .children_signal_vec(report::render(Rc::clone(&state)).to_signal_vec())
    })
}

fn resource_type_name_signal(
    state: Rc<State>,
    resource_type_id: ResourceTypeId,
) -> impl Signal<Item = String> {
    state
        .player_state
        .resource_types
        .signal_cloned()
        .map(move |resource_types| {
            let resource_type = resource_types
                .iter()
                .find(|resource_type| resource_type_id == resource_type.id);

            match resource_type {
                None => String::new(),
                Some(resource_type) => resource_type.display_name.to_owned(),
            }
        })
}
