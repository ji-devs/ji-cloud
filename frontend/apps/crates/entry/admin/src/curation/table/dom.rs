use super::state::*;
use components::module::_common::thumbnail::ModuleThumbnail;
use dominator::{clone, html, Dom, with_node};
use futures_signals::{signal_vec::SignalVecExt, signal::{Signal, SignalExt}, map_ref};
use shared::domain::{jig::JigResponse, meta::{AgeRangeId, AffiliationId}};
use web_sys::HtmlSelectElement;
use std::rc::Rc;
use utils::{languages::Language, events, routes::AdminCurationRoute};

impl CurationTable {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-curation-table", {
            .child(html!("input-search", {
                .property("slot", "search")
                .property("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_jigs(e.query());
                }))
            }))
            .child(html!("button", {
                .property("slot", "pagination")
                .property("title", "Previous")
                .property_signal("disabled", state.curation_state.active_page.signal().map(|active_page| {
                    active_page == 0
                }))
                .text("<")
                .event(clone!(state => move |_: events::Click| {
                    let active_page = state.curation_state.active_page.get();
                    state.curation_state.go_to_page(active_page - 1);
                }))
            }))
            .child_signal(state.curation_state.total_pages.signal().map(clone!(state => move |total_pages| {
                total_pages.map(|total_pages| {
                    html!("select" => HtmlSelectElement, {
                        .with_node!(elem => {
                            .property("slot", "pagination")
                            .property_signal("value", state.curation_state.active_page.signal().map(|active_page| {
                                active_page + 1
                            }))
                            .event(clone!(state => move |_: events::Change| {
                                let page = elem.value().parse::<u32>();
                                if let Ok(page) = page {
                                    state.curation_state.go_to_page(page - 1);
                                };
                            }))
                            .children((0..total_pages).map(|page| {
                                html!("option", {
                                    .text(&(page + 1).to_string())
                                })
                            }))
                        })
                    })
                })
            })))
            .child(html!("button", {
                .property("slot", "pagination")
                .property("title", "Next")
                .property_signal("disabled", map_ref! {
                    let total_pages = state.curation_state.total_pages.signal(),
                    let active_page = state.curation_state.active_page.signal() => {
                        match total_pages {
                            None => true,
                            Some(total_pages) => {
                                // active_page is 0 indexed in the code side, so need to add 1 for display
                                *active_page == total_pages - 1
                            }
                        }
                    }
                })
                .text(">")
                .event(clone!(state => move |_: events::Click| {
                    let active_page = state.curation_state.active_page.get();
                    state.curation_state.go_to_page(active_page + 1);
                }))
            }))
            .children_signal_vec(state.curation_state.jigs.signal_vec_cloned().map(clone!(state => move |jig: JigResponse| {
                let jig_id = jig.id.clone();
                html!("admin-curation-table-line", {
                    .child(html!("div", {
                        .style("display", "grid")
                        .style("grid-template-columns", "repeat(3, 100px)")
                        .style("align-items", "start")
                        .style("padding", "0")
                        .children((0..3).filter_map(|i| {
                            jig.jig_data.modules.get(i).map(|module| {
                                ModuleThumbnail::render(
                                    Rc::new(ModuleThumbnail {
                                        jig_id: jig.id,
                                        module: Some(module.clone()),
                                        is_jig_fallback: true,
                                    }),
                                    None
                                )
                            })
                        }))
                    }))
                    .children(&mut [
                        html!("a", {
                            .text(&jig.jig_data.display_name)
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminCurationRoute::Jig(jig_id);
                                state.curation_state.navigate_to(route);
                            }))
                        }),
                        html!("span", {
                            .text(&jig.author_name.unwrap_or_default())
                        }),
                        // html!("span", {
                        //     .text("AUTHOR BADGE")
                        // }),
                        html!("span", {
                            .text(&match jig.published_at {
                                Some(published_at) => published_at.format("%b %e, %Y").to_string(),
                                None => "".to_string()
                            })
                        }),
                        html!("span", {
                            .text(Language::code_to_display_name(&jig.jig_data.language))
                        }),
                        // html!("span", {
                        //     .text("CURATORS")
                        // }),
                        html!("span", {
                            .style("display", "flex")
                            .style("flex-wrap", "wrap")
                            .style("column-gap", "16px")
                            .children(jig.jig_data.age_ranges.into_iter().map(|age_id| {
                                html!("span", {
                                    .text_signal(state.age_label(age_id))
                                })
                            }))
                        }),
                        html!("span", {
                            .style("display", "flex")
                            .style("flex-wrap", "wrap")
                            .style("column-gap", "16px")
                            .children(jig.jig_data.affiliations.into_iter().map(|affiliation_id| {
                                html!("span", {
                                    .text_signal(state.affiliation_label(affiliation_id))
                                })
                            }))
                        }),
                    ])
                })
            })))
        })
    }

    fn age_label(self: &Rc<Self>, age_id: AgeRangeId) -> impl Signal<Item = String> {
        self.curation_state.ages.signal_ref(move |ages| {
            let age = ages.iter().find(|age| age.id == age_id);
            match age {
                Some(age) => age.display_name.clone(),
                None => "-".to_string(),
            }
        })
    }

    fn affiliation_label(self: &Rc<Self>, affiliation_id: AffiliationId) -> impl Signal<Item = String> {
        self.curation_state.affiliations.signal_ref(move |affiliations| {
            let affiliation = affiliations.iter().find(|affiliation| affiliation.id == affiliation_id);
            match affiliation {
                Some(affiliation) => affiliation.display_name.clone(),
                None => "-".to_string(),
            }
        })
    }
}
