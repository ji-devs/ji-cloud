use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::{SearchQuery, SearchResponse, ImageId, GetResponse},
    error::image::*,
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};
use crate::{
    fetch::{api_with_auth, api_with_auth_empty, api_upload_file},
    path
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use url::Url;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, HtmlImageElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, dynamic_class_signal, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use itertools::Itertools;
use crate::math::{RectF64, PointF64};
use super::data::*;
use crate::drag::*;

pub struct TransformImage {
    img: SimpleImage,
    size: Mutable<Option<RectF64>>,
    drag: RefCell<BasicDrag>,
}

impl TransformImage { 
    pub fn new(img: SimpleImage) -> Self {
        Self { 
            img ,
            size: Mutable::new(None),
            drag: RefCell::new(BasicDrag::new())
        }
    }

    pub fn width_signal(&self) -> impl Signal<Item = String> {
        self.size.signal_ref(|size| {
            match size {
                None => "0".to_string(),
                Some(size) => format!("{}rem", size.width / 20.0) // should be 10.0 but sticker size is built for larger stage
            }
        }) 
    }
    pub fn height_signal(&self) -> impl Signal<Item = String> {
        self.size.signal_ref(|size| {
            match size {
                None => "0".to_string(),
                Some(size) => format!("{}rem", size.height/ 20.0) // should be 10.0 but sticker size is built for larger stage
            }
        }) 
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::image_transform(&_self.img), {
            .style_signal("width", _self.width_signal())
            .style_signal("height", _self.height_signal())
            .style_signal("transform", _self.drag.borrow().transform_signal())
            .class_signal("hidden", _self.size.signal_ref(|x| x.is_none()))
            .with_data_id!("img" => HtmlImageElement, {
                .with_node!(img_elem => {
                    .event(clone!(_self => move |evt:dominator_helpers::events::Load| {
                        let width = img_elem.width();
                        let height = img_elem.height();

                        _self.size.set(Some(RectF64::new(width.into(), height.into())));
                    }))
                })
            })
            .event(clone!(_self => move |evt:events::MouseDown| {
                if let Some(size) = _self.size.get() {
                    log::info!("starting drag!");
                    _self.drag.borrow_mut().start(
                        evt.x(), evt.y(), 
                        0.0, 0.0 // TODO - use size
                    );
                }
            }))
            .global_event(clone!(_self => move |evt:events::MouseMove| {
                _self.drag.borrow_mut().on_move(evt.x(), evt.y());
            }))
        })
    }
}
