use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use shared::domain::image::{ImageSearchQuery, ImageSearchResponse, ImageMetadata};
use dominator_helpers::futures::AsyncLoader;
use crate::strings;

pub struct State {
    pub query: Mutable<ImageSearchQuery>,
    pub response: Mutable<Option<ImageSearchResponse>>,
    pub loader: AsyncLoader,
}

impl From<Option<ImageSearchQuery>> for State {
    fn from(init:Option<ImageSearchQuery>) -> Self {
        Self {
            query: Mutable::new(init.unwrap_or_default()),
            response: Mutable::new(None),
            loader: AsyncLoader::new()
        }
    }
}

impl State {
    pub fn query_string_signal(&self) -> impl Signal<Item = String> {
        self.query.signal_ref(|query| query.q.clone())
    }

    pub fn images_signal_vec(&self) -> impl SignalVec<Item = ImageMetadata> {
        self.response
            .signal_cloned()
            .map(|resp| {
                match resp {
                    None => Vec::new(),
                    Some(resp) => {
                        resp.images
                            .into_iter()
                            .map(|img| img.metadata)
                            .collect()
                    }
                }
            })
            .to_signal_vec()
    }

    pub fn filter_value_signal(&self) -> impl Signal<Item = &'static str> {
        self.query
            .signal_ref(|query| {
                match query.is_published {
                    None => strings::STR_FILTER_SHOW_ALL,
                    Some(is_published) => {
                        if is_published {
                            strings::STR_FILTER_PUBLISHED
                        } else {
                            strings::STR_FILTER_SAVED
                        }
                    }
                }
            })
    }

    pub fn page_signal(&self) -> impl Signal<Item = u32> {
        self.query.signal_ref(|query| query.page.unwrap_or_default() + 1)
    }
    pub fn total_page_signal(&self) -> impl Signal<Item = u32> {
        self.response.signal_ref(|resp| match resp {
            None => 0,
            Some(resp) => resp.pages
        })
    }
    pub fn n_results_signal(&self) -> impl Signal<Item = f64> {
        self.response.signal_ref(|resp| match resp {
            None => 0.0,
            Some(resp) => resp.total_image_count as f64
        })
    }
}
