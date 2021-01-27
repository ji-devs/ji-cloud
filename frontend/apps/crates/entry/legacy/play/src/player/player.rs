use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, futures::AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::shell::Shell;
use awsm_web::loaders::fetch::fetch_url;
use legacy::*;
use super::sticker_image::StickerImage;

pub struct Player {
    pub module:Module
}

impl Player {
    pub fn render(shell: Rc<Shell>, module:Module) -> Dom {
        let _self = Rc::new(Self { 
            module
        });

        log::info!("{:?}", _self.module.design.bg);

        let mut children:Vec<Dom> = Vec::new();

        let jig_id = shell.jig_id.clone();
        let module_id = _self.module.id.clone();

        if let Some(bg) = _self.module.design.bg.as_ref() {
            children.push(
                html!("img-legacy", {
                    .property("jigId", &jig_id)
                    .property("moduleId", &module_id)
                    .property("path", &format!("layers/{}", bg))
                    .apply_if(shell.debug.is_mock, |dom| {
                        dom.property("mock", true)
                    })
                })
            )
        }
       
        for sticker in _self.module.design.stickers.iter() {
            match sticker {
                Sticker::Text(_) => {},
                Sticker::Image(img) => {
                    children.push(
                        StickerImage::render(&jig_id, &module_id, &img, shell.debug.is_mock)
                    )
                }
            }
        }

        html!("div", {
            .children(children)
        })
    }
}
