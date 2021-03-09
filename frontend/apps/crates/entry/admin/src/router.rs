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
    sidebar::dom::SidebarDom,
};

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }
    pub fn render(&self) -> Dom {
        html!("empty_fragment", {
            .child_signal(Self::dom_signal())
        })
    }

    fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
            Self::route_signal()
                .map(|route| {
                    match route {
                        Route::Admin(route_ref) => {
                            let route = route_ref.clone();
                            match route_ref {
                                AdminRoute::Categories=> Some(Self::with_child(route, CategoriesPage::render())),
                                AdminRoute::Locale => Some(Self::with_child(route, LocalePage::render())),
                                AdminRoute::ImageAdd => Some(Self::with_child(route, ImageAddPage::render())),
                                AdminRoute::ImageMeta(id, is_new) => Some(Self::with_child(route, ImageMetaPage::render(id, is_new))),
                                AdminRoute::ImageSearch(query) => Some(Self::with_child(route, ImageSearchPage::render(query))),
                                _ => Some(Self::with_child(route, html!("empty-fragment"))),
                            }
                        }
                        _ => None
                    }
                })
    }

    fn with_child(route: AdminRoute, dom:Dom) -> Dom {
        html!("admin-shell", { 
            .child(SidebarDom::render(route))
            .child(dom)
        })
    }
}
