use std::rc::Rc;

use components::overlay::handle::OverlayHandle;
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::{
    asset::{Asset, DraftOrLive},
    jig::JigResponse,
    meta::ResourceTypeId,
};
use utils::{
    ages::AgeRangeVecExt,
    asset::{published_at_string, PlaylistPlayerOptions, ResourceContentExt},
    events,
    routes::{AssetPlayRoute, AssetRoute, CommunityMembersRoute, CommunityRoute, Route},
    unwrap::UnwrapJiExt,
};
use web_sys::HtmlElement;

use super::{super::Sidebar, report};

impl Sidebar {
    pub fn render_info(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("empty-fragment" => HtmlElement, {
            .with_node!(elem => {
                .prop("slot", "actions")
                .event(clone!(state => move |_: events::Close| {
                    state.info_popup_active.set(false);
                }))
                .child(html!("empty-fragment", {
                    .style("display", "flex")
                    .event(clone!(state => move |_: events::Click| {
                        let mut info_popup_active = state.info_popup_active.lock_mut();
                        *info_popup_active = !*info_popup_active;
                        state.track_action("Information Click");
                    }))
                    .child(html!("jig-play-sidebar-action", {
                        .prop("kind", "info")
                        .prop_signal("active", state.info_open_signal())
                    }))
                }))
                .apply(OverlayHandle::lifecycle(
                    clone!(state => move || {
                        html!("overlay-content", {
                            .prop("target", &elem)
                            .prop("contentAnchor", "oppositeH")
                            .prop("targetAnchor", "tr")
                            .event(clone!(state => move |_:events::Close| {
                                state.info_popup_active.set(false);
                            }))
                            .child(html!("empty-fragment", {
                                .child_signal({
                                    (map_ref!{
                                        let info_popup_active = state.info_popup_active.signal_cloned(),
                                        let jig = state.player_state.jig.signal_cloned() => {
                                            (*info_popup_active, jig.clone())
                                        }
                                    }).map(clone!(state => move|(info_popup_active, jig)| {
                                        match (info_popup_active, jig) {
                                            (true, Some(jig)) => {
                                                Some(state.render_jig_info(&jig))
                                            },
                                            _ => None,
                                        }
                                    }))
                                })
                            }))
                        })
                    })
                ))
            })
        })
    }

    fn info_open_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        self.info_popup_active.signal_cloned()
    }

    fn render_jig_info(self: &Rc<Self>, jig: &JigResponse) -> Dom {
        let state = self;
        let asset = Asset::from(jig.clone());

        html!("jig-play-sidebar-jig-info", {
            .prop("slot", "overlay")
            .prop("name", &jig.jig_data.display_name)
            .prop("playedCount", jig.plays as usize)
            .prop("likedCount", jig.likes as usize)
            .prop("language", &jig.jig_data.language)
            .prop("author", jig.author_name.clone())
            .prop("target", "_BLANK")
            .prop("href",  Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(jig.author_id.unwrap_ji()))).to_string())
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
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    state.info_popup_active.set(false);
                }))
            }))
            .apply_if(!asset.categories().is_empty(), clone!(asset, state => move |dom| {
                dom.child(html!("div", {
                    .prop("slot", "category-labels")
                    .child(html!("div", {
                        .children(asset.categories().iter().map(|category_id| {
                            html!("jig-info-category", {
                                .prop_signal("label", state.player_state.category_label_lookup.signal_cloned().map(clone!(category_id => move |category_label_lookup| {
                                    category_label_lookup.get(&category_id).unwrap_ji().clone()
                                })))
                            })
                        }))
                    }))
                }))
            }))
            .apply_if(!jig.jig_data.additional_resources.is_empty(),|dom| {
                dom.prop("showResources", true)
                .children(jig.jig_data.additional_resources.iter().map(|resource| {
                    html!("a", {
                        .prop("slot", "additional-resources")
                        .prop("target", "_BLANK")
                        .prop("href", resource.resource_content.get_link())
                        .prop("title", &resource.display_name)
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-light fa-file")
                        }))
                        .text_signal(state.resource_type_name_signal(resource.resource_type_id))
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
                            .prop("href",  Route::Asset(AssetRoute::Play(AssetPlayRoute::Playlist(
                                playlist.id,
                                PlaylistPlayerOptions {
                                    draft_or_live: DraftOrLive::Live,
                                    is_student: state.player_state.is_student
                                }
                            ))).to_string())
                            .text(format!(" {}  ", &playlist.playlist_data.display_name).as_str())
                        })
                    }).collect()
                })).to_signal_vec())
            })
            .children_signal_vec(report::render(Rc::clone(&state)).to_signal_vec())
        })
    }

    fn resource_type_name_signal(
        self: &Rc<Self>,
        resource_type_id: ResourceTypeId,
    ) -> impl Signal<Item = String> {
        self.player_state
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
}
