use utils::routes::{Route, ModuleRoute};
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
use crate::index::dom::{IndexDom, Page};

pub struct Router {
    loader: AsyncLoader,
    page: RefCell<Option<Page>>
}


pub fn render() {
    let _self = Rc::new(Router {
        loader: AsyncLoader::new(),
        page: RefCell::new(None)
    });

    _self.clone().loader.load(
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
            .for_each(clone!(_self => move |route| {
                *_self.page.borrow_mut() = match route {
                    Route::Module(route) => {
                        match route {
                            ModuleRoute::Play(kind, jig_id, module_id) => {
                                match kind {
                                    ModuleKind::Poster => Some(IndexDom::render(jig_id, module_id)),
                                    _ => None
                                }
                            }
                            _ => None
                        }
                    },
                    _ => None
                };
                async {}
            }))
    );
}
