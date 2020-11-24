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
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::routes::{Route, AdminRoute};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::GetResponse,
};

use super::actions::*;

pub struct ImageSearch {
    prev_query:RefCell<Option<String>>,
    query_input:RefCell<Option<HtmlInputElement>>,
    page_input:RefCell<Option<HtmlInputElement>>,
    is_published:RefCell<Option<bool>>,
    query: Mutable<String>,
    state: Mutable<SearchState>,
    max_page: Mutable<u32>,
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

enum PageDelta {
    Back,
    Next
}

impl ImageSearch {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 

            prev_query: RefCell::new(None),
            query_input: RefCell::new(None),
            page_input: RefCell::new(None),
            is_published: RefCell::new(None),
            query: Mutable::new("".to_string()),
            error: Mutable::new(None),
            state: Mutable::new(SearchState::None),
            max_page: Mutable::new(0),
            results: MutableVec::new(),
            loader: AsyncLoader::new(),
        });

        Self::do_search(_self.clone());

        _self
    }
  
    pub fn get_input_page(&self) -> u32 {
        let input = self.page_input.borrow();
        let max_page = self.max_page.get();

        let page:Option<u32> = input
            .as_ref()
            .map(|input| {
                let page:String = input.value();
                page.parse::<u32>().ok()
            })
            .flatten();

        let (page_num, set_input_value) = match page {
            Some(value) => {
                if value < 1 {
                    (1, true) 
                } else if value > max_page {
                    let value = if max_page > 0 { max_page } else { 1 };
                    (value, true)
                } else {
                    (value, false)
                }
            },
            None => {
                (1, true)
            }
        };

        if set_input_value {
            if let Some(input) = input.as_ref() {
                input.set_value(&format!("{}", page_num));
            }
        }

        page_num
    }

    fn change_page(_self: Rc<Self>, delta:PageDelta) {
        let page = _self.get_input_page();
        let next_page = match delta {
            PageDelta::Back => {
                if page > 1 {
                    Some(page - 1)
                } else {
                    None
                }
            },
            PageDelta::Next => {
                if page < _self.max_page.get() {
                    Some(page + 1)
                } else {
                    None
                }
            }
        };

        if let Some(next_page) = next_page {
            _self.page_input
                .borrow()
                .as_ref()
                .unwrap_throw()
                .set_value(&format!("{}", next_page));

            Self::do_search(_self);
        }
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

        if let Some(prev_query) = _self.prev_query.borrow().as_ref() {
            if prev_query == &query {
                _self.page_input
                    .borrow_mut()
                    .as_ref()
                    .unwrap_throw()
                    .set_value(&format!("{}", 1));
            }
        }
        *_self.prev_query.borrow_mut() = Some(query.clone());

        let page = _self.get_input_page() - 1;

        let is_published = {
            let is_published = _self.is_published.borrow();
            *is_published
        };

        _self.query.set(query.clone());

        _self.clone().loader.load(async move {
            let results = search_images(query, Some(page), is_published).await; 
            match results {
                Ok((images, max_page)) => {
                    _self.results.lock_mut().replace_cloned(images);
                    *_self.max_page.lock_mut() = max_page;
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

            .with_data_id!("pages-nav", {
                .with_data_id!("back-btn", {
                    .event(clone!(_self => move |evt:events::Click| {
                        Self::change_page(_self.clone(), PageDelta::Back);
                    }))
                })
                .with_data_id!("next-btn", {
                    .event(clone!(_self => move |evt:events::Click| {
                        Self::change_page(_self.clone(), PageDelta::Next);
                    }))
                })
                .class_signal("hidden", _self.max_page.signal_ref(|max_page| {
                    *max_page == 0
                }))
            })

            .with_data_id!("page" => HtmlInputElement, {
                .with_node!(input => {
                    .event(clone!(_self => move |evt:events::Change| {
                        Self::do_search(_self.clone());
                    }))
                })
                .after_inserted(clone!(_self => move |elem| {
                    *_self.page_input.borrow_mut() = Some(elem.unchecked_into()); 
                }))
            })

            .with_data_id!("filter" => HtmlSelectElement, {
                .with_node!(element => {
                    .event(clone!(_self => move |evt:events::Change| {
                        let value = element.value();
                        *_self.is_published.borrow_mut() = match value.as_ref() {
                            "0" => None,
                            "1" => Some(true),
                            "2" => Some(false),
                            _ => panic!("unsupported filter!"),
                        };
                        Self::do_search(_self.clone());
                    }))
                })
            })

            .with_data_id!("pages", {
                .text_signal(_self.max_page.signal().map(|max_page| format!("{}", max_page)))
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
