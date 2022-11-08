use super::state::*;
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use components::page_header::state::PageLinks;
use components::{page_footer, page_header};
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::asset::{Asset, DraftOrLive};
use std::collections::HashMap;
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::ages::AgeRangeVecExt;
use utils::asset::published_at_string;
use utils::init::analytics;
use utils::prelude::*;

const STR_DELETE: &str = "Delete";
const STR_DUPLICATE: &str = "Duplicate";
const STR_SEARCH: &str = "Search";
const STR_SHOW_ALL: &str = "Show all";
const STR_SHOW_PUBLISHED: &str = "Show published";
const STR_SHOW_DRAFT: &str = "Show drafts";

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this JIG?";
const STR_DELETE_CONTENT_WARNING_1: &str = "Deleting in Jigzi is ";
const STR_DELETE_CONTENT_WARNING_2: &str = "permanent";
const STR_DELETE_CONTENT_WARNING_3: &str = " and cannot be undone.";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

const STR_LOAD_MORE: &str = "See more";

impl Gallery {
    fn visible_assets_option_string(visible_jigs: &VisibleAssets) -> &'static str {
        match visible_jigs {
            VisibleAssets::All => STR_SHOW_ALL,
            VisibleAssets::Published => STR_SHOW_PUBLISHED,
            VisibleAssets::Draft => STR_SHOW_DRAFT,
        }
    }

    pub fn render(self: Rc<Self>) -> Dom {
        analytics::event("Jig Gallery Load", None);

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
            .child(page_header::dom::render(Rc::new(page_header::state::State::new()), None, Some(PageLinks::Create), true))
            .child_signal(state.confirm_delete.signal().map(clone!(state => move |confirm_delete| {
                confirm_delete.map(|jig_id| {
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
                            state.delete_asset(jig_id);
                        }))
                    })
                })
            })))
            .child(
                html!("jig-gallery", {
                    .prop("assetDisplayName", state.asset_type_name())
                    .child(html!("jig-gallery-create", {
                        .prop("slot", "create-jig")
                        .event(clone!(state => move |_: events::Click| {
                            state.create_asset();
                            analytics::event("Jig Gallery Create", None);
                        }))
                    }))
                    // .apply_if(state.focus.is_modules(), move |dom| {
                    //     dom.children(TEMPLATE_KINDS.iter().map(|kind| {
                    //         html!("jig-gallery-template", {
                    //             .prop("slot", "jig-templates")
                    //             .prop("kind", *kind)
                    //         })
                    //     }))
                    // })
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
                        .prop_signal("value", state.visible_assets.signal_cloned().map(|visible_jigs| Self::visible_assets_option_string(&visible_jigs)))
                        .children(VisibleAssets::iter().map(|option| {
                            html!("input-select-option", {
                                .prop("value", &option.to_string())
                                .text(Self::visible_assets_option_string(&option))
                                .prop_signal("selected", state.visible_assets.signal_cloned().map(clone!(option => move |visible_jigs| {
                                    visible_jigs == option
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
                        let asset_ages = asset.age_ranges().clone();

                        html!("jig-gallery-recent", {
                            .prop("slot", "recent-items")
                            .prop("label", asset.display_name())
                            .prop("draft", !asset.live_up_to_date())
                            .prop("href", {
                                match &asset {
                                    Asset::Jig(jig) => {
                                        String::from(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                            jig.id,
                                            JigEditRoute::Landing
                                        ))))
                                    },
                                    Asset::Resource(resource) => {
                                        String::from(Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                                            resource.id,
                                            ResourceEditRoute::Landing
                                        ))))
                                    },
                                    Asset::Course(course) => {
                                        String::from(Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                                            course.id,
                                            CourseEditRoute::Landing
                                        ))))
                                    },
                                }
                            })
                            .event(clone!(asset => move |_: events::Click| {
                                let mut properties = HashMap::new();
                                properties.insert("Asset ID", asset.id().uuid().to_string());
                                analytics::event("Asset Gallery Edit", Some(properties));
                            }))
                            .child_signal(state.age_ranges.signal_cloned().map(clone!(asset => move |age_ranges| {
                                let icon = match asset.published_at() {
                                    None => "entry/jig/gallery/age-icon-draft.svg",
                                    Some(_) => "entry/jig/gallery/age-icon.svg",
                                };
                                let range = age_ranges.range(&asset_ages);
                                Some(html!("age-range", {
                                    .prop("slot", "ages")
                                    .prop("icon", icon)
                                    .prop("from", range.0)
                                    .prop("to", range.1)
                                }))
                            })))
                            .apply(|dom| {
                                match asset.published_at() {
                                    None => {
                                        // dom.prop("draft", true)
                                        dom
                                    },
                                    Some(published_at) => {
                                        dom.prop("publishedAt", published_at_string(published_at, true))
                                    },
                                }
                            })
                            .child(
                                ModuleThumbnail::new(
                                    asset.id(),
                                    asset.cover().cloned(),
                                    ThumbnailFallback::Asset,
                                    DraftOrLive::Draft
                                ).render(Some("thumbnail"))
                            )
                            .children(&mut [
                                html!("menu-line", {
                                    .prop("slot", "menu-content")
                                    .prop("icon", "duplicate")
                                    .text(STR_DUPLICATE)
                                    .event(clone!(state, asset => move |_: events::Click| {
                                        state.copy_asset(asset.id());
                                    }))
                                }),
                                html!("menu-line", {
                                    .prop("slot", "menu-content")
                                    .prop("icon", "delete")
                                    .text(STR_DELETE)
                                    .event(clone!(state, asset => move |_: events::Click| {
                                        state.confirm_delete.set_neq(Some(asset.id()));
                                    }))
                                }),
                            ])
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
        })
    }
}
