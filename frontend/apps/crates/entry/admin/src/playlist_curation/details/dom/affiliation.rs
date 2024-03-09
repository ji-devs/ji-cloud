use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::meta::Affiliation;
use std::rc::Rc;
use utils::events;

use crate::playlist_curation::details::state::PlaylistDetails;

const STR_AFFILIATION_LABEL: &str = "Affiliation";
const STR_AFFILIATION_PLACEHOLDER: &str = "Select one or more";

impl PlaylistDetails {
    pub fn render_affiliations(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(self);
        html!("input-select", {
            .prop("slot", "affiliation")
            .prop("label", STR_AFFILIATION_LABEL)
            .prop("placeholder", STR_AFFILIATION_PLACEHOLDER)
            .prop("multiple", true)
            .prop_signal("value", affiliation_value_signal(state.clone()))
            // .prop_signal("error", {
            //     (map_ref! {
            //         let submission_tried = state.submission_tried.signal(),
            //         let value = state.playlist.affiliation_ranges.signal_cloned()
            //             => (*submission_tried, value.clone())
            //     })
            //         .map(|(submission_tried, value)| {
            //             submission_tried && value.is_empty()
            //         })
            // })
            .children_signal_vec(state.curation_state.affiliations.signal_cloned().map(clone!(state => move |affiliations| {
                affiliations.iter().map(|affiliation| {
                    render_affiliation(affiliation, state.clone())
                }).collect()
            })).to_signal_vec())
        })
    }
}

fn render_affiliation(affiliation: &Affiliation, state: Rc<PlaylistDetails>) -> Dom {
    let affiliation_id = affiliation.id;
    html!("input-select-option", {
        .text(&affiliation.display_name)
        .prop_signal("selected", state.playlist.affiliations.signal_cloned().map(clone!(affiliation_id => move |affiliations| {
            affiliations.contains(&affiliation_id)
        })))
        .event(clone!(state => move |_: events::CustomSelectedChange| {
            let mut affiliations = state.playlist.affiliations.lock_mut();
            if affiliations.contains(&affiliation_id) {
                affiliations.remove(&affiliation_id);
            } else {
                affiliations.insert(affiliation_id);
            }
        }))
    })
}

fn affiliation_value_signal(state: Rc<PlaylistDetails>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_affiliations = state.playlist.affiliations.signal_cloned(),
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
