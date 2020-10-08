use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlSelectElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, AdminRoute};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::GetResponse,
};

use super::actions::*;

pub struct ImageSearch {
    query_input:RefCell<Option<HtmlInputElement>>,
    page_input:RefCell<Option<HtmlInputElement>>,
    is_published:RefCell<Option<bool>>,
    query: Mutable<String>,
    state: Mutable<SearchState>,
    pages: Mutable<u32>,
    error: Mutable<Option<String>>,
    results: MutableVec<BasicImage>,
    loader: AsyncLoader,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SearchState {
    None,
    Loading,
    Results,
}


impl ImageSearch {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            query_input: RefCell::new(None),
            page_input: RefCell::new(None),
            is_published: RefCell::new(None),
            query: Mutable::new("".to_string()),
            error: Mutable::new(None),
            state: Mutable::new(SearchState::None),
            pages: Mutable::new(0),
            results: MutableVec::new(),
            loader: AsyncLoader::new(),
        });

        Self::do_search(_self.clone());

        _self
    }
   
    pub fn do_search(_self: Rc<Self>) {
        _self.state.set(SearchState::Loading);
        _self.error.set(None);

        let query = {
            let query = _self.query_input.borrow();
            match query.as_ref() {
                Some(input) => input.value(),
                None => "".to_string()
            }
        };

        let page = {
            let page_input = _self.page_input.borrow();
            let page:String = page_input.as_ref().unwrap_throw().value();
            let page:u32 = page.parse().unwrap_throw();
            Some(page)
        };
        let is_published = {
            let is_published = _self.is_published.borrow();
            *is_published
        };

        _self.query.set(query.clone());

        _self.clone().loader.load(async move {
            let results = search_images(query, page, is_published).await; 
            match results {
                Ok((images, pages)) => {
                    _self.results.lock_mut().replace_cloned(images);
                    *_self.pages.lock_mut() = pages;
                },
                Err(_) => _self.error.set(Some("got an error!".to_string())),
            }
            _self.state.set(SearchState::Results);
        });
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::images_search(), {
            .with_data_id!("results", {
                .class_signal("hidden", _self.state.signal().map(|state| state != SearchState::Results))
                .with_data_id!("query-display", {
                    .text_signal(_self.query.signal_cloned())
                })
                .with_data_id!("grid", {
                    .children_signal_vec(_self.results.signal_vec_cloned().map(|img| {
                        let el = if img.is_published {
                            templates::image_grid_item_green(&img.src, &img.text)
                        } else {
                            templates::image_grid_item_red(&img.src, &img.text)
                        };

                        let route:String = Route::Admin(AdminRoute::ImageEdit(img.id)).into();
                        html!("a", {
                            .property("href", route)
                            .child(elem!(el, {}))
                        })
                    }))
                })
            })
            .with_data_id!("page", {
                .event(clone!(_self => move |evt:events::Change| {
                    //TODO - re-trigger search?
                }))
                .after_inserted(clone!(_self => move |elem| {
                    *_self.page_input.borrow_mut() = Some(elem.unchecked_into()); 
                }))
            })

            //TODO - need https://github.com/dakom/dominator-helpers/issues/1#issue-717271232
            .with_data_id!("filter" => HtmlSelectElement, {
                .with_node!(element => {
                    .event(clone!(_self => move |evt:events::Change| {
                        let value = element.value();
                        //TODO - change is_published
                        log::info!("{}", value);
                    }))
                })
            })
            .with_data_id!("pages", {
                .text_signal(_self.pages.signal().map(|pages| format!("{}", pages)))
            })
            .with_data_id!("error-message", {
                .class_signal("hidden", _self.error.signal_ref(|err| err.is_none()))
                .text_signal(_self.error.signal_ref(|err| match err {
                    None => "".to_string(),
                    Some(err) => err.to_string()
                }))
            })
            .with_data_id!("loading", {
                .class_signal("hidden", _self.state.signal().map(|state| state != SearchState::Loading))
            })
            .with_data_id!("search-btn", {
                .event(clone!(_self => move |evt:events::KeyDown| {
                    if evt.key() == "Enter" {
                        Self::do_search(_self.clone());
                    } 
                }))
            })
            .with_data_id!("query", {
                .event(clone!(_self => move |evt:events::KeyDown| {
                    if evt.key() == "Enter" {
                        Self::do_search(_self.clone());
                    } 
                }))
                .after_inserted(clone!(_self => move |elem| {
                    *_self.query_input.borrow_mut() = Some(elem.unchecked_into()); 
                }))
            })
        })
        /*
        elem!(templates::image_add(), {
            .with_data_id!("add-btn", {
                .event(clone!(_self => move |_evt:events::Click| {
                    if let Some(file_input) = _self.file_input.borrow().as_ref() {
                        file_input.click();
                    }
                }))
            })
            .with_data_id!("file", {
                .event(clone!(_self => move |_evt:events::Change| {
                    let file =
                        _self.file_input.borrow().as_ref()
                            .and_then(|input| input.files())
                            .and_then(|files| files.get(0));

                    if let Some(file) = file {
                        spawn_local(async move {
                            let id = actions::create_image(file).await.unwrap_throw();
                            let route:String = Route::Admin(AdminRoute::ImageEdit(id)).into();
                            dominator::routing::go_to_url(&route);
                        });
                    }
                }))

            })
            .after_inserted(clone!(_self => move |elem| {
                *_self.file_input.borrow_mut() = Some(elem.select(&data_id("file")));
            }))
        })
        */
    }
}
