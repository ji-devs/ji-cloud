use utils::routes::{Route, AdminRoute};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::pages::{
    container::ContainerPage,
    categories::CategoriesPage,
    images::{ImagesPage, PageMode},
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

    fn signal_dom() -> impl Signal<Item = Option<Dom>> {
            Self::route_signal()
                .map(|route| {
                    match route {
                        Route::Admin(route) => {
                            match route {
                                AdminRoute::Categories=> Some(CategoriesPage::render(CategoriesPage::new())),
                                AdminRoute::ImageAdd => Some(ImagesPage::render(ImagesPage::new(PageMode::Add))),
                                AdminRoute::ImageEdit(id, query) => Some(ImagesPage::render(ImagesPage::new(PageMode::Edit(id, query)))),
                                AdminRoute::ImageSearch(query) => Some(ImagesPage::render(ImagesPage::new(PageMode::Search(query)))),
                            }
                        }
                        _ => None
                    }
                })
    }

    pub fn render(&self) -> Dom {
        ContainerPage::render(ContainerPage::new(),
            Self::signal_dom(),
            Self::route_signal()
        )
    }
}
