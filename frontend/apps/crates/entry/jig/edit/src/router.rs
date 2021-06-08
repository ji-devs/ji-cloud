use utils::routes::{JigEditRoute, JigPlayMode, JigRoute, Route};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::{
    edit::dom::EditPage,
    gallery::dom::GalleryDom,
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
                    Route::Jig(route) => {
                        match route {
                            JigRoute::Gallery => Some(GalleryDom::render()),
                            JigRoute::Edit(jig_id, route) => {
                                Some(EditPage::render(jig_id, route)) 
                            },
                            _ => None
                        }
                    },
                    _ => None
                }
            })
    }
    
    pub fn render(&self) -> Dom {
        html!("main", { .child_signal(Self::dom_signal()) } )
    }
}
