use core::routes::{Route, AdminRoute};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::pages::{
    categories::CategoriesPage,
    images::{ImagesPage, PageMode},
};

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal()
            .map(|route| {
                match route {
                    Route::Admin(route) => {
                        match route {
                            AdminRoute::Categories=> Some(CategoriesPage::render(CategoriesPage::new())),
                            AdminRoute::ImageAdd => Some(ImagesPage::render(ImagesPage::new(PageMode::Add))),
                            AdminRoute::ImageEdit(id) => Some(ImagesPage::render(ImagesPage::new(PageMode::Edit(id)))),
                            _ => None
                        }
                    }
                    _ => None
                }
            })
    }
    
    pub fn render(&self) -> Dom {
        html!("main", { .child_signal(Self::dom_signal()) } )
    }
}
