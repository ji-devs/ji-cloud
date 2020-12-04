use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::{SearchQuery, SearchResponse, ImageId, GetResponse},
    error::image::*,
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};
use utils::{
    fetch::{api_with_auth, api_with_auth_empty, api_upload_file},
    path
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
use super::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use itertools::Itertools;
use super::data::*;

pub trait SearchQueryExt {
    fn simple(q:String, is_published: Option<bool>) -> SearchQuery {
        SearchQuery {
            q,
            page: None,
            is_published,
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            categories: Vec::new(),
            is_premium: None,
        }
    }
}

impl SearchQueryExt for SearchQuery {}

pub struct ImageSearchWidget <F>
where F: FnMut(MetaImage)
{
    results: MutableVec<MetaImage>,
    loader: AsyncLoader,
    is_published: Option<bool>,
    on_select: Option<RefCell<F>>,
}

pub struct ImageSearchWidgetDebug {
    pub results:Option<Vec<MetaImage>>, 
    pub is_published: Option<Option<bool>>
}

const DEBUG_STICKER_ID:&'static str = "b0f20a28-2e9b-11eb-9af8-176e032a6567";

impl ImageSearchWidgetDebug {
    pub fn new() -> Self {
        Self {
            //results: None,
            is_published: Some(None),

            results: Some(vec![
                MetaImage::new_debug(
                    DEBUG_STICKER_ID,
                    "bar",
                    MediaLibraryKind::Global
                )
            ]),
        }
    }
}

impl <F> ImageSearchWidget<F> 
where F: FnMut(MetaImage) + 'static
{
    pub fn new(debug:Option<ImageSearchWidgetDebug>, on_select: Option<F>) -> Rc<Self> {

        let results = MutableVec::new();
        //TODO - change to this when ticket is fixed:
        //let mut is_published = Some(true);
        let mut is_published = None; 

        if let Some(debug) = debug {
            if let Some(x) = debug.is_published {
                is_published = x;
            }
            if let Some(x) = debug.results {
                results.lock_mut().replace_cloned(x);
            }
        };

        let _self = Rc::new(Self { 
            results,
            loader: AsyncLoader::new(),
            is_published,
            on_select: on_select.map(|x| RefCell::new(x))
        });

        _self
    }

    fn search(_self: Rc<Self>, query:SearchQuery) {
        _self.clone().loader.load(async move {
            match search_images(query).await {
                Ok((results, pages, total_count)) => {
                    _self.results.lock_mut().replace_cloned(results);
                },
                Err(_) => {
                    log::error!("uhoh... couldn't search!");
                }
            }
        });
    }

    fn search_results_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.results
            .signal_vec_cloned()
            .map(move |item| {
                let id = item.id_str().to_string();
                elem!(templates::search_result_thumbnail(&item), {
                    .event(clone!(_self, item => move |evt:events::Click| {
                        if let Some(cb) = _self.on_select.as_ref() {
                            (cb.borrow_mut())(item.clone());
                        }
                    }))
                    .event(move |evt:events::DragStart| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            data_transfer.set_data(SEARCH_THUMBNAIL_DATA_TRANSFER, &id);
                            data_transfer.set_drop_effect("all");
                        } else {
                            log::error!("no data transfer - use a real computer!!!");
                        }
                    })
                })
            })
    }
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::search_widget(), { 
            .with_data_id!("search", {
                .with_data_id!("query" => HtmlInputElement, {
                    .with_node!(input => {
                        .event(clone!(_self => move |evt:events::KeyDown| {
                            if evt.key() == "Enter" {
                                Self::search(_self.clone(), SearchQuery::simple(input.value(), _self.is_published));
                            } 
                        }))
                    })
                })
                .with_data_id!("items", {
                    .children_signal_vec(Self::search_results_dom_signal(_self.clone()))
                })
            })
        })
    }
}


pub type PageCount = u32;
pub type TotalCount = u64;

pub async fn search_images(query: SearchQuery) -> Result<(Vec<MetaImage>, PageCount, TotalCount), ()> {
    api_with_auth::<SearchResponse, SearchError, _>(Search::PATH, Search::METHOD, Some(query)).await
        .map_err(|err:SearchError| { 
            ()
        })
        .map(|res| {
            let SearchResponse { images, pages, total_image_count } = res;
            let images:Vec<MetaImage> = images
                .into_iter()
                .map(|resp| {
                    let image:MetaImage = (resp.metadata, MediaLibraryKind::Global).into();
                    image
                })
                .collect();

            (images, pages, total_image_count)
        })
}
