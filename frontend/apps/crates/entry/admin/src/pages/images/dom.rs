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
use utils::routes::{Route, UserRoute};
use shared::domain::{
    user::UserProfile,
    category::Category
};
use super::add::ImageAdd;
use super::edit::ImageEdit;
use super::search::ImageSearch;

#[derive(Clone, Debug)]
pub enum PageMode {
    Add,
    Edit(String),
    Search 
}

pub struct ImagesPage {
    page_mode: Mutable<PageMode>
}

impl ImagesPage {
    pub fn new(mode:PageMode) -> Rc<Self> {
        let _self = Rc::new(Self { 
            page_mode: Mutable::new(mode),
        });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::images_page(), {
            .with_data_id!("page-contents", {
                .child_signal(_self.page_mode.signal_cloned().map(clone!(_self => move |page_mode| {
                    match page_mode {
                        PageMode::Add => Some(ImageAdd::render(ImageAdd::new())),
                        PageMode::Edit(id) => Some(ImageEdit::render(ImageEdit::new(id))),
                        PageMode::Search => Some(ImageSearch::render(ImageSearch::new())),
                    }
                })))
            })
        })
    }
}
