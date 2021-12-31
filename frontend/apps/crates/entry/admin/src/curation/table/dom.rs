use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::{signal_vec::SignalVecExt, signal::Signal};
use shared::domain::{jig::JigResponse, meta::{AgeRangeId, AffiliationId}};
use std::rc::Rc;
use utils::{languages::Language, events, routes::AdminCurationRoute};

impl CurationTable {
    fn render_jig_span(slot: &str, text: String) -> Dom {
        html!("span", {
            .attribute("slot", slot)
            .text(&text)
        })
    }
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-curation", {
            .children_signal_vec(state.curation_state.jigs.signal_vec_cloned().map(clone!(state => move |jig: JigResponse| {
                let jig_id = jig.id.clone();
                html!("admin-curation-single-jig", {
                    .children(&mut [
                        html!("span", {
                            .attribute("slot", "jig-name")
                            .text(&jig.jig_data.display_name)
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminCurationRoute::Jig(jig_id);
                                state.curation_state.navigate_to(route);
                            }))
                        }),
                        Self::render_jig_span("author", match jig.author_name {
                            Some(name) => name,
                            None => "".to_string()
                        }),
                        Self::render_jig_span("author-badge", "AUTHOR BADGE".to_string()),
                        Self::render_jig_span("date", match jig.published_at {
                            Some(published_at) => published_at.format("%b %e, %Y").to_string(),
                            None => "".to_string()
                        }),
                        Self::render_jig_span("language", {
                            Language::code_to_display_name(&jig.jig_data.language).to_string()
                        }),
                        Self::render_jig_span("curators", "CURATORS".to_string()),
                        html!("span", {
                            .property("slot", "age-ranges")
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
                            .property("slot", "affiliations")
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
