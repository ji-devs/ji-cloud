use utils::routes::{Route, DevRoute};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html, clone};
use crate::pages::module_grid;
use discard::DiscardOnDrop;
use components::module::page::*;
use dominator_helpers::{elem, with_data_id,futures::{spawn_future, AsyncLoader}};

pub struct Router {
    loader: AsyncLoader,
    page: RefCell<Option<PageKind>>
}

enum PageKind {
    Grid(module_grid::dom::Page) // dev/showcase/001/grid
}

impl Router {
    pub fn render() {

        let _self = Rc::new(Self {
            loader: AsyncLoader::new(),
            page: RefCell::new(None)
        });

        _self.clone().loader.load(
            dominator::routing::url()
                .signal_ref(|url| Route::from_url(&url))
                .for_each(clone!(_self => move |route| {
                    *_self.page.borrow_mut() =
                        page_str(route)
                            .and_then(|page| match page.as_ref() {
                                "grid" => Some(PageKind::Grid(module_grid::dom::render())),
                                _ => None
                            });

                    async {}
                }))
        );
    }
}

fn page_str(route:Route) -> Option<String> {
    match route {
        Route::Dev(route) => match route {
            DevRoute::Showcase(_, page) => {
                Some(page)
            },
            _ => None
        },
        _ => None
    }
}
