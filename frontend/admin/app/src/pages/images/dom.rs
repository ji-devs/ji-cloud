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
use crate::utils::templates;
use awsm_web::dom::*;
use super::actions;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, UserRoute};
use shared::domain::{
    user::UserProfile,
    category::Category
};
use super::{data::*, actions::*};

#[derive(Copy, Clone)]
enum PageMode {
    Add
}

pub struct ImagesPage {
    page_mode: Mutable<PageMode>
}

impl ImagesPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            page_mode: Mutable::new(PageMode::Add),
        });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::images_page(), {
            .with_data_id!("page-contents", {
                .child_signal(_self.page_mode.signal().map(clone!(_self => move |page_mode| {
                    Some(ImageAdd::render(ImageAdd::new()))
                })))
            })
        })
    }
}

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
                        log::info!("uploading {}", file.name()); 
                    }
                }))

            })
            .after_inserted(clone!(_self => move |elem| {
                *_self.file_input.borrow_mut() = Some(elem.select(&data_id("file")));
            }))
        })
    }
}
