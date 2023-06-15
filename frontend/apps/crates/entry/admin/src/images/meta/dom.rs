use super::{
    actions,
    sections::{
        categories::dom::CategoriesDom, general::dom::GeneralDom, summary::dom::SummaryDom,
    },
    state::*,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::image::{ImageId, ImageSearchQuery};
use std::rc::Rc;
use utils::{events, routes::*, unwrap::UnwrapJiExt};
use web_sys::HtmlInputElement;

const STR_REPLACE: &str = "Replace";
const STR_DELETE: &str = "Delete";
const STR_PREMIUM: &str = "Premium";
const STR_IMAGENAME: &str = "Image name";
const STR_DESCRIPTION: &str = "Image description";
const STR_NEXT: &str = "Next";
const STR_PUBLISH: &str = "Publish";

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this image?";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

pub struct ImageMetaPage {}

impl ImageMetaPage {
    pub fn render(id: ImageId, is_new: bool) -> Dom {
        let state = Rc::new(State::new(id, is_new));

        let initial_data = actions::load_initial(state.clone());

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .prop_signal("visible", state.loader.is_loading())
            }))
            .child_signal(
                state.loaded.signal().map(clone!(state, initial_data => move |loaded| {
                    if loaded {
                        let (image, categories, metadata) = initial_data.borrow_mut().take().unwrap_ji();

                        Some(html!("image-meta-container", {
                            .event(|evt:events::CustomRoute| {
                                if evt.route() == "add" {
                                    let route:String = Route::Admin(AdminRoute::ImageAdd).into();
                                    dominator::routing::go_to_url(&route);
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
                                    tags_priority: Vec::new(),
                                    is_premium: None,
                                    is_published: None,
                                    size: None,
                                    page_limit: None,
                                };
                                let route:String = Route::Admin(AdminRoute::ImageSearch(Some(query))).into();
                                dominator::routing::go_to_url(&route);
                            })
                            .children(&mut [
                                html!("image-meta-header", {
                                    .event(clone!(state => move |evt:events::CustomRoute| {
                                        let route = evt.route();
                                        let route:&str = route.as_ref();

                                        match route {
                                            "publish" => {
                                                actions::publish(state.clone())
                                            },
                                            "add" => {
                                                Route::push_state(Route::Admin(AdminRoute::ImageAdd));
                                            }
                                            _ => { }
                                        }
                                    }))
                                    .prop("slot", "header")
                                    .prop_signal("section", state.section.signal_ref(|x| x.as_str()))
                                }),

                                html!("img-ji", {
                                    .prop("slot", "image")
                                    .prop("size", "thumb")
                                    .prop("cacheBust", true)
                                    .prop_signal("id", image.id.signal_cloned().map(|id| {
                                        id.0.to_string()
                                    }))
                                }),
                                html!("button-rect", {
                                    .prop("slot", "replace")
                                    .prop("kind", "text")
                                    .prop("color", "blue")
                                    .prop("size", "regular")
                                    .text(STR_REPLACE)
                                    .event(clone!(state => move |_evt:events::Click| {
                                        if let Some(elem) = state.file_input.borrow().as_ref() {
                                            elem.click();
                                        }
                                    }))
                                }),
                                html!("button-rect", {
                                    .prop("slot", "delete")
                                    .prop("kind", "text")
                                    .prop("color", "blue")
                                    .prop("size", "regular")
                                    .text(STR_DELETE)
                                    .event(clone!(state => move |_evt:events::Click| {
                                        state.delete_modal.set_neq(true);
                                    }))
                                }),
                                html!("input-checkbox", {
                                    .prop("slot", "premium")
                                    .prop("label", STR_PREMIUM)
                                    .prop_signal("checked", image.is_premium.signal())
                                    .event(clone!(state, image => move |evt:events::CustomToggle| {
                                        actions::toggle_premium(state.clone(), image.clone(), evt.value());
                                    }))
                                }),
                                html!("input-text-underline", {
                                    .prop("slot", "description")
                                    .prop("label", STR_IMAGENAME)
                                    .prop_signal("value", image.name.signal_cloned())
                                    .event(clone!(state, image => move |evt:events::CustomChange| {
                                        actions::change_name(state.clone(), image.clone(), evt.value());
                                    }))
                                }),
                                html!("input-textarea-underline", {
                                    .prop("slot", "description")
                                    .prop("label", STR_DESCRIPTION)
                                    .prop_signal("value", image.description.signal_cloned())
                                    .event(clone!(state, image => move |evt:events::CustomChange| {
                                        actions::change_description(state.clone(), image.clone(), evt.value());
                                    }))
                                }),
                                html!("button-rect", {
                                    .prop("slot", "next")
                                    .prop("color", "red")
                                    .prop("size", "regular")
                                    .text_signal(state.section.signal().map(|section| {
                                        match section {
                                            Section::General | Section::Categories => STR_NEXT,
                                            Section::Summary => STR_PUBLISH
                                        }
                                    }))
                                    .event(clone!(state => move |_evt:events::Click| {
                                        match state.section.get() {
                                            Section::General => state.section.set(Section::Categories),
                                            Section::Categories => state.section.set(Section::Summary),
                                            Section::Summary => actions::publish(state.clone())
                                        }
                                    }))
                                }),
                                html!("input" => HtmlInputElement, {
                                    .prop("type", "file")
                                    .prop("accept", "image/*")
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
                                    .prop("slot", "right")
                                    .child_signal(state.section.signal().map(clone!(state, image, categories, metadata => move |section| {
                                        match section {
                                            Section::General => Some(GeneralDom::render(state.clone(), image.clone(), metadata.clone())),
                                            Section::Categories => Some(CategoriesDom::render(state.clone(), image.clone(), categories.clone())),
                                            Section::Summary => Some(SummaryDom::render(state.clone(), image.clone(), metadata.clone(), categories.clone())),
                                        }
                                    })))
                                }),
                            ])
                            .child_signal(state.delete_modal.signal().map(clone!(state => move |delete_modal| {
                                if delete_modal {
                                    Some(html!("modal-confirm", {
                                        .prop("slot", "modal")
                                        .prop("dangerous", true)
                                        .prop("title", STR_DELETE_TITLE)
                                        .prop("content", STR_DELETE_CONTENT)
                                        .prop("cancel_text", STR_DELETE_CANCEL)
                                        .prop("confirm_text", STR_DELETE_CONFIRM)
                                        .prop("confirmIcon", "core/menus/delete-white.svg")
                                        .event(clone!(state => move |_evt: events::CustomCancel| state.delete_modal.set_neq(false)))
                                        .event(clone!(state => move |_evt: events::CustomConfirm| {
                                            state.delete_modal.set_neq(false);
                                            actions::delete(state.clone());
                                        }))
                                    }))
                                } else {
                                    None
                                }
                            })))
                        }))
                    } else {
                        None
                    }
                }))
            )
        })
    }
}
