use dominator::{html, clone, Dom};
use shared::domain::image::{ImageId, ImageSearchQuery};
use std::rc::Rc;
use std::cell::RefCell;
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use utils::{routes::*, events};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use super::{state::*, actions};

pub struct ImageSearchPage {
}

impl ImageSearchPage {
    pub fn render(query: Option<ImageSearchQuery>) -> Dom {
        let state:Rc<State> = Rc::new(query.into());

        html!("empty-fragment", {

            .child(html!("image-search", {
                .future(state.query.signal_cloned().for_each(clone!(state => move |query| {
                    actions::search(state.clone(), query);
                    async {}
                })))
                .property_signal("query", state.query_string_signal())
                .property_signal("nResults", state.n_results_signal())
                .event(clone!(state => move |evt:events::CustomSearch| {
                    let mut query = state.query.lock_mut();
                    query.q = evt.query();
                }))
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
                .child(PaginationDom::render(state.clone(), "pagination-top"))
                .child(FilterDom::render(state.clone(), "publish-filter"))
                .child(PaginationDom::render(state.clone(), "pagination-bottom"))
                .children_signal_vec(state.images_signal_vec().map(|image| {
                    let id = &image.id;
                    html!("a", {
                        .property("slot", "images")
                        .property("href", {
                            let route:String = Route::Admin(AdminRoute::ImageMeta(*id, false)).into();
                            route
                        })
                        .child(html!("search-image-cell", {
                            .property("name", image.name)
                            .property("mode", {
                                //Technically it could be only set to be published in the future
                                //so comparing to now would be more correct
                                //but we aren't using that functionality
                                //and also for admin purposes "set to be published" might be
                                //more correct to have the visual indicator for "published"
                                if image.publish_at.is_some() {
                                    "published"
                                } else {
                                    "saved"
                                }
                            })
                            .child(html!("img-ji", {
                                .property("slot", "image")
                                .property("size", "thumb")
                                .property("cacheBust", true)
                                .property("id", id.0.to_string())
                            }))
                        }))
                    })
                }))
            }))
            .child(html!("window-loader-block", {
                .property_signal("visible", state.loader.is_loading())
            }))
        })
    }
}

struct PaginationDom {
}

impl PaginationDom {
    pub fn render(state: Rc<State>, slot:&str) -> Dom {
        html!("pagination-widget", {
            .property_signal("page", state.page_signal())
            .property_signal("total", state.total_page_signal())
            .property("slot", slot)
            .event(clone!(state => move |evt:events::CustomChange| {
                let page:u32 = evt.value().parse().unwrap();
                let mut query = state.query.lock_mut();
                query.page = Some(page - 1);
            }))
        })
    }
}

struct FilterDom {
    pub elem_ref:RefCell<Option<HtmlElement>>
}

impl FilterDom {
    pub fn close_menu(&self) {
        unsafe {
            js_sys::Reflect::set(
                self.elem_ref.borrow().as_ref().unwrap(), 
                &JsValue::from_str("open"), 
                &JsValue::from_bool(false)
            );
        }
    }
    pub fn render(state: Rc<State>, slot:&str) -> Dom {
        let _self = Rc::new(
            Self {
                elem_ref: RefCell::new(None)
            }
        );

        html!("dropdown-underlined", {
            .property("slot", slot)
            .property_signal("value", state.filter_value_signal())
            .children(&mut [
                html!("image-search-publish-filter", {
                    .property("slot", "options")
                    .property("mode", "all")
                    .event(clone!(state, _self => move |evt:events::Click| {
                        let mut query = state.query.lock_mut();
                        query.is_published = None;
                        _self.close_menu();
                    }))
                }),
                html!("image-search-publish-filter", {
                    .property("slot", "options")
                    .property("mode", "published")
                    .event(clone!(state, _self => move |evt:events::Click| {
                        let mut query = state.query.lock_mut();
                        query.is_published = Some(true);
                        _self.close_menu();
                    }))
                }),
                html!("image-search-publish-filter", {
                    .property("slot", "options")
                    .property("mode", "saved")
                    .event(clone!(state, _self => move |evt:events::Click| {
                        let mut query = state.query.lock_mut();
                        query.is_published = Some(false);
                        _self.close_menu();
                    }))
                }),
            ])
            .after_inserted(clone!(_self => move |dom| {
                *_self.elem_ref.borrow_mut() = Some(dom);
            }))
        })
    }
}
