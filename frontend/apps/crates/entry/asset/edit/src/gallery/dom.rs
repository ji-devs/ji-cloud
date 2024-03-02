use super::state::*;
use components::asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig};
use components::page_header::PageHeaderConfig;
use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use components::{
    page_footer,
    page_header::{PageHeader, PageLinks},
};
use dominator::{clone, html, link, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use gloo::utils::window;
use shared::domain::asset::{Asset, AssetId};
use std::collections::HashMap;
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::asset::{AssetPlayerOptions, ResourceContentExt};
use utils::init::analytics;
use utils::prelude::*;

const STR_EDIT: &str = "Edit";
const STR_PLAY: &str = "Play";
const STR_VIEW: &str = "View";
// const STR_SHARE: &str = "Share";
const STR_DUPLICATE: &str = "Duplicate";
const STR_DELETE: &str = "Delete";
const STR_SEARCH: &str = "Search";
const STR_SHOW_ALL: &str = "Show all";
const STR_SHOW_PUBLISHED: &str = "Show published";
const STR_SHOW_DRAFT: &str = "Show drafts";

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this?";
const STR_DELETE_CONTENT_WARNING_1: &str = "Deleting in Jigzi is ";
const STR_DELETE_CONTENT_WARNING_2: &str = "permanent";
const STR_DELETE_CONTENT_WARNING_3: &str = " and cannot be undone.";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

const STR_LOAD_MORE: &str = "See more";

impl Gallery {
    fn visible_assets_option_string(visible_assets: &VisibleAssets) -> &'static str {
        match visible_assets {
            VisibleAssets::All => STR_SHOW_ALL,
            VisibleAssets::Published => STR_SHOW_PUBLISHED,
            VisibleAssets::Draft => STR_SHOW_DRAFT,
        }
    }

    pub fn render(self: Rc<Self>) -> Dom {
        analytics::event("Asset Gallery Load", None);

        let state = self;
        state.load_data();

        let load_more_signal = map_ref! {
            let total_asset_count = state.total_asset_count.signal_cloned(),
            let assets = state.assets.signal_vec_cloned().to_signal_cloned()
                => {
                    match total_asset_count {
                        Some(total_asset_count) => {
                            (assets.len() as u64) < *total_asset_count
                        },
                        None => false
                    }
                }
        };

        html!("empty-fragment", {
            .child(PageHeader::new(PageHeaderConfig {
                active_page: Some(PageLinks::Create),
                ..Default::default()
            }).render())
            .child_signal(state.confirm_delete.signal().map(clone!(state => move |confirm_delete| {
                confirm_delete.map(|asset_id| {
                    html!("modal-confirm", {
                        .prop("dangerous", true)
                        .prop("title", STR_DELETE_TITLE)
                        .prop("cancel_text", STR_DELETE_CANCEL)
                        .prop("confirm_text", STR_DELETE_CONFIRM)
                        .prop("confirmIcon", "core/menus/delete-white.svg")
                        .child(html!("div", {
                            .prop("slot", "content")
                            .child(html!("p", {
                                .text(STR_DELETE_CONTENT)
                            }))
                            .child(html!("p", {
                                .text(STR_DELETE_CONTENT_WARNING_1)
                                .child(html!("strong", {
                                    .text(STR_DELETE_CONTENT_WARNING_2)
                                }))
                                .text(STR_DELETE_CONTENT_WARNING_3)
                            }))
                        }))
                        .event(clone!(state => move |_evt: events::CustomCancel| state.confirm_delete.set_neq(None)))
                        .event(clone!(state => move |_evt: events::CustomConfirm| {
                            state.confirm_delete.set_neq(None);
                            state.delete_asset(asset_id);
                        }))
                    })
                })
            })))
            .child(
                html!("asset-gallery", {
                    .prop("kind", state.asset_type.as_str())
                    .prop("assetDisplayName", state.asset_type_name())
                    .child(link!(Route::Asset(AssetRoute::Studio).to_string(), {
                        .prop("slot", "back")
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-regular fa-chevron-left")
                        }))
                        .text("Back to Jigzi Studio")
                    }))
                    .child(html!("asset-gallery-create", {
                        .prop("slot", "create-asset")
                        .prop("assetName", state.asset_type_name())
                        .event(clone!(state => move |_: events::Click| {
                            state.create_asset();
                            analytics::event("Asset Gallery Create", None);
                        }))
                    }))
                    .child(html!("input-search", {
                        .style("grid-column", "3") // TODO: remove once draft filter is enabled
                        .prop("slot", "search-input")
                        .prop("placeholder", STR_SEARCH)
                        .event(clone!(state => move |evt: events::CustomSearch| {
                            let value = evt.query();
                            if !value.is_empty() {
                                state.search_assets(value);
                            } else {
                                state.load_assets_regular();
                            }
                        }))
                    }))
                    .child(html!("input-select", {
                        .visible(false)
                        .prop("slot", "filters")
                        .prop_signal("value", state.visible_assets.signal_cloned().map(|visible_assets| Self::visible_assets_option_string(&visible_assets)))
                        .children(VisibleAssets::iter().map(|option| {
                            html!("input-select-option", {
                                .prop("value", &option.to_string())
                                .text(Self::visible_assets_option_string(&option))
                                .prop_signal("selected", state.visible_assets.signal_cloned().map(clone!(option => move |visible_assets| {
                                    visible_assets == option
                                })))
                                .event(clone!(state, option => move |evt: events::CustomSelectedChange| {
                                    if evt.selected() {
                                        state.visible_assets.set(option.clone());
                                        state.load_assets_regular();
                                    }
                                }))
                            })
                        }))
                    }))
                    // todo: deal with loading
                    // .child(html!("window-loader-block", {
                    //     .prop("slot", "recent-items")
                    //     .prop_signal("visible", state.loader.is_loading())
                    // }))
                    .children_signal_vec(state.assets.signal_vec_cloned().map(clone!(state => move |asset| {
                        let asset_id = asset.id();
                        let resource_link = match &asset {
                            Asset::Resource(resource) => match resource.resource_data.additional_resources.first() {
                                Some(resource) => Some(resource.resource_content.get_link()),
                                None => None,
                            },
                            _ => None,
                        };
                        html!("a", {
                            .prop("slot", "recent-items")
                            .event(clone!(asset => move |_: events::Click| {
                                let mut properties = HashMap::new();
                                properties.insert("Asset ID", asset.id().uuid().to_string());
                                analytics::event("Asset Gallery Edit", Some(properties));
                            }))
                            .prop("href", get_asset_link(asset_id))
                            .child(render_asset_card(
                                &asset,
                                AssetCardConfig {
                                    bottom_indicator: AssetCardBottomIndicator::Status,
                                    dense: true,
                                    menu: Some(Rc::new(clone!(state, resource_link => move || {
                                        html!("menu-kebab", {
                                            .prop("slot", "menu")
                                            .child(html!("menu-line", {
                                                .prop("icon", "edit")
                                                .text(STR_EDIT)
                                                .event(clone!(asset_id => move |_: events::Click| {
                                                    let route = get_asset_link(asset_id);
                                                    dominator::routing::go_to_url(&route);
                                                }))
                                            }))
                                            .apply(clone!(state, resource_link => move |dom| {
                                                match (asset_id, resource_link) {
                                                    (AssetId::ResourceId(_), Some(link)) => {
                                                        dom.child(html!("menu-line", {
                                                            .prop("icon", "view")
                                                            .text(STR_VIEW)
                                                            .event(clone!(link => move |_: events::Click| {
                                                                log::info!("{link:?}");
                                                                let _ = window().open_with_url_and_target(&link, "_BLANK");
                                                            }))
                                                        }))
                                                    },
                                                    (AssetId::ResourceId(_), None) => {
                                                        // empty resource
                                                        dom
                                                    },
                                                    _ => {
                                                        dom.child(html!("menu-line", {
                                                            .prop("icon", "play")
                                                            .text(STR_PLAY)
                                                            .event(clone!(state, asset_id => move |_: events::Click| {
                                                                state.play_asset.set(Some(asset_id));
                                                            }))
                                                        }))
                                                    }
                                                }
                                            }))
                                            .children(&mut [
                                                // ShareAsset::new(asset.clone()).render(
                                                //     html!("menu-line", {
                                                //         .prop("icon", "share")
                                                //         .text(STR_SHARE)
                                                //     }),
                                                //     None
                                                // ),
                                                html!("menu-line", {
                                                    .prop("icon", "duplicate")
                                                    .text(STR_DUPLICATE)
                                                    .event(clone!(state, asset_id => move |_: events::Click| {
                                                        state.copy_asset(asset_id);
                                                    }))
                                                }),
                                                html!("menu-line", {
                                                    .prop("icon", "delete")
                                                    .text(STR_DELETE)
                                                    .event(clone!(state, asset_id => move |_: events::Click| {
                                                        state.confirm_delete.set_neq(Some(asset_id));
                                                    }))
                                                }),
                                            ])
                                        })
                                    }))),
                                    ..Default::default()
                                },
                            ))
                        })
                    })))
                    .child_signal(load_more_signal.map(clone!(state => move |load_more| {
                        if load_more {
                            Some(html!("button-rect", {
                                .prop("slot", "load-more")
                                .prop("color", "blue")
                                .prop("type", "filled")
                                .prop_signal("disabled", state.loader.is_loading())
                                .text(STR_LOAD_MORE)
                                .event(clone!(state => move |_: events::Click| {
                                    state.loader.load(clone!(state => async move {
                                        state.load_data();
                                    }));
                                }))
                            }))
                        } else {
                            None
                        }
                    })))
                })
            )
            .child(page_footer::dom::render(None))
            .child_signal(state.play_asset.signal().map(clone!(state => move |play_asset| {
                play_asset.map(|asset_id| {
                    PlayerPopup::new(
                        asset_id,
                        None,
                        None,
                        AssetPlayerOptions::default_from_id_draft(&asset_id),
                        PreviewPopupCallbacks {
                            close: Box::new(clone!(state => move|| {
                                state.play_asset.set(None);
                            }))
                        },
                    ).render(None)
                })
            })))
        })
    }
}

fn get_asset_link(asset_id: AssetId) -> String {
    match asset_id {
        AssetId::JigId(jig_id) => Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
            jig_id,
            JigEditRoute::Landing,
        )))
        .into(),
        AssetId::ResourceId(resource_id) => Route::Asset(AssetRoute::Edit(
            AssetEditRoute::Resource(resource_id, ResourceEditRoute::Landing),
        ))
        .into(),
        AssetId::PlaylistId(playlist_id) => Route::Asset(AssetRoute::Edit(
            AssetEditRoute::Playlist(playlist_id, PlaylistEditRoute::Landing),
        ))
        .into(),
        AssetId::CourseId(course_id) => Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
            course_id,
            CourseEditRoute::Landing,
        )))
        .into(),
    }
}
