use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    share_asset::ShareAsset,
};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::{Signal, SignalExt},
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
    ages::AgeRangeVecExt,
    asset::{published_at_string, ResourceContentExt},
    events,
    init::analytics,
    prelude::{get_user_cloned, ApiEndpointExt},
    routes::{AssetEditRoute, AssetRoute, CourseEditRoute, JigEditRoute, ResourceEditRoute, Route},
};

use super::state::SearchResultsSection;

const STR_LOAD_MORE: &str = "See more";

impl SearchResultsSection {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        // Only set this once, but I don't want to add once_cell crate when it's not really needed.
        state.user.set(get_user_cloned());

        html!("home-search-results-section", {
            .property("slot", "sections")
            .property("kind", state.asset_type.as_str())
            .property_signal("resultsCount", state.total.signal())
            .children_signal_vec(state.list.signal_vec_cloned().map(clone!(state => move |jig| {
                state.render_result(jig)
            })))
            .child_signal(state.all_loaded_signal().map(clone!(state => move |all_loaded| {
                match all_loaded {
                    true => None,
                    false => {
                        Some(html!("button-rect", {
                            .property("slot", "load-more")
                            .property("color", "blue")
                            .property("type", "filled")
                            .property_signal("disabled", state.loader.is_loading())
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
        let jig_ages = asset.age_ranges().clone();
        let share_asset = ShareAsset::new((*asset).clone());
        let user_id = state.user.get_cloned().map(|user| user.id);

        html!("home-search-result", {
            .property("slot", "results")
            .property("title", asset.display_name())
            .property("playedCount", asset.plays())
            .property("likedCount", asset.likes())
            .property("author", asset.author_name().clone().unwrap_or_default())
            .property("language", asset.language())
            .property_signal("flipped", share_asset.active_popup.signal_cloned().map(|active_popup| active_popup.is_some()))
            .property("kind", state.asset_type.as_str())
            .property("publishedAt", {
                match asset.published_at() {
                    Some(publish_at) => published_at_string(publish_at, false),
                    None => String::new(),
                }
            })
            .child_signal(state.home_state.search_options.age_ranges.signal_cloned().map(move |age_ranges| {
                let range = age_ranges.range(&jig_ages);
                Some(html!("age-range", {
                    .property("slot", "ages")
                    .property("icon", "entry/home/search-results/age.svg")
                    .property("from", range.0)
                    .property("to", range.1)
                }))
            }))
            .property("description", asset.description().clone())
            .child(
                ModuleThumbnail::new(
                    asset.id(),
                    asset.cover().cloned(),
                    ThumbnailFallback::Asset,
                    DraftOrLive::Live
                ).render(Some("image"))
            )
            .apply_if(!asset.categories().is_empty(), clone!(state, asset => move |dom| {
                dom.child(html!("home-search-result-details", {
                    .property("slot", "categories")
                    .child(html!("div", {
                        .children(asset.categories().iter().map(|category_id| {
                            html!("home-search-result-category", {
                                .property_signal("label", {
                                    state.home_state.search_options.category_label_lookup.signal_cloned().map(clone!(category_id => move |category_label_lookup| {
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
            .property("showAdditionalResources", {
                !asset.additional_resources().is_empty()
                &&
                !state.asset_type.is_resource()
            })
            .children(asset.additional_resources().iter().map(|resource| {
                html!("a", {
                    .property("slot", "additional-resources")
                    .property("target", "_BLANK")
                    .property("title", &resource.display_name)
                    .property("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .property("icon", "fa-light fa-file")
                    }))
                    .text(" ")
                    .text_signal(state.resource_type_name(resource.resource_type_id))
                })
            }))
            .child(share_asset.render(
                html!("button-empty", {
                    .style("display", "flex")
                    .style("align-items", "center")
                    .style("gap", "10px")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-share-nodes")
                        .style("font-size", "26px")
                    }))
                    .text(" Share")
                    .event(clone!(asset => move |_: events::Click| {
                        track_action("share", asset.clone());
                    }))
                }),
                Some("actions"),
            ))
            .apply_if(asset.author_id() == &user_id, clone!(asset => move |dom| {
                dom.child(html!("a", {
                    .property("slot", "actions")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-light fa-pencil")
                        .style("font-size", "18px")
                    }))
                    .text(" Edit")
                    .property("href", {
                        match asset.id() {
                            AssetId::JigId(jig_id) => {
                                Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                    jig_id,
                                    JigEditRoute::Landing
                                )))
                            },
                            AssetId::CourseId(course_id) => {
                                Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                                    course_id,
                                    CourseEditRoute::Landing
                                )))
                            },
                            AssetId::ResourceId(resource_id) => {
                                Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                                    resource_id,
                                    ResourceEditRoute::Landing
                                )))
                            },
                        }.to_string()
                    })
                    .event(clone!(asset => move |_: events::Click| {
                        track_action("edit", asset.clone());
                    }))
                }))
            }))
            .apply(|dom| {
                match state.asset_type {
                    AssetType::Jig | AssetType::Course => {
                        dom.child(html!("button-rect-icon", {
                            .property("slot", "play-button")
                            .property("color", "red")
                            .property("bold", true)
                            .property("size", "small")
                            .property("iconBeforePath", "search/cards/play.svg")
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
                                        .property("slot", "play-button")
                                        .property("color", "green")
                                        .property("bold", true)
                                        .property("href", resource.resource_content.get_link())
                                        .property("target", "_BLANK")
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
                                        .property("slot", "play-button")
                                    })
                                },
                            }
                        })
                    },
                }
            })
        })
    }
    // new
    // leaningPathJigCount
    // byJiTeam

    fn resource_type_name(self: &Rc<Self>, id: ResourceTypeId) -> impl Signal<Item = String> {
        self.home_state
            .search_options
            .resource_types
            .signal_ref(move |resource_types| {
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

    let asset_type = match asset_id {
        AssetId::JigId(_) => "Jig",
        AssetId::CourseId(_) => "Course",
        AssetId::ResourceId(_) => "Resource",
    };

    let mut properties = HashMap::new();
    properties.insert("Asset ID", format!("{}", asset_id.uuid()));
    properties.insert("Asset Type", asset_type.to_owned());
    properties.insert("Asset Name", asset.display_name().clone());

    analytics::event(action, Some(properties));
}
