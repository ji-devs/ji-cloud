use components::module::_common::thumbnail::ModuleThumbnail;
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::{
    jig::{JigFocus, JigResponse},
    meta::ResourceTypeId,
};
use std::rc::Rc;
use utils::{
    ages::AgeRangeVecExt,
    events,
    jig::{published_at_string, ResourceContentExt},
};

use super::state::SearchResultsSection;

const STR_LOAD_MORE: &str = "See more";

impl SearchResultsSection {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("home-search-results-section", {
            .property("slot", "sections")
            .property("kind", match state.focus {
                JigFocus::Modules => "jigs",
                JigFocus::Resources => "resources",
            })
            .property_signal("resultsCount", state.total.signal())
            .children_signal_vec(state.list.signal_vec_cloned().map(clone!(state => move |jig| {
                state.render_result(&jig)
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
                                    let req = state.search_selected.to_search_request();
                                    state.load_items(req).await;
                                }));
                            }))
                        }))
                    },
                }
            })))
        })
    }

    fn render_result(self: &Rc<Self>, jig: &JigResponse) -> Dom {
        let state = self;
        let jig_ages = jig.jig_data.age_ranges.clone();

        html!("home-search-result", {
            .property("slot", "results")
            .property("title", &jig.jig_data.display_name)
            .property("playedCount", jig.plays)
            .property("likedCount", jig.likes)
            .property("author", &jig.author_name.clone().unwrap_or_default())
            .property("language", &jig.jig_data.language)
            .property("kind", match state.focus {
                JigFocus::Modules => "jig",
                JigFocus::Resources => "resource",
            })
            .property("publishedAt", {
                match jig.published_at {
                    Some(publish_at) => published_at_string(publish_at, false),
                    None => String::new(),
                }
            })
            .property_signal("ages", state.search_options.age_ranges.signal_cloned().map(move |age_ranges| {
                age_ranges.range_string(&jig_ages)
            }))
            .property("description", jig.jig_data.description.clone())
            .child(ModuleThumbnail::render(
                Rc::new(ModuleThumbnail {
                    jig_id: jig.id,
                    module: jig.jig_data.modules.first().cloned(),
                    is_jig_fallback: true,
                }),
                Some("image")
            ))
            .apply_if(!jig.jig_data.categories.is_empty(), clone!(state => move |dom| {
                dom.child(html!("home-search-result-details", {
                    .property("slot", "categories")
                    .child(html!("div", {
                        .children(jig.jig_data.categories.iter().map(|category_id| {
                            html!("home-search-result-category", {
                                .property_signal("label", {
                                    state.search_options.category_label_lookup.signal_cloned().map(clone!(category_id => move |category_label_lookup| {
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
                !jig.jig_data.additional_resources.is_empty()
                &&
                jig.jig_focus.is_modules()
            })
            .children(jig.jig_data.additional_resources.iter().map(|resource| {
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
            .apply(|dom| {
                match jig.jig_focus {
                    JigFocus::Modules => {
                        dom.child(html!("button-rect", {
                            .property("slot", "play-button")
                            .property("color", "blue")
                            .property("bold", true)
                            .text("Play")
                            .event({
                                let jig_id = jig.id;
                                clone!(state => move |_: events::Click| {
                                    state.play_jig.set(Some(jig_id));
                                })
                            })
                        }))
                    },
                    JigFocus::Resources => {
                        dom.child({
                            match jig.jig_data.additional_resources.get(0) {
                                Some(resource) => {
                                    html!("button-rect", {
                                        .property("slot", "play-button")
                                        .property("color", "green")
                                        .property("bold", true)
                                        .property("href", resource.resource_content.get_link())
                                        .property("target", "_BLANK")
                                        .text("View")
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
        self.search_options
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
