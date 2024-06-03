use std::{collections::HashMap, rc::Rc};

use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    share_asset::ShareAsset,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::{from_future, Mutable, Signal, SignalExt};
use shared::{
    api::endpoints,
    domain::{
        asset::{Asset, AssetId, AssetType, DraftOrLive},
        jig::{JigLikePath, JigUnlikePath},
        meta::ResourceTypeId,
        playlist::{PlaylistLikePath, PlaylistUnlikePath},
        resource::ResourceViewPath,
        resource::{ResourceLikePath, ResourceUnlikePath},
        user::UserId,
    },
};
use utils::{
    asset::{published_at_string, ResourceContentExt},
    events,
    fetch::ApiEndpointExt,
    init::analytics,
    metadata::{get_category_label_lookup, get_resource_types},
    routes::{
        AssetEditRoute, AssetRoute, CommunityMembersRoute, CommunityRoute, JigEditRoute,
        PlaylistEditRoute, ResourceEditRoute, Route,
    },
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;

pub fn render_flippable_asset_card(
    asset: Rc<Asset>,
    user_id: Option<UserId>,
    on_play: Box<dyn Fn()>,
) -> Dom {
    let share_asset = ShareAsset::new((*asset).clone());
    let user_liked = Mutable::new(asset.is_liked());

    html!("home-search-result", {
        .prop("slot", "results")
        .prop("name", asset.display_name())
        .prop("playedCount", asset.plays())
        .prop("likedCount", asset.likes())
        .prop("language", asset.language())
        .prop("premium", asset.premium())
        .prop_signal("flipped", share_asset.active_popup.signal_cloned().map(|active_popup| active_popup.is_some()))
        .prop("kind", asset.asset_type().as_str())
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
                Asset::Course(_) => todo!()
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
            !asset.asset_type().is_resource()
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
                .text_signal(resource_type_name(resource.resource_type_id))
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
                                AssetId::CourseId(_) => todo!()
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
                        .event(clone!(user_liked => move |_: events::Click| {
                            on_like_click(asset.id(), &user_liked);
                        }))
                    }))
                },
            }
        }))
        .apply(|dom| {
            match asset.asset_type() {
                AssetType::Jig | AssetType::Playlist => {
                    dom.child(html!("button-rect-icon", {
                        .prop("slot", "play-button")
                        .prop("color", "red")
                        .prop("bold", true)
                        .prop("size", "regular")
                        .prop("iconBeforePath", "search/cards/play.svg")
                        .text("Play")
                        .event(move |_: events::Click| {
                            // state.home_state.play_asset.set(Some(asset.id()));
                            on_play();
                            track_action("play", asset.clone());
                        })
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
                                    .prop("size", "regular")
                                    .prop("href", resource.resource_content.get_link())
                                    .prop("target", "_BLANK")
                                    .text("View")
                                    .event(move |_: events::Click| {
                                        track_action("play", asset.clone());

                                        spawn_local(clone!(asset => async move {
                                            let _ = endpoints::resource::View::api_no_auth(
                                                ResourceViewPath(asset.unwrap_resource().id),
                                                None,
                                            ).await;
                                        }))
                                    })
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
                AssetType::Course => todo!()
            }
        })
    })
}

fn resource_type_name(id: ResourceTypeId) -> impl Signal<Item = String> {
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

fn on_like_click(asset_id: AssetId, liked_mutable: &Mutable<bool>) {
    let is_liked = !liked_mutable.get();
    liked_mutable.set(is_liked);
    spawn_local(async move {
        match asset_id {
            AssetId::JigId(jig_id) => {
                match is_liked {
                    true => {
                        endpoints::jig::Like::api_with_auth(JigLikePath(jig_id), None)
                            .await
                            .unwrap_ji();
                    }
                    false => {
                        endpoints::jig::Unlike::api_with_auth(JigUnlikePath(jig_id), None)
                            .await
                            .unwrap_ji();
                    }
                };
            }
            AssetId::PlaylistId(playlist_id) => {
                match is_liked {
                    true => {
                        endpoints::playlist::Like::api_with_auth(
                            PlaylistLikePath(playlist_id),
                            None,
                        )
                        .await
                        .unwrap_ji();
                    }
                    false => {
                        endpoints::playlist::Unlike::api_with_auth(
                            PlaylistUnlikePath(playlist_id),
                            None,
                        )
                        .await
                        .unwrap_ji();
                    }
                };
            }
            AssetId::ResourceId(resource_id) => {
                match is_liked {
                    true => {
                        endpoints::resource::Like::api_with_auth(
                            ResourceLikePath(resource_id),
                            None,
                        )
                        .await
                        .unwrap_ji();
                    }
                    false => {
                        endpoints::resource::Unlike::api_with_auth(
                            ResourceUnlikePath(resource_id),
                            None,
                        )
                        .await
                        .unwrap_ji();
                    }
                };
            }
            AssetId::CourseId(_) => todo!(),
        }
    });
}
