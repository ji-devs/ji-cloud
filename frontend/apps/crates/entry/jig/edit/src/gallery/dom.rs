use super::state::*;
use components::module::_common::thumbnail::ModuleThumbnail;
use components::page_header::state::PageLinks;
use components::{page_footer, page_header};
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::ages::AgeRangeVecExt;
use utils::jig::published_at_string;
use utils::prelude::*;

const STR_DELETE: &'static str = "Delete";
const STR_DUPLICATE: &'static str = "Duplicate";
const STR_SEARCH: &'static str = "Search";
const STR_SHOW_JIG_ALL: &'static str = "Show all my JIGs";
const STR_SHOW_JIG_PUBLISHED: &'static str = "Show published JIGs";
const STR_SHOW_JIG_DRAFT: &'static str = "Show drafts";

const STR_DELETE_TITLE: &'static str = "Warning";
const STR_DELETE_CONTENT: &'static str = "Are you sure you want to delete this JIG?";
const STR_DELETE_CONFIRM: &'static str = "Delete JIG";
const STR_DELETE_CANCEL: &'static str = "Don't delete";

const STR_LOAD_MORE: &str = "See more";

impl JigGallery {
    fn visible_jigs_option_string(visible_jigs: &VisibleJigs) -> &'static str {
        match visible_jigs {
            VisibleJigs::All => STR_SHOW_JIG_ALL,
            VisibleJigs::Published => STR_SHOW_JIG_PUBLISHED,
            VisibleJigs::Draft => STR_SHOW_JIG_DRAFT,
        }
    }

    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        let load_more_signal = map_ref! {
            let total_jig_count = state.total_jig_count.signal_cloned(),
            let jigs = state.jigs.signal_vec_cloned().to_signal_cloned()
                => {
                    match total_jig_count {
                        Some(total_jig_count) => {
                            (jigs.len() as u64) < *total_jig_count
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
                        .property("content", STR_DELETE_CONTENT)
                        .property("cancel_text", STR_DELETE_CANCEL)
                        .property("confirm_text", STR_DELETE_CONFIRM)
                        .event(clone!(state => move |_evt: events::CustomCancel| state.confirm_delete.set_neq(None)))
                        .event(clone!(state => move |_evt: events::CustomConfirm| {
                            state.confirm_delete.set_neq(None);
                            state.delete_jig(jig_id);
                        }))
                    })
                })
            })))
            .child(
                html!("jig-gallery", {
                    .property("jigFocus", state.focus.as_str())
                    .child(html!("jig-gallery-create", {
                        .property("slot", "create-jig")
                        .event(clone!(state => move |_: events::Click| {
                            state.create_jig();
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
                                state.search_jigs(value);
                            } else {
                                state.load_jigs_regular();
                            }
                        }))
                    }))
                    .child(html!("input-select", {
                        .visible(false)
                        .property("slot", "filters")
                        .property_signal("value", state.visible_jigs.signal_cloned().map(|visible_jigs| Self::visible_jigs_option_string(&visible_jigs)))
                        .children(VisibleJigs::iter().map(|option| {
                            html!("input-select-option", {
                                .property("value", &option.to_string())
                                .text(Self::visible_jigs_option_string(&option))
                                .property_signal("selected", state.visible_jigs.signal_cloned().map(clone!(option => move |visible_jigs| {
                                    if visible_jigs == option {
                                        true
                                    } else {
                                        false
                                    }
                                })))
                                .event(clone!(state, option => move |evt: events::CustomSelectedChange| {
                                    if evt.selected() {
                                        state.visible_jigs.set(option.clone());
                                        state.load_jigs_regular();
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
                    .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |jig| {
                        let jig_ages = jig.jig_data.age_ranges.clone();
                        html!("jig-gallery-recent", {
                            .property("slot", "recent-items")
                            .property("label", jig.jig_data.display_name.clone())
                            .property("href", {
                                String::from(Route::Jig(JigRoute::Edit(
                                    jig.id,
                                    jig.jig_focus,
                                    JigEditRoute::Landing
                                )))
                            })
                            .property_signal("ages", state.age_ranges.signal_cloned().map(move|age_ranges| {
                                age_ranges.range_string(&jig_ages)
                            }))
                            .apply(|dom| {
                                match jig.published_at {
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
                                    jig_id: jig.id.clone(),
                                    module: jig.jig_data.modules.first().cloned(),
                                    is_jig_fallback: true,
                                }),
                                Some("thumbnail")
                            ))
                            .children(&mut [
                                html!("menu-line", {
                                    .property("slot", "menu-content")
                                    .property("icon", "duplicate")
                                    .text(STR_DUPLICATE)
                                    .event(clone!(state, jig => move |_: events::Click| {
                                        state.copy_jig(&jig.id);
                                    }))
                                }),
                                html!("menu-line", {
                                    .property("slot", "menu-content")
                                    .property("icon", "delete")
                                    .text(STR_DELETE)
                                    .event(clone!(state, jig => move |_: events::Click| {
                                        state.confirm_delete.set_neq(Some(jig.id));
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
