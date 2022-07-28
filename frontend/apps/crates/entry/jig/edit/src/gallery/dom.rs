use super::state::*;
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use components::page_header::state::PageLinks;
use components::{page_footer, page_header};
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::asset::{Asset, DraftOrLive};
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::ages::AgeRangeVecExt;
use utils::asset::published_at_string;
use utils::prelude::*;

const STR_DELETE: &str = "Delete";
const STR_DUPLICATE: &str = "Duplicate";
const STR_SEARCH: &str = "Search";
const STR_SHOW_JIG_ALL: &str = "Show all my JIGs";
const STR_SHOW_JIG_PUBLISHED: &str = "Show published JIGs";
const STR_SHOW_JIG_DRAFT: &str = "Show drafts";

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this JIG?";
const STR_DELETE_CONTENT_WARNING_1: &str = "Deleting in Jigzi is ";
const STR_DELETE_CONTENT_WARNING_2: &str = "permanent";
const STR_DELETE_CONTENT_WARNING_3: &str = " and cannot be undone.";
const STR_DELETE_CONFIRM: &str = "Delete JIG";
const STR_DELETE_CANCEL: &str = "Don't delete";

const STR_LOAD_MORE: &str = "See more";

impl Gallery {
    fn visible_assets_option_string(visible_jigs: &VisibleAssets) -> &'static str {
        match visible_jigs {
            VisibleAssets::All => STR_SHOW_JIG_ALL,
            VisibleAssets::Published => STR_SHOW_JIG_PUBLISHED,
            VisibleAssets::Draft => STR_SHOW_JIG_DRAFT,
        }
    }

    pub fn render(self: Rc<Self>) -> Dom {
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
                        .property("dangerous", true)
                        .property("title", STR_DELETE_TITLE)
                        .property("cancel_text", STR_DELETE_CANCEL)
                        .property("confirm_text", STR_DELETE_CONFIRM)
                        .child(html!("div", {
                            .property("slot", "content")
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
                    .property("assetDisplayName", state.asset_type_name())
                    .child(html!("jig-gallery-create", {
                        .property("slot", "create-jig")
                        .event(clone!(state => move |_: events::Click| {
                            state.create_asset();
                        }))
                    }))
                    // .apply_if(state.focus.is_modules(), move |dom| {
                    //     dom.children(TEMPLATE_KINDS.iter().map(|kind| {
                    //         html!("jig-gallery-template", {
                    //             .property("slot", "jig-templates")
                    //             .property("kind", *kind)
                    //         })
                    //     }))
                    // })
                    .child(html!("input-search", {
                        .style("grid-column", "3") // TODO: remove once draft filter is enabled
                        .property("slot", "search-input")
                        .property("placeholder", STR_SEARCH)
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
                        .property("slot", "filters")
                        .property_signal("value", state.visible_assets.signal_cloned().map(|visible_jigs| Self::visible_assets_option_string(&visible_jigs)))
                        .children(VisibleAssets::iter().map(|option| {
                            html!("input-select-option", {
                                .property("value", &option.to_string())
                                .text(Self::visible_assets_option_string(&option))
                                .property_signal("selected", state.visible_assets.signal_cloned().map(clone!(option => move |visible_jigs| {
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
                    //     .property("slot", "recent-items")
                    //     .property_signal("visible", state.loader.is_loading())
                    // }))
                    .children_signal_vec(state.assets.signal_vec_cloned().map(clone!(state => move |jig| {
                        let jig_ages = jig.age_ranges().clone();
                        html!("jig-gallery-recent", {
                            .property("slot", "recent-items")
                            .property("label", jig.display_name())
                            .property("href", {
                                match &jig {
                                    Asset::Jig(jig) => {
                                        String::from(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                            jig.id,
                                            jig.jig_focus,
                                            JigEditRoute::Landing
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
                            .child_signal(state.age_ranges.signal_cloned().map(clone!(jig => move |age_ranges| {
                                let icon = match jig.published_at() {
                                    None => "entry/jig/gallery/age-icon-draft.svg",
                                    Some(_) => "entry/jig/gallery/age-icon.svg",
                                };
                                let range = age_ranges.range(&jig_ages);
                                Some(html!("age-range", {
                                    .property("slot", "ages")
                                    .property("icon", icon)
                                    .property("from", range.0)
                                    .property("to", range.1)
                                }))
                            })))
                            .apply(|dom| {
                                match jig.published_at() {
                                    None => {
                                        // dom.property("draft", true)
                                        dom
                                    },
                                    Some(published_at) => {
                                        dom.property("publishedAt", published_at_string(published_at, true))
                                    },
                                }
                            })
                            .child(ModuleThumbnail::render(
                                Rc::new(ModuleThumbnail {
                                    asset_id: jig.id(),
                                    module: jig.cover().cloned(),
                                    fallback: ThumbnailFallback::Asset,
                                    draft_or_live: DraftOrLive::Draft
                                }),
                                Some("thumbnail")
                            ))
                            .children(&mut [
                                html!("menu-line", {
                                    .property("slot", "menu-content")
                                    .property("icon", "duplicate")
                                    .text(STR_DUPLICATE)
                                    .event(clone!(state, jig => move |_: events::Click| {
                                        state.copy_asset(jig.id());
                                    }))
                                }),
                                html!("menu-line", {
                                    .property("slot", "menu-content")
                                    .property("icon", "delete")
                                    .text(STR_DELETE)
                                    .event(clone!(state, jig => move |_: events::Click| {
                                        state.confirm_delete.set_neq(Some(jig.id()));
                                    }))
                                }),
                            ])
                        })
                    })))
                    .child_signal(load_more_signal.map(clone!(state => move |load_more| {
                        if load_more {
                            Some(html!("button-rect", {
                                .property("slot", "load-more")
                                .property("color", "blue")
                                .property("type", "filled")
                                .property_signal("disabled", state.loader.is_loading())
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
