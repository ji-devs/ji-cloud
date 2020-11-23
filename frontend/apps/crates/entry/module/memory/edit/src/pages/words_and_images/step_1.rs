use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::*,
    error::image::*
};
use utils::{
    fetch::{api_with_auth, api_with_auth_empty, api_upload_file}
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use url::Url;
use web_sys::File;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, dynamic_class_signal, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::*;
use itertools::Itertools;
use crate::pages::all_modes::{
    steps_nav::apply_steps_nav,
    text_area_widget::apply_text_area_widget,
    card_dom::apply_edit_cards,
};

pub struct Step1Page {
    state: Rc<BaseGameState>,
    content_mode: Mutable<ContentMode>,
    search_results: MutableVec<SearchResultImage>,
    loader: AsyncLoader,
}


impl Step1Page {
    pub fn new(state:Rc<BaseGameState>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            state,
            content_mode: Mutable::new(crate::debug::settings().content_mode),
            search_results: MutableVec::new(),
            loader: AsyncLoader::new(),
        });

        _self
    }
 

    fn do_search(_self: Rc<Self>, query:String) {
        _self.clone().loader.load(async move {
            let results = search_images(query, None, None).await;
            match results {
                Ok(results) => {
                    _self.search_results.lock_mut().replace_cloned(results.0);
                },
                Err(_) => {
                    log::error!("uhoh... couldn't search!");
                }
            }
        });
    }

    fn search_results_dom_signal(&self) -> impl SignalVec<Item = Dom> {
        self.search_results
            .signal_vec_cloned()
            .map(|item| {
                let id = item.id;
                elem!(templates::words_and_images::step_1_thumbnail(&item.src), {
                    .event(move |evt:events::DragStart| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            data_transfer.set_data("card_image", &id);
                            data_transfer.set_drop_effect("all");
                        } else {
                            log::error!("no data transfer - use a real computer!!!");
                        }
                    })
                })
            })
    }
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::words_and_images::step_1_page(), { 
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
            .apply(|dom| apply_edit_cards(dom, _self.state.clone()))
            .with_data_id!("images-text-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.content_mode.set(ContentMode::Text);
                }))
            })
            .with_data_id!("text-images-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.content_mode.set(ContentMode::Images);
                }))
            })
            .with_data_id!("images-widget", {
                .class_signal("hidden", _self.content_mode.signal().map(|content_mode| {
                    if content_mode == ContentMode::Text {
                        true
                    } else {
                        false
                    }
                }))
                .with_data_id!("search-input" => HtmlInputElement, {
                    .with_node!(input => {
                        .event(clone!(_self => move |evt:events::KeyDown| {
                            if evt.key() == "Enter" {
                                Self::do_search(_self.clone(), input.value());
                            } 
                        }))
                    })
                })
                .with_data_id!("search", {
                    .with_data_id!("list", {
                        .children_signal_vec(_self.search_results_dom_signal())
                    })
                })
            })
            .with_data_id!("text-widget", {
                .apply(|dom| apply_text_area_widget(dom, _self.state.clone()))

                .class_signal("hidden", _self.content_mode.signal().map(|content_mode| {
                    if content_mode == ContentMode::Images {
                        true
                    } else {
                        false
                    }
                }))
            })
        })
    }
}


#[derive(Clone, Debug)]
pub struct SearchResultImage {
    pub id: String,
    pub src: String,
    pub is_published: bool,
    pub text: String
}
impl SearchResultImage {
    pub fn new(resp:GetResponse) -> Self {
        Self {
            id: resp.metadata.id.0.to_string(),
            src: resp.thumbnail_url.to_string(),
            is_published: resp.metadata.publish_at.is_some(),
            text: resp.metadata.name
        }
    }
}

pub async fn search_images(query:String, page: Option<u32>, is_published: Option<bool>) -> Result<(Vec<SearchResultImage>, u32), ()> {
    _search_images_api(query, page, is_published).await
        .map_err(|err:SearchError| { 
            ()
        })
        .map(|res| {
            let SearchResponse { images, pages } = res;
            let images:Vec<SearchResultImage> = images
                .into_iter()
                .map(SearchResultImage::new)
                .collect();

            (images, pages)
        })
}


async fn _search_images_api(query:String, page: Option<u32>, is_published: Option<bool>) -> Result < <Search as ApiEndpoint>::Res, <Search as ApiEndpoint>::Err> {
    let req = SearchQuery {
        q: query,
        page,
        is_published,
        //future query powers :)
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new(),
        is_premium: None,
    };

    api_with_auth(Search::PATH, Search::METHOD, Some(req)).await
}
