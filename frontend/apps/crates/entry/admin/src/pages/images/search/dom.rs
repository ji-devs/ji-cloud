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
use utils::{
    routes::{Route, AdminRoute},
    components::image::{
        data::*,
        search::search_images,
    }
};

use shared::domain::{
    user::UserProfile,
    category::Category,
    image::{SearchQuery, GetResponse},
};


pub struct ImageSearchDom {
    prev_query_text:RefCell<Option<String>>,
    query_input:RefCell<Option<HtmlInputElement>>,
    page_input:RefCell<Option<HtmlInputElement>>,
    is_published_input:RefCell<Option<HtmlSelectElement>>,
    query_text_display: Mutable<String>,
    state: Mutable<SearchState>,
    max_page: Mutable<u32>,
    total_count: Mutable<u64>,
    error: Mutable<Option<String>>,
    results: MutableVec<MetaImage>,
    loader: AsyncLoader,
    serialized_query: RefCell<Option<SearchQuery>>,
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

impl ImageSearchDom {
    pub fn new(initial_query:Option<SearchQuery>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            prev_query_text: RefCell::new(None),
            query_input: RefCell::new(None),
            page_input: RefCell::new(None),
            is_published_input: RefCell::new(None),
            query_text_display: Mutable::new("".to_string()),
            error: Mutable::new(None),
            state: Mutable::new(SearchState::None),
            max_page: Mutable::new(0),
            total_count: Mutable::new(0),
            results: MutableVec::new(),
            loader: AsyncLoader::new(),
            serialized_query: RefCell::new(initial_query),
        });


        _self
    }

    fn get_is_published(&self) -> Option<bool> {
        let value = self.is_published_input
            .borrow()
            .as_ref()
            .map(|input| {
                let x:String = input.value();
                x.parse::<u32>().ok()
            })
            .flatten()
            .unwrap_or(0);

        match value {
            0 => None,
            1 => Some(true),
            2 => Some(false),
            _ => panic!("unknown publish filter!")
        }
    }


    fn get_raw_input_page(&self) -> u32 {
        self.page_input
            .borrow()
            .as_ref()
            .map(|input| {
                let page:String = input.value();
                page.parse::<u32>().ok()
            })
            .flatten()
            .unwrap_or(1)
    }

    fn get_sanitized_input_page(&self) -> u32 {
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
                log::info!("value: {}, max: {}", value, max_page);

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
        let page = _self.get_raw_input_page();

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

            Self::do_search(_self, true);
        }
    }

    pub fn do_search(_self: Rc<Self>, sanitize_page:bool) {

        _self.state.set(SearchState::Loading);
        _self.error.set(None);

        let query_text = {

            let input = _self.query_input.borrow();
            match input.as_ref() {
                Some(input) => input.value(),
                None => "".to_string()
            }
        };

        if let Some(prev_query_text) = _self.prev_query_text.borrow().as_ref() {
            if prev_query_text != &query_text {
                log::info!("query is changed, reset to page 1");
                _self.page_input
                    .borrow_mut()
                    .as_ref()
                    .unwrap_throw()
                    .set_value(&format!("{}", 1));
            }
        }
        *_self.query_text_display.lock_mut() = query_text.clone();
        *_self.prev_query_text.borrow_mut() = Some(query_text.clone());

        let mut page = if sanitize_page {
            _self.get_sanitized_input_page()
        } else {
            _self.get_raw_input_page()
        };

        page -= 1;

        let query = SearchQuery {
            q: query_text,
            page: Some(page),
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            categories: Vec::new(),
            is_premium: None,
            is_published: _self.get_is_published()
        };

        *_self.serialized_query.borrow_mut() = Some(query.clone());

        let route = Route::Admin(AdminRoute::ImageSearch(Some(query.clone())));
        route.replace_state();

        _self.clone().loader.load(async move {
            let results = search_images(query).await; 
            match results {
                Ok((images, max_page, total_count)) => {
                    _self.results.lock_mut().replace_cloned(images);
                    *_self.max_page.lock_mut() = max_page;
                    *_self.total_count.lock_mut() = total_count;
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
                .with_data_id!("n-results", {
                    .text_signal(_self.total_count.signal().map(|count| format!("{}", count)))
                })
                .with_data_id!("query-display", {
                    .text_signal(_self.query_text_display.signal_cloned())
                })
                .with_data_id!("grid", {
                    .children_signal_vec(_self.results.signal_vec_cloned().map(clone!(_self => move |img| {
                        let el = if img.is_published() {
                            templates::image_grid_item_green(&img.thumbnail_src(), &img.meta.name)
                        } else {
                            templates::image_grid_item_red(&img.thumbnail_src(), &img.meta.name)
                        };

                        let route:String = Route::Admin(AdminRoute::ImageEdit(img.id_string(), _self.serialized_query.borrow().clone())).into();
                        html!("a", {
                            .property("href", route)
                            .child(elem!(el, {}))
                        })
                    })))
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
                .apply_if(_self.serialized_query.borrow().is_some(), |dom| {
                    dom.property("value", _self.serialized_query.borrow().as_ref().map(|query| {
                        query.page
                            .map(|page| format!("{}", page+1))
                            .unwrap_or("1".to_string())
                    }))
                })
                .with_node!(input => {
                    .event(clone!(_self => move |evt:events::Change| {
                        Self::do_search(_self.clone(), true);
                    }))
                })
                .after_inserted(clone!(_self => move |elem| {
                    *_self.page_input.borrow_mut() = Some(elem.unchecked_into()); 
                }))
            })

            .with_data_id!("publish-filter" => HtmlSelectElement, {
                .apply_if(_self.serialized_query.borrow().is_some(), |dom| {
                    dom.property("value", _self.serialized_query.borrow().as_ref().map(|query| {
                        match query.is_published {
                            None => "0",
                            Some(flag) => if flag { "1" } else { "2" }
                        }
                    }))
                })
                .event(clone!(_self => move |evt:events::Change| {
                    Self::do_search(_self.clone(), true);
                }))
                .after_inserted(clone!(_self => move |elem| {
                    *_self.is_published_input.borrow_mut() = Some(elem.unchecked_into()); 
                }))
            })

            .with_data_id!("max-page", {
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
                .event(clone!(_self => move |evt:events::Click| {
                    Self::do_search(_self.clone(), true);
                }))
            })
            .with_data_id!("query", {
                .apply_if(_self.serialized_query.borrow().is_some(), |dom| {
                    dom.property("value", _self.serialized_query.borrow().as_ref().map(|query| {
                        query.q.to_string()
                    }))
                })
                .event(clone!(_self => move |evt:events::KeyDown| {
                    if evt.key() == "Enter" {
                        Self::do_search(_self.clone(), true);
                    } 
                }))
                .after_inserted(clone!(_self => move |elem| {
                    *_self.query_input.borrow_mut() = Some(elem.unchecked_into()); 
                }))
            })
            .after_inserted(clone!(_self => move |elem| {
                Self::do_search(_self, false);
            }))
        })

    }
}
