use utils::{prelude::*, routes::{Route, ModuleRoute, AdminRoute}};
use shared::domain::jig::ModuleKind;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html, clone};
use dominator_helpers::futures::AsyncLoader;
use std::cell::RefCell;
use crate::{
    categories::dom::CategoriesPage,
    locale::{
        dom::LocalePage,
        state::LoaderState as LocaleLoaderState
    },
    images::{
        add::dom::ImageAddPage,
        meta::dom::ImageMetaPage,
        search::dom::ImageSearchPage,
        tags::ImageTags,
    },
    sidebar::dom::SidebarDom,
};
pub struct Router {
    loader: AsyncLoader,
    app: RefCell<Option<AppState>>
}

impl Router {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            app: RefCell::new(None)
        }
    }
}

pub enum AppState {
    Locale(Rc<LocaleLoaderState>)
}

pub fn render(state: Rc<Router>) {

    state.clone().loader.load(
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
            .for_each(clone!(state => move |route| {
                state.app.borrow_mut().take();

                let dom = match route {
                    Route::Admin(route_ref) => {
                        let route = route_ref.clone();
                        match route_ref {
                            AdminRoute::Categories=> Some(with_child(route, CategoriesPage::render())),
                            AdminRoute::Locale => {
                                let app_state = Rc::new(LocaleLoaderState::new());
                                *state.app.borrow_mut() = Some(AppState::Locale(app_state.clone()));
                                Some(with_child(route, LocalePage::render(app_state.clone())))
                            },
                            AdminRoute::ImageAdd => Some(with_child(route, ImageAddPage::render())),
                            AdminRoute::ImageMeta(id, is_new) => Some(with_child(route, ImageMetaPage::render(id, is_new))),
                            AdminRoute::ImageSearch(query) => Some(with_child(route, ImageSearchPage::render(query))),
                            AdminRoute::ImageTags => Some(with_child(route, ImageTags::render(ImageTags::new()))),
                            _ => Some(with_child(route, html!("empty-fragment"))),
                        }
                    }
                    _ => None

                };
                
                if let Some(dom) = dom {
                    let body = dominator::body();
                    body.set_inner_html("");
                    dominator::append_dom(&body, dom);
                }

                async {}
            }))
    );
}

fn with_child(route: AdminRoute, dom:Dom) -> Dom {
    html!("admin-shell", { 
        .child(SidebarDom::render(route))
        .child(dom)
    })
}






/*
use utils::routes::{Route, AdminRoute};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};

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
*/
