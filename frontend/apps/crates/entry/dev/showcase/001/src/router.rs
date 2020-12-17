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
use crate::pages::resize::{self, ResizePage};
use discard::DiscardOnDrop;
use components::module::page::*;
use dominator_helpers::{elem, with_data_id,futures::{spawn_future, AsyncLoader}};

pub struct Router {
    loader: AsyncLoader,
    page: RefCell<Option<ResizePage>>
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
                    *_self.page.borrow_mut() = Some(resize::render());
                    async {}
                }))
        );

        //No need to leak because _self is held in the router signal
                
    }
}
