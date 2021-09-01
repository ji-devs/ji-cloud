use dominator::{clone, html, Dom};
use utils::routes::{JigRoute, Route};
use std::rc::Rc;
use utils::events;

use super::super::{actions::search, state::Testimonial};

use super::super::state::State;

const STR_PLAY: &'static str = "Play Series";
const STR_JIGS: &'static str = "JIGs";

const STR_CONTENT_ACTION: &'static str = "See our library";
const STR_CREATE_ACTION: &'static str = "Try it for free";
const STR_CUSTOMIZE_ACTION: &'static str = "See our lesson outlines";
const STR_COMMUNITY_ACTION: &'static str = "Get inspired";
const STR_CLASSROOM_ACTION: &'static str = "Manage your class";

pub fn render(state: Rc<State>) -> Dom {
    html!("empty-fragment", {
        .children(&mut [
            html!("home-quick-search", {
                .visible(false)
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
            html!("home-why-ji", {
                .children(&mut [
                    html!("home-why-ji-item", {
                        .property("kind", "content")
                        .children(&mut [
                            html!("button-rect", {
                                .property("kind", "text")
                                .property("color", "blue")
                                .property("size", "small")
                                .property("weight", "normal")
                                .text(STR_CONTENT_ACTION)
                            }),
                        ])
                    }),
                    html!("home-why-ji-item", {
                        .property("kind", "create")
                        .children(&mut [
                            html!("button-rect", {
                                .property("kind", "text")
                                .property("color", "blue")
                                .property("size", "small")
                                .property("weight", "normal")
                                .property("href", &Route::Jig(JigRoute::Gallery).to_string())
                                .text(STR_CREATE_ACTION)
                            }),
                        ])
                    }),
                    html!("home-why-ji-item", {
                        .property("kind", "customize")
                        .children(&mut [
                            html!("button-rect", {
                                .property("kind", "text")
                                .property("color", "blue")
                                .property("size", "small")
                                .property("weight", "normal")
                                .property("href", &Route::Jig(JigRoute::Gallery).to_string())
                                .text(STR_CUSTOMIZE_ACTION)
                            }),
                        ])
                    }),
                    html!("home-why-ji-item", {
                        .property("kind", "community")
                        .children(&mut [
                            html!("button-rect", {
                                .property("kind", "text")
                                .property("color", "blue")
                                .property("size", "small")
                                .property("weight", "normal")
                                .property("href", "javascript:alert(\"Coming soon\")")
                                .text(STR_COMMUNITY_ACTION)
                            }),
                        ])
                    }),
                    html!("home-why-ji-item", {
                        .property("kind", "classroom")
                        .children(&mut [
                            html!("button-rect", {
                                .property("kind", "text")
                                .property("color", "blue")
                                .property("size", "small")
                                .property("weight", "normal")
                                .property("href", "javascript:alert(\"Coming soon\")")
                                .text(STR_CLASSROOM_ACTION)
                            }),
                        ])
                    }),
                ])
            }),
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
            html!("img-ui", {
                .property("slot", "image")
                .property("path", format!("entry/home/testimonials/{}", &testimonial.image_id))
                .property("size", "original")
            }),
            html!("h4", {
                .property("slot", "name")
                .text(&testimonial.name)
            }),
            html!("h6", {
                .property("slot", "bio")
                .text(&testimonial.bio)
            }),
            html!("p", {
                .property("slot", "paragraph")
                .text(&testimonial.paragraph)
            }),
        ])
    })
}
