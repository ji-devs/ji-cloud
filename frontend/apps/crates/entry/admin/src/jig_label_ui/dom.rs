use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::{signal_vec::SignalVecExt, signal::Signal};
use shared::domain::{jig::JigResponse, meta::{AgeRangeId, AffiliationId}};
use std::rc::Rc;
use utils::languages::Language;

impl JigUI {
    fn render_jig_span(slot: &str, text: String) -> Dom {
        html!("span", {
            .attribute("slot", slot)
            .text(&text)
        })
    }
    fn render_jig_details() -> Dom {
        html!("jig-details", {
            .attribute("slot", "jig-details")
            .children(&mut [
                html!("div", {
                    .attribute("slot", "buttons")
                    .children(&mut [
                        html!("button-rect", {
                            .attribute("kind", "text")
                            .attribute("color", "blue")
                            .text("JIG's name")
                        }),
                        html!("button-rect", {
                            .attribute("kind", "outline")
                            .attribute("color", "blue")
                            .text("Save Changes")
                        }),
                    ])
                }),
                html!("div", {
                    .attribute("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .attribute("label", "JIG's name")
                            .children(&mut [
                                html!("input", {
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .attribute("label", "Author name")
                            .children(&mut [
                                html!("input", {
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                        html!("input-select", {
                            .attribute("label", "Instruction Language")
                            .children(&mut [
                                html!("input-select-option", {
                                    .text("English")
                                }),
                                html!("input-select-option", {
                                    .text("Spanish")
                                }),
                                html!("input-select-option", {
                                    .text("Hebrew")
                                }),
                                html!("input-select-option", {
                                    .text("French")
                                }),
                                html!("input-select-option", {
                                    .text("Italian")
                                }),
                            ])
                        }),
                        html!("input-select", {
                            .attribute("label", "Suitable for age")
                            .children(&mut [
                                html!("input-select-option", {
                                    .text("All ages")
                                }),
                                html!("input-select-option", {
                                    .text("No ages")
                                }),
                            ])
                        }),
                        html!("input-select", {
                            .attribute("label", "Affiliation")
                            .children(&mut [
                                html!("input-select-option", {
                                    .text("Affiliation 1")
                                }),
                                html!("input-select-option", {
                                    .text("Affiliation 2")
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .attribute("label", "JIG teacher's description")
                            .children(&mut [
                                html!("textarea", {
                                    .attribute("id", "description")
                                    .attribute("rows", "6")
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .attribute("label", "Additional keywords")
                            .children(&mut [
                                html!("textarea", {
                                    .attribute("rows", "6")
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                    ])
                })
            ])
        })
    }
    pub fn render(state: Rc<Self>) -> Dom {
        state.load_data();

        html!("jig-label-ui", {
            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |jig: JigResponse| {
                html!("single-jig", {
                    .children(&mut [
                        Self::render_jig_span("jig-name", jig.jig_data.display_name),
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
                        Self::render_jig_details(),
                    ])
                })
            })))
        })
    }

    fn age_label(self: &Rc<Self>, age_id: AgeRangeId) -> impl Signal<Item = String> {
        self.ages.signal_ref(move |ages| {
            match ages.get(&age_id) {
                Some(age) => age.display_name.clone(),
                None => "-".to_string(),
            }
        })
    }

    fn affiliation_label(self: &Rc<Self>, affiliation_id: AffiliationId) -> impl Signal<Item = String> {
        self.affiliations.signal_ref(move |affiliations| {
            match affiliations.get(&affiliation_id) {
                Some(affiliation) => affiliation.display_name.clone(),
                None => "-".to_string(),
            }
        })
    }
}
