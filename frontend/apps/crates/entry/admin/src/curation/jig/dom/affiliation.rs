use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::meta::Affiliation;
use std::rc::Rc;
use utils::events;

use crate::curation::jig::state::CurationJig;

const STR_AFFILIATION_LABEL: &'static str = "Affiliation";
const STR_AFFILIATION_PLACEHOLDER: &'static str = "Select one or more";

impl CurationJig {
    pub fn render_affiliations(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("input-select", {
            .property("slot", "affiliation")
            .property("label", STR_AFFILIATION_LABEL)
            .property("placeholder", STR_AFFILIATION_PLACEHOLDER)
            .property("multiple", true)
            .property_signal("value", affiliation_value_signal(state.clone()))
            // .property_signal("error", {
            //     (map_ref! {
            //         let submission_tried = state.submission_tried.signal(),
            //         let value = state.jig.affiliation_ranges.signal_cloned()
            //             => (*submission_tried, value.clone())
            //     })
            //         .map(|(submission_tried, value)| {
            //             submission_tried && value.is_empty()
            //         })
            // })
            .children_signal_vec(state.curation_state.affiliations.signal_cloned().map(clone!(state => move |affiliations| {
                affiliations.iter().map(|affiliation| {
                    render_affiliation(&affiliation, state.clone())
                }).collect()
            })).to_signal_vec())
        })
    }
}

fn render_affiliation(affiliation: &Affiliation, state: Rc<CurationJig>) -> Dom {
    let affiliation_id = affiliation.id.clone();
    html!("input-select-option", {
        .text(&affiliation.display_name)
        .property_signal("selected", state.jig.affiliations.signal_cloned().map(clone!(affiliation_id => move |affiliations| {
            affiliations.contains(&affiliation_id)
        })))
        .event(clone!(state => move |_: events::CustomSelectedChange| {
            let mut affiliations = state.jig.affiliations.lock_mut();
            if affiliations.contains(&affiliation_id) {
                affiliations.remove(&affiliation_id);
            } else {
                affiliations.insert(affiliation_id);
            }
        }))
    })
}

fn affiliation_value_signal(state: Rc<CurationJig>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_affiliations = state.jig.affiliations.signal_cloned(),
        let available_affiliations = state.curation_state.affiliations.signal_cloned() => {
            let mut output = vec![];
            selected_affiliations.iter().for_each(|affiliation_id| {
                let affiliation = available_affiliations.iter().find(|affiliation| affiliation.id == *affiliation_id);
                match affiliation {
                    Some(affiliation) => {
                        output.push(affiliation.display_name.clone());
                    },
                    None => {
                        
                    }
                }
            });
            output.join(", ")
        }
    }
}
