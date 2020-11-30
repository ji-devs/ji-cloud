use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::{SearchQuery, SearchResponse, ImageId, GetResponse},
    error::image::*,
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};
use crate::{
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
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct LibraryImage {
    pub id: String,
    pub raw_id: ImageId,
    pub name: String,
    pub library_kind: MediaLibraryKind,
}

impl LibraryImage {
    pub fn from_string(id: String, name: String, library_kind: MediaLibraryKind) -> Self {

        let raw_id = uuid::Uuid::parse_str(&id).unwrap_throw();
        let raw_id = ImageId(raw_id);

        Self {
            id,
            raw_id,
            name,
            library_kind
        }
    }

    fn from_response(resp:GetResponse, library_kind:MediaLibraryKind) -> Self {
        let raw_id = resp.metadata.id;

        let id = raw_id.0.to_string();

        Self {
            id,
            name: resp.metadata.name,
            raw_id,
            library_kind,
        }
    }

    pub fn thumbnail_src(&self) -> String {
        path::library_image_id(self.library_kind, MediaVariant::Thumbnail, self.raw_id)
    }
}

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

pub struct ImageSearchWidget {
    results: MutableVec<LibraryImage>,
    loader: AsyncLoader,
    is_published: Option<bool>
}

impl ImageSearchWidget {
    pub fn new() -> Rc<Self> {

        let _self = Rc::new(Self { 
            results: MutableVec::new(),
            loader: AsyncLoader::new(),
            is_published: Some(true)
        });

        _self
    }
 
    pub fn new_debug(init_results:Option<Vec<LibraryImage>>, is_published: Option<Option<bool>>) -> Rc<Self> {
        let is_published = match is_published {
            None => Some(true),
            Some(is_published) => is_published
        };

        let results = MutableVec::new();
        if let Some(init) = init_results {
            results.lock_mut().replace_cloned(init);
        }

        let _self = Rc::new(Self { 
            results,
            loader: AsyncLoader::new(),
            is_published,
        });

        _self
    }

    fn search(_self: Rc<Self>, query:SearchQuery) {
        _self.clone().loader.load(async move {
            match search_images(query).await {
                Ok((results, pages)) => {
                    _self.results.lock_mut().replace_cloned(results);
                },
                Err(_) => {
                    log::error!("uhoh... couldn't search!");
                }
            }
        });
    }

    fn search_results_dom_signal(&self) -> impl SignalVec<Item = Dom> {
        self.results
            .signal_vec_cloned()
            .map(|item| {
                let id = item.id.to_string();
                elem!(templates::image_search_result_thumbnail(&item), {
                    .event(move |evt:events::DragStart| {
                        if let Some(data_transfer) = evt.data_transfer() {
                            data_transfer.set_data("search-image-result", &id);
                            data_transfer.set_drop_effect("all");
                        } else {
                            log::error!("no data transfer - use a real computer!!!");
                        }
                    })
                })
            })
    }
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::image_search_widget(), { 
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
                    .children_signal_vec(_self.search_results_dom_signal())
                })
            })
        })
    }
}



async fn search_images(query: SearchQuery) -> Result<(Vec<LibraryImage>, u32), ()> {
    api_with_auth::<SearchResponse, SearchError, _>(Search::PATH, Search::METHOD, Some(query)).await
        .map_err(|err:SearchError| { 
            ()
        })
        .map(|res| {
            let SearchResponse { images, pages } = res;
            let images:Vec<LibraryImage> = images
                .into_iter()
                .map(|res| LibraryImage::from_response(res, MediaLibraryKind::Global))
                .collect();

            (images, pages)
        })
}
