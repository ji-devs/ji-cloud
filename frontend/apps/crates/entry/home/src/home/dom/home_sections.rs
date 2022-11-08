// use dominator::{html, Dom};
// use std::rc::Rc;
// use utils::routes::{JigRoute, Route};

// use super::super::{actions, state::Testimonial};

// use super::super::state::State;

// const STR_PLAY: &str = "Play Series";
// // const STR_JIGS: &str = "JIGs";

// const STR_CONTENT_ACTION: &str = "See our library";
// const STR_CREATE_ACTION: &str = "Try it for free";
// const STR_CUSTOMIZE_ACTION: &str = "See our lesson outlines";
// const STR_COMMUNITY_ACTION: &str = "Get inspired";
// const STR_CLASSROOM_ACTION: &str = "Manage your class";

// const STR_COMING_SOON: &str = "(coming soon!)";

// pub fn render(state: Rc<State>) -> Dom {
//     html!("empty-fragment", {
//         .children(&mut [
//             html!("home-quick-search", {
//                 .children(state.quick_searches.iter().map(|item| {
//                     html!("home-quick-search-item", {
//                         .prop("href", actions::search_url(&item.search_term))
//                         .children(&mut [
//                             html!("img-ui", {
//                                 .prop("slot", "image")
//                                 .prop("path", {
//                                     format!("entry/home/quick-search/{}.svg", &item.search_term.to_lowercase().replace(" ", "-"))
//                                 })
//                             }),
//                             html!("h4", {
//                                 .prop("slot", "title")
//                                 .text(&item.search_term)
//                             }),
//                             // html!("h5", {
//                             //     .prop("slot", "subtitle")
//                             //     .text(&item.jigs_count.to_string())
//                             //     .text(STR_JIGS)
//                             // }),
//                         ])
//                     })
//                 }))
//             }),
//             html!("home-create"),
//             html!("home-why-ji", {
//                 .children(&mut [
//                     html!("home-why-ji-item", {
//                         .prop("kind", "content")
//                         .children(&mut [
//                             html!("button-rect", {
//                                 .prop("kind", "text")
//                                 .prop("color", "blue")
//                                 .prop("size", "small")
//                                 .prop("weight", "normal")
//                                 .prop("href", actions::search_url(""))
//                                 .text(STR_CONTENT_ACTION)
//                             }),
//                         ])
//                     }),
//                     html!("home-why-ji-item", {
//                         .prop("kind", "create")
//                         .children(&mut [
//                             html!("button-rect", {
//                                 .prop("kind", "text")
//                                 .prop("color", "blue")
//                                 .prop("size", "small")
//                                 .prop("weight", "normal")
//                                 .prop("href", &Route::Jig(JigRoute::Gallery).to_string())
//                                 .text(STR_CREATE_ACTION)
//                             }),
//                         ])
//                     }),
//                     html!("home-why-ji-item", {
//                         .prop("kind", "customize")
//                         .children(&mut [
//                             html!("button-rect", {
//                                 .prop("kind", "text")
//                                 .prop("color", "blue")
//                                 .prop("size", "small")
//                                 .prop("weight", "normal")
//                                 .prop("href", &Route::Jig(JigRoute::Gallery).to_string())
//                                 .text(STR_CUSTOMIZE_ACTION)
//                             }),
//                         ])
//                     }),
//                     html!("home-why-ji-item", {
//                         .prop("kind", "community")
//                         .children(&mut [
//                             html!("button-rect", {
//                                 .prop("kind", "text")
//                                 .prop("color", "blue")
//                                 .prop("size", "small")
//                                 .prop("weight", "normal")
//                                 .text(STR_COMMUNITY_ACTION)
//                                 .child(html!("br"))
//                                 .text(STR_COMING_SOON)
//                             }),
//                         ])
//                     }),
//                     html!("home-why-ji-item", {
//                         .prop("kind", "classroom")
//                         .children(&mut [
//                             html!("button-rect", {
//                                 .prop("kind", "text")
//                                 .prop("color", "blue")
//                                 .prop("size", "small")
//                                 .prop("weight", "normal")
//                                 .text(STR_CLASSROOM_ACTION)
//                                 .child(html!("br"))
//                                 .text(STR_COMING_SOON)
//                             }),
//                         ])
//                     }),
//                 ])
//             }),
//             html!("home-whats-new", {
//                 .visible(false)
//                 .prop("pageCount", state.whats_new.len() as u32)
//                 .children(state.whats_new.iter().map(|item| {
//                     html!("home-new-item", {
//                         .prop("slot", "items")
//                         .children(&mut [
//                             html!("img-ji", {
//                                 .prop("slot", "image")
//                                 .prop("id", &item.image_id)
//                                 .prop("lib", &item.image_lib)
//                                 .prop("size", "original")
//                             }),
//                             html!("h2", {
//                                 .prop("slot", "subtitle")
//                                 .text(&item.header)
//                             }),
//                             html!("p", {
//                                 .prop("slot", "lines")
//                                 .text(&item.paragraph)
//                             }),
//                             html!("button-rect", {
//                                 .prop("slot", "button")
//                                 .prop("size", "large")
//                                 .prop("color", "red")
//                                 .prop("bold", "")
//                                 .text(STR_PLAY)
//                             }),
//                         ])
//                     })
//                 }))
//             }),
//             html!("home-testimonials", {
//                 .prop("teachersPageCount", state.teachers_testimonials.len() as u32)
//                 .prop("parentsPageCount", state.parents_testimonials.len() as u32)
//                 .children(state.teachers_testimonials.iter().map(|t| {
//                     testimonial(t, "teachers")
//                 }))
//                 .children(state.parents_testimonials.iter().map(|t| {
//                     testimonial(t, "parents")
//                 }))
//             }),
//         ])
//     })
// }

// fn testimonial(testimonial: &Testimonial, slot: &str) -> Dom {
//     html!("home-testimonial-item", {
//         .prop("slot", slot)
//         .children(&mut [
//             html!("img-ui", {
//                 .prop("slot", "image")
//                 .prop("path", format!("entry/home/testimonials/{}", &testimonial.image_id))
//                 .prop("size", "original")
//             }),
//             html!("h4", {
//                 .prop("slot", "name")
//                 .text(&testimonial.name)
//             }),
//             html!("h6", {
//                 .prop("slot", "bio")
//                 .text(&testimonial.bio)
//             }),
//             html!("p", {
//                 .prop("slot", "paragraph")
//                 .text(&testimonial.paragraph)
//             }),
//         ])
//     })
// }
