use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::jig::Jig;
use std::rc::Rc;
use utils::ages::AgeRangeVecExt;

use crate::state::State;

pub fn render(state: Rc<State>, query: String, jigs: Rc<MutableVec<Jig>>) -> Dom {
    html!("home-search-results", {
        .property_signal("resultsCount", jigs.signal_vec_cloned().len().map(|len| len as u32))
        .property("query", &query)
        .child(
            html!("home-search-results-section", {
                .property("slot", "sections")
                .children_signal_vec(jigs.signal_vec_cloned().map(clone!(state => move |jig| {
                    render_result(state.clone(), &jig)
                })))
            })
        )
    })
}

fn render_result(state: Rc<State>, jig: &Jig) -> Dom {
    let jig_ages = jig.age_ranges.clone();
    html!("home-search-result", {
        .property("slot", "results")
        .property("title", &jig.display_name)
        .property("playedCount", "???")
        .property("likedCount", "???")
        .property("language", &jig.language)
        .property_signal("ages", state.search_options.age_ranges.signal_cloned().map(move |age_ranges| {
            age_ranges.range_string(&jig_ages)
        }))
        .property("description", jig.description.clone())
        .children(&mut [
            html!("img-ji", {
                .property("lib", "mock")
                .property("size", "full")
                .property("id", "jig-gallery.jpg")
                .property("slot", "image")
            }),
            html!("home-search-result-details", {
                .property("slot", "categories")
                .children(jig.categories.iter().map(|category_id| {
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
            }),
            html!("button-rect", {
                .property("slot", "play-button")
                .property("color", "bluewhite")
                .property("bold", true)
                .text("Play")
            }),
        ])
    })
}

// new: false,
// leaningPathJigCount: undefined,
// // title: "The Big Gematria challenge",
// // playedCount: 10,
// // likedCount: 20,
// ages: "5-8",
// // language: "english",
// byJiTeam: false,
// author: "Corinne",
// // description: "This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry",
