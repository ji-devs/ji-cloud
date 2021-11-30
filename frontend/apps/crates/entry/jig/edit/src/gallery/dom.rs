use super::{actions, state::*};
use components::module::_common::thumbnail::ModuleThumbnail;
use components::page_header::state::PageLinks;
use components::{page_footer, page_header};
use dominator::{clone, html, Dom};
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

        html!("empty-fragment", {
            .child(page_header::dom::render(Rc::new(page_header::state::State::new()), None, Some(PageLinks::Create)))
            .child(
                html!("jig-gallery", {
                    .property("jigFocus", state.focus.as_str())
                    .child(html!("jig-gallery-create", {
                        .property("slot", "create-jig")
                        .event(clone!(state => move |_: events::Click| {
                            state.create_jig();
                        }))
                    }))
                    .apply_if(state.focus.is_modules(), clone!(state => move |dom| {
                        dom.children(TEMPLATE_KINDS.iter().map(|kind| {
                            html!("jig-gallery-template", {
                                .property("slot", "jig-templates")
                                .property("kind", *kind)
                            })
                        }))
                    }))
                    .child(html!("input-search", {
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
                            .property("href", jig.id.0.to_string())
                            .property("label", jig.jig_data.display_name.clone())
                            .property_signal("ages", state.age_ranges.signal_cloned().map(move|age_ranges| {
                                age_ranges.range_string(&jig_ages)
                            }))
                            .apply(|dom| {
                                match jig.published_at {
                                    None => {
                                        dom.property("draft", true)
                                    },
                                    Some(published_at) => {
                                        dom.property("publishedAt", published_at_string(published_at, true))
                                    },
                                }
                            })
                            .child(ModuleThumbnail::render(
                                Rc::new(ModuleThumbnail {
                                    jig_id: jig.id.clone(),
                                    module: jig.jig_data.modules[0].clone(),
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
                                        state.delete_jig(jig.id);
                                    }))
                                }),
                            ])
                        })
                    })))
                })
            )
            .child(page_footer::dom::render(None))
        })
    }
}
