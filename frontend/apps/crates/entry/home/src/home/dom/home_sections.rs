use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::events;

use super::super::{actions::search, state::Testimonial};

use super::super::state::State;

const STR_PLAY: &'static str = "Play Series";
const STR_JIGS: &'static str = "JIGs";

pub fn render(state: Rc<State>) -> Dom {
    html!("empty-fragment", {
        .children(&mut [
            html!("home-quick-search", {
                .children(state.quick_searches.iter().map(clone!(state => move|item| {
                    html!("home-quick-search-item", {
                        .children(&mut [
                            html!("img-ji", {
                                .property("slot", "image")
                                .property("id", &item.image_id)
                                .property("lib", &item.image_lib)
                                .property("size", "original")
                            }),
                            html!("h4", {
                                .property("slot", "title")
                                .text(&item.search_term)
                            }),
                            html!("h5", {
                                .property("slot", "subtitle")
                                .text(&item.jigs_count.to_string())
                                .text(STR_JIGS)
                            }),
                        ])
                        .event(clone!(state, item => move |_: events::Click| {
                            state.search_selected.query.set(item.search_term.clone());
                            search(Rc::clone(&state));
                        }))
                    })
                })))
            }),
            html!("home-create"),
            html!("home-why-ji"),
            html!("home-whats-new", {
                .property("pageCount", state.whats_new.len() as u32)
                .children(state.whats_new.iter().map(|item| {
                    html!("home-new-item", {
                        .property("slot", "items")
                        .children(&mut [
                            html!("img-ji", {
                                .property("slot", "image")
                                .property("id", &item.image_id)
                                .property("lib", &item.image_lib)
                                .property("size", "original")
                            }),
                            html!("h2", {
                                .property("slot", "subtitle")
                                .text(&item.header)
                            }),
                            html!("p", {
                                .property("slot", "lines")
                                .text(&item.paragraph)
                            }),
                            html!("button-rect", {
                                .property("slot", "button")
                                .property("size", "large")
                                .property("color", "red")
                                .property("bold", "")
                                .text(STR_PLAY)
                            }),
                        ])
                    })
                }))
            }),
            html!("home-testimonials", {
                .property("teachersPageCount", state.teachers_testimonials.len() as u32)
                .property("parentsPageCount", state.parents_testimonials.len() as u32)
                .children(state.teachers_testimonials.iter().map(|t| {
                    testimonial(t, "teachers")
                }))
                .children(state.parents_testimonials.iter().map(|t| {
                    testimonial(t, "parents")
                }))
            }),
        ])
    })
}

fn testimonial(testimonial: &Testimonial, slot: &str) -> Dom {
    html!("home-testimonial-item", {
        .property("slot", slot)
        .children(&mut [
            html!("img-ji", {
                .property("slot", "image")
                .property("id", &testimonial.image_id)
                .property("lib", &testimonial.image_lib)
                .property("size", "original")
            }),
            html!("h4", {
                .property("slot", "header")
                .text(&testimonial.header)
            }),
            html!("p", {
                .property("slot", "paragraph")
                .text(&testimonial.paragraph)
            }),
        ])
    })
}
