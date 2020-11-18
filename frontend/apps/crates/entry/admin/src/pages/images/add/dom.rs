use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::routes::{Route, AdminRoute};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::ImageKind,
};
use super::actions;

pub struct ImageAdd {
    file_input:RefCell<Option<HtmlInputElement>>
}

impl ImageAdd{
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            file_input: RefCell::new(None)
        });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {

        elem!(templates::image_add(), {
            .with_data_id!("add-btn", {
                .event(clone!(_self => move |_evt:events::Click| {
                    if let Some(file_input) = _self.file_input.borrow().as_ref() {
                        file_input.click();
                    }
                }))
            })
            .with_data_id!("file", {
                .event(clone!(_self => move |_evt:events::Change| {
                    let file =
                        _self.file_input.borrow().as_ref()
                            .and_then(|input| input.files())
                            .and_then(|files| files.get(0));

                    if let Some(file) = file {
                        spawn_local(async move {
                            let id = actions::create_image(file, get_image_kind()).await.unwrap_throw();
                            let route:String = Route::Admin(AdminRoute::ImageEdit(id)).into();
                            dominator::routing::go_to_url(&route);
                        });
                    }
                }))

            })
            .after_inserted(clone!(_self => move |elem| {
                *_self.file_input.borrow_mut() = Some(elem.select(&data_id("file")));
            }))
        })
    }
}

fn get_image_kind() -> ImageKind {
    let document:web_sys::Document = 
        web_sys::window()
            .unwrap_throw()
            .document()
            .unwrap_throw();

    let input:HtmlInputElement = document.select("input[name='img_kind']:checked");
    match input.value().as_ref() {
        "sticker" => ImageKind::Sticker,
        "canvas" => ImageKind::Canvas,
        _ => panic!("unknown img kind!")
    }
}
