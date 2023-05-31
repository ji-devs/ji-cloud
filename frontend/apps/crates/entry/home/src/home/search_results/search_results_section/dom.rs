use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    share_asset::ShareAsset,
};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::{from_future, Mutable, Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::{
    api::endpoints::resource,
    domain::{
        asset::{Asset, AssetId, AssetType, DraftOrLive},
        meta::ResourceTypeId,
        resource::ResourceViewPath,
    },
};
use std::{collections::HashMap, rc::Rc};
use utils::{
    asset::{published_at_string, ResourceContentExt},
    events,
    init::analytics,
    metadata::{get_category_label_lookup, get_resource_types},
    prelude::{get_user_cloned, ApiEndpointExt},
    routes::{
        AssetEditRoute, AssetRoute, CommunityMembersRoute, CommunityRoute, JigEditRoute,
        PlaylistEditRoute, ResourceEditRoute, Route,
    },
};

use crate::home::actions::search;

use super::state::SearchResultsSection;

const STR_LOAD_MORE: &str = "See more";

impl SearchResultsSection {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        // Only set this once, but I don't want to add once_cell crate when it's not really needed.
        state.user.set(get_user_cloned());

        html!("home-search-results-section", {
            .prop("slot", "sections")
            .prop("kind", state.asset_type.as_str())
            .prop_signal("resultsCount", state.total.signal())
            .apply_if(self.asset_type.is_jig(), |dom| {
                dom.child(state.home_state.search_bar.render_rated_toggle(Rc::new(clone!(state => move || {
                    search(&state.home_state)
                })), Some("rated")))
            })
            .children_signal_vec(state.list.signal_vec_cloned().map(clone!(state => move |jig| {
                state.render_result(jig)
            })))
            .child_signal(state.all_loaded_signal().map(clone!(state => move |all_loaded| {
                match all_loaded {
                    true => None,
                    false => {
                        Some(html!("button-rect", {
                            .prop("slot", "load-more")
                            .prop("color", "blue")
                            .prop("type", "filled")
                            .prop_signal("disabled", state.loader.is_loading())
                            .text(STR_LOAD_MORE)
                            .event(clone!(state => move |_: events::Click| {
                                state.loader.load(clone!(state => async move {
                                    state.load_items().await;
                                }));
                            }))
                        }))
                    },
                }
            })))
        })
    }

    fn render_result(self: &Rc<Self>, asset: Rc<Asset>) -> Dom {
        let state = self;
        let share_asset = ShareAsset::new((*asset).clone());
        let user_id = state.user.get_cloned().map(|user| user.id);
        let user_liked = Mutable::new(asset.is_liked());

        html!("home-search-result", {
            .prop("slot", "results")
            .prop("name", asset.display_name())
            .prop("playedCount", asset.plays())
            .prop("likedCount", asset.likes())
            .prop("language", asset.language())
            .prop_signal("flipped", share_asset.active_popup.signal_cloned().map(|active_popup| active_popup.is_some()))
            .prop("kind", state.asset_type.as_str())
            .prop("publishedAt", {
                match asset.published_at() {
                    Some(publish_at) => published_at_string(publish_at, false),
                    None => String::new(),
                }
            })
            .apply(|mut dom| {
                if let (Some(name), Some(id)) = (asset.author_name(), asset.author_id()) {
                    let url = Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(*id))).to_string();
                    dom = dom.prop("authorName", name);
                    dom = dom.prop("authorLink", url);
                }
                dom
            })
            .child(render_asset_card(
                &asset,
                AssetCardConfig {
                    bottom_indicator: AssetCardBottomIndicator::Author,
                    slot: Some("front"),
                    ..Default::default()
                },
            ))
            .apply(|dom| {
                match &*asset {
                    Asset::Playlist(_) | Asset::Resource(_) => dom,
                    Asset::Jig(jig) => {
                        dom.children(jig.jig_data.modules.iter().map(|module| {
                            ModuleThumbnail::new(
                                asset.id(),
                                Some(module.clone()),
                                ThumbnailFallback::Asset,
                                DraftOrLive::Live
                            ).render(Some("thumbnails"))
                        }))
                    },
                    Asset::ProDev(_) => todo!()
                }
            })
            .prop("description", asset.description().clone())
            .apply_if(!asset.categories().is_empty(), clone!(asset => move |dom| {
                dom.child(html!("home-search-result-details", {
                    .prop("slot", "categories")
                    .child(html!("div", {
                        .children(asset.categories().iter().map(|category_id| {
                            html!("home-search-result-category", {
                                .prop_signal("label", {
                                    from_future(get_category_label_lookup()).map(|x| x.unwrap_or_default()).map(clone!(category_id => move |category_label_lookup| {
                                        match category_label_lookup.get(&category_id) {
                                            Some(label) => label.to_owned(),
                                            None => String::new(),
                                        }
                                    }))
                                })
                            })
                        }))
                    }))
                }))
            }))
            .prop("showAdditionalResources", {
                !asset.additional_resources().is_empty()
                &&
                !state.asset_type.is_resource()
            })
            .children(asset.additional_resources().iter().map(|resource| {
                html!("a", {
                    .prop("slot", "additional-resources")
                    .prop("target", "_BLANK")
                    .prop("title", &resource.display_name)
                    .prop("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-file")
                    }))
                    .text(" ")
                    .text_signal(state.resource_type_name(resource.resource_type_id))
                })
            }))
            .child(share_asset.render(
                html!("button-empty", {
                    .style("height", "32px")
                    .style("width", "32px")
                    .style("display", "inline-grid")
                    .style("place-content", "center")
                    .prop("title", "Share")
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-share-nodes")
                        .style("font-size", "16px")
                    }))
                    .event(clone!(asset => move |_: events::Click| {
                        track_action("share", asset.clone());
                    }))
                }),
                Some("actions"),
            ))
            .apply(clone!(asset => move |dom| {
                match asset.author_id() == &user_id {
                    true => {
                        dom.child(html!("a", {
                            .prop("slot", "actions")
                            .prop("title", "Edit")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-light fa-pencil")
                            }))
                            .prop("href", {
                                match asset.id() {
                                    AssetId::JigId(jig_id) => {
                                        Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                            jig_id,
                                            JigEditRoute::Landing
                                        )))
                                    },
                                    AssetId::PlaylistId(playlist_id) => {
                                        Route::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                                            playlist_id,
                                            PlaylistEditRoute::Landing
                                        )))
                                    },
                                    AssetId::ResourceId(resource_id) => {
                                        Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                                            resource_id,
                                            ResourceEditRoute::Landing
                                        )))
                                    },
                                    AssetId::ProDevId(_) => todo!()
                                }.to_string()
                            })
                            .event(clone!(asset => move |_: events::Click| {
                                track_action("edit", asset.clone());
                            }))
                        }))
                    },
                    false => {
                        dom.child(html!("fa-button", {
                            .prop("slot", "actions")
                            .prop("title", "Like")
                            .attr_signal("icon", user_liked.signal().map(|liked| {
                                match liked {
                                    true => "fa-solid fa-heart",
                                    false => "fa-regular fa-heart",
                                }
                            }))
                            .style_signal("color", user_liked.signal().map(|liked| {
                                match liked {
                                    true => "var(--dark-red-2)",
                                    false => "var(--main-blue)",
                                }
                            }))
                            .event(clone!(state, user_liked => move |_: events::Click| {
                                state.on_like_click(asset.id(), &user_liked);
                            }))
                        }))
                    },
                }
            }))
            .apply(|dom| {
                match state.asset_type {
                    AssetType::Jig | AssetType::Playlist => {
                        dom.child(html!("button-rect-icon", {
                            .prop("slot", "play-button")
                            .prop("color", "red")
                            .prop("bold", true)
                            .prop("size", "small")
                            .prop("iconBeforePath", "search/cards/play.svg")
                            .text("Play")
                            .event(clone!(state => move |_: events::Click| {
                                state.on_play_asset_click(asset.id());
                                track_action("play", asset.clone());
                            }))
                        }))
                    },
                    AssetType::Resource => {
                        dom.child({
                            match asset.additional_resources().get(0) {
                                Some(resource) => {
                                    html!("button-rect", {
                                        .prop("slot", "play-button")
                                        .prop("color", "red")
                                        .prop("bold", true)
                                        .prop("size", "small")
                                        .prop("href", resource.resource_content.get_link())
                                        .prop("target", "_BLANK")
                                        .text("View")
                                        .event(clone!(state => move |_: events::Click| {
                                            track_action("play", asset.clone());

                                            state.loader.load(clone!(asset => async move {
                                                let _ = resource::View::api_no_auth_empty(
                                                    ResourceViewPath(asset.unwrap_resource().id),
                                                    None,
                                                ).await;
                                            }))
                                        }))
                                    })
                                },
                                None => {
                                    // should not be possible, resource focused jigs need to have exactly one additional resource
                                    html!("span", {
                                        .text("Error 😞")
                                        .prop("slot", "play-button")
                                    })
                                },
                            }
                        })
                    },
                    AssetType::ProDev => todo!()
                }
            })
        })
    }
    // new
    // leaningPathJigCount
    // byJiTeam

    fn resource_type_name(self: &Rc<Self>, id: ResourceTypeId) -> impl Signal<Item = String> {
        from_future(get_resource_types())
            .map(|x| x.unwrap_or_default())
            .map(move |resource_types| {
                let resource_type = resource_types
                    .iter()
                    .find(move |resource_type| resource_type.id == id);

                match resource_type {
                    Some(resource_type) => resource_type.display_name.clone(),
                    None => String::new(),
                }
            })
    }
}

fn track_action(action: &str, asset: Rc<Asset>) {
    let asset_id = asset.id();

    let mut properties = HashMap::new();
    properties.insert("Asset ID", format!("{}", asset_id.uuid()));
    properties.insert(
        "Asset Type",
        asset_id.asset_type().display_name().to_string(),
    );
    properties.insert("Asset Name", asset.display_name().clone());

    analytics::event(action, Some(properties));
}
