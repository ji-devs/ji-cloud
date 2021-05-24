use std::rc::Rc;
use dominator::{Dom, html};
use futures_signals::{signal::SignalExt, signal_vec::{MutableVec, SignalVecExt}};
use shared::domain::jig::Jig;

use crate::state::State;


pub fn render(_state: Rc<State>, query: String, jigs: Rc<MutableVec<Jig>>) -> Dom {
    html!("home-search-results", {
        .property_signal("resultsCount", jigs.signal_vec_cloned().len().map(|len| len as u32))
        .property("query", &query)
        .child(
            html!("home-search-results-section", {
                .property("slot", "sections")
                .children_signal_vec(jigs.signal_vec_cloned().map(|jig| {
                    render_result(&jig)
                }))
            })
        )
    })
}

fn render_result(jig: &Jig) -> Dom {
    html!("home-search-result", {
        .property("slot", "results")
        .property("title", &jig.display_name)
        .property("playedCount", "???")
        .property("likedCount", "???")
        .property("language", &jig.language)
        .property("description", jig.description.clone().unwrap_or_default())
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
                        .property("label", category_id.0.to_string().chars().take(10).collect::<String>())
                    })
                }))
            }),
            html!("button-text", {
                .property("slot", "play-button")
                .property("color", "white")
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
