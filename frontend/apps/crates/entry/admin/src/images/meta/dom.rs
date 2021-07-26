use dominator::{html, clone, Dom};
use shared::domain::image::{ImageId, ImageSearchQuery};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use super::{
    actions, 
    state::*, 
    sections::{
        one::dom::Section1Dom,
        two::dom::Section2Dom,
        three::dom::Section3Dom,
    }
};
use utils::{routes::*, events};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

const STR_REPLACE:&'static str ="Replace";
const STR_DELETE:&'static str = "Delete";
const STR_PREMIUM:&'static str ="Premium";
const STR_IMAGENAME:&'static str = "Image name";
const STR_DESCRIPTION:&'static str = "Image description";
const STR_NEXT:&'static str = "Next";
const STR_PUBLISH:&'static str = "Publish";

pub struct ImageMetaPage {
}

impl ImageMetaPage {
    pub fn render(id: ImageId, is_new: bool) -> Dom {
        let state = Rc::new(State::new(id, is_new));
        
        let initial_data = actions::load_initial(state.clone());

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
            .child_signal(
                state.loaded.signal().map(clone!(state, initial_data => move |loaded| {
                    if loaded {
                        let (image, categories, metadata, tag_list) = initial_data.borrow_mut().take().unwrap_throw();

                        Some(html!("image-meta-page", {
                            .event(|evt:events::CustomRoute| {
                                match evt.route().as_ref() {
                                    "add" => {
                                        let route:String = Route::Admin(AdminRoute::ImageAdd).into();
                                        dominator::routing::go_to_url(&route);
                                    },
                                    _ => {
                                    }
                                }
                            })
                            .event(|evt:events::CustomSearch| {
                                let q:String = evt.query();
                                let query = ImageSearchQuery {
                                    q,
                                    page: None, 
                                    styles: Vec::new(),
                                    tags: Vec::new(),
                                    age_ranges: Vec::new(),
                                    affiliations: Vec::new(),
                                    categories: Vec::new(),
                                    is_premium: None,
                                    is_published: None,
                                    kind: None,
                                };
                                let route:String = Route::Admin(AdminRoute::ImageSearch(Some(query))).into();
                                dominator::routing::go_to_url(&route);
                            })
                            .children(&mut [
                                html!("img-ji", {
                                    .property("slot", "image")
                                    .property("size", "thumb")
                                    .property("cacheBust", true)
                                    .property_signal("id", image.id.signal_cloned().map(|id| {
                                        id.0.to_string()
                                    }))
                                }),
                                html!("button-rect", {
                                    .property("slot", "replace")
                                    .property("kind", "text")
                                    .property("color", "blue")
                                    .property("size", "small")
                                    .text(STR_REPLACE)
                                    .event(clone!(state => move |evt:events::Click| {
                                        if let Some(elem) = state.file_input.borrow().as_ref() {
                                            elem.click();
                                        }
                                    }))
                                }),
                                html!("button-rect", {
                                    .property("slot", "delete")
                                    .property("kind", "text")
                                    .property("color", "blue")
                                    .property("size", "small")
                                    .text(STR_DELETE)
                                    .event(clone!(state => move |evt:events::Click| {
                                        state.delete_modal.set_neq(true);
                                    }))
                                }),
                                html!("input-checkbox", {
                                    .property("slot", "premium")
                                    .property("label", STR_PREMIUM)
                                    .property_signal("checked", image.is_premium.signal())
                                    .event(clone!(state, image => move |evt:events::CustomToggle| {
                                        actions::toggle_premium(state.clone(), image.clone(), evt.value());
                                    }))
                                }),
                                html!("input-text-underline", {
                                    .property("slot", "description")
                                    .property("label", STR_IMAGENAME)
                                    .property_signal("value", image.name.signal_cloned())
                                    .event(clone!(state, image => move |evt:events::CustomChange| {
                                        actions::change_name(state.clone(), image.clone(), evt.value());
                                    }))
                                }),
                                html!("input-textarea-underline", {
                                    .property("slot", "description")
                                    .property("label", STR_DESCRIPTION)
                                    .property_signal("value", image.description.signal_cloned())
                                    .event(clone!(state, image => move |evt:events::CustomChange| {
                                        actions::change_description(state.clone(), image.clone(), evt.value());
                                    }))
                                }),
                                html!("button-rect", {
                                    .property("slot", "next")
                                    .property("color", "red")
                                    .property("size", "medium")
                                    .text_signal(state.section.signal().map(|section| {
                                        match section {
                                            Section::One | Section::Two => STR_NEXT,
                                            Section::Three => STR_PUBLISH
                                        }
                                    }))
                                    .event(clone!(state => move |evt:events::Click| {
                                        match state.section.get() {
                                            Section::One => state.section.set(Section::Two), 
                                            Section::Two => state.section.set(Section::Three), 
                                            Section::Three => actions::publish(state.clone())
                                        }
                                    }))
                                }),
                                html!("input" => HtmlInputElement, {
                                    .property("type", "file")
                                    .property("accept", "image/*")
                                    .style("display", "none")
                                    .after_inserted(clone!(state => move |elem| {
                                        *state.file_input.borrow_mut() = Some(elem);
                                    }))
                                    .event(clone!(state, image => move |_evt:events::Change| {
                                        let file =
                                            state.file_input.borrow().as_ref()
                                                .and_then(|input| input.files())
                                                .and_then(|files| files.get(0));

                                        if let Some(file) = file {
                                            actions::on_file(state.clone(), image.clone(), file);
                                        }
                                    }))
                                }),
                                html!("div", {
                                    .property("slot", "right")
                                    .child_signal(state.section.signal().map(clone!(state, image, categories, metadata, tag_list => move |section| {
                                        match section {
                                            Section::One => Some(Section1Dom::render(state.clone(), image.clone(), metadata.clone(), tag_list.clone())),
                                            Section::Two => Some(Section2Dom::render(state.clone(), image.clone(), categories.clone())),
                                            Section::Three => Some(Section3Dom::render(state.clone(), image.clone(), metadata.clone(), categories.clone(), tag_list.clone())),
                                        }
                                    })))
                                }),
                                html!("modal-confirm", {
                                    .property("mode", "deleteImage")
                                    .property_signal("visible", state.delete_modal.signal())
                                    .property("slot", "modal")
                                    .event(clone!(state, image => move |evt:events::CustomToggle| {
                                        state.delete_modal.set_neq(false);
                                        if evt.value() {
                                            actions::delete(state.clone());
                                        }
                                    }))
                                })
                            ])
                        }))
                    } else {
                        None
                    }
                }))
            )
        })
    }
}
