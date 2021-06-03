use std::rc::Rc;
use dominator::{html, Dom};

use crate::state::Testimonial;

use super::super::state::State;

const STR_PLAY: &'static str = "Play Series";

pub fn render(state: Rc<State>) -> Dom {
    html!("empty-fragment", {
        .children(&mut [
            html!("home-quick-search", {
                .children(state.quick_searches.iter().map(|item| {
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
                                .text(&item.title)
                            }),
                            html!("h5", {
                                .property("slot", "subtitle")
                                .text(&item.subtitle)
                            }),
                        ])
                    })
                }))
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
