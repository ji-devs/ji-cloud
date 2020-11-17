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
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, AdminRoute};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::ImageKind,
};

use super::data::*;

pub struct ModuleSelect {
}

impl ModuleSelect {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self {});

        _self
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::edit_module_selection(), {
            .with_data_id!("poster", {
                .event(drag_callback("poster".to_string()))
            })
            .with_data_id!("memory-game", {
                .event(drag_callback("memory-game".to_string()))
            })
        })
    }
}

fn drag_callback(name:String) -> impl Fn(events::DragStart) {
    move |evt:events::DragStart| {
        if let Some(data_transfer) = evt.data_transfer() {
            data_transfer.set_data("module_kind", &name);
            data_transfer.set_drop_effect("all");
        } else {
            log::error!("no data transfer - use a real computer!!!");
        }
    }
}
