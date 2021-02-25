use utils::routes::{Route, AdminRoute};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::{
    categories::dom::CategoriesPage,
    locale::dom::LocalePage,
    images::{
        add::dom::ImageAddPage,
        meta::dom::ImageMetaPage,
        search::dom::ImageSearchPage
    },
};

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }

    fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
            Self::route_signal()
                .map(|route| {
                    match route {
                        Route::Admin(route) => {
                            match route {
                                AdminRoute::Categories=> Some(CategoriesPage::render()),
                                AdminRoute::Locale => Some(LocalePage::render()),
                                AdminRoute::ImageAdd => Some(ImageAddPage::render()),
                                AdminRoute::ImageMeta(id, query) => Some(ImageMetaPage::render(id, query)),
                                AdminRoute::ImageSearch(query) => Some(ImageSearchPage::render(query)),
                                _ => None
                            }
                        }
                        _ => None
                    }
                })
    }

    pub fn render(&self) -> Dom {
        html!("empty-fragment", { .child_signal(Self::dom_signal()) } )
    }
}
