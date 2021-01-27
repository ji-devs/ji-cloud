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
use super::design::renderer::DesignRenderer;
use super::activities::renderer::ActivityRenderer;
use awsm_web::canvas::get_2d_context;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use utils::resize::get_resize_info;

impl Drop for Player {
    fn drop(&mut self) {
        log::info!("Player dropped!");
    }
}
pub struct Player {
    pub canvas: RefCell<Option<HtmlCanvasElement>>,
    pub jig_id: String,
    pub module_id: String,
    pub is_mock: bool,
    pub canvas_ready: Mutable<bool>,
    pub module: RefCell<Option<Module>>,
}

impl Player {
    pub fn render(shell: Rc<Shell>, module:Module) -> Dom {

        let _self = Rc::new(Self {
            canvas: RefCell::new(None),
            canvas_ready: Mutable::new(false),
            jig_id: shell.jig_id.clone(),
            module_id: module.id.clone(),
            is_mock: shell.debug.is_mock,
            module: RefCell::new(Some(module))
        });
       
        html!("empty-fragment", {
            .children(&mut[
                html!("canvas" => HtmlCanvasElement, {
                    .future(get_resize_info().signal_cloned().for_each(clone!(_self => move |resize_info| {
                        let canvas = _self.canvas.borrow();
                        if let Some(canvas) = canvas.as_ref() {
                            canvas.set_width(resize_info.width.round() as u32);
                            canvas.set_height(resize_info.height.round() as u32);
                        }
                        async {}
                    })))
                    .style("position", "absolute")
                    .style("top", "0")
                    .style("left", "0")
                    .style_signal("width", get_resize_info().signal_ref(|info| { format!("{}px", info.width) }))
                    .style_signal("height", get_resize_info().signal_ref(|info| { format!("{}px", info.height) }))
                    .after_inserted(clone!(_self => move |canvas| {
                        *_self.canvas.borrow_mut() = Some(canvas);
                        _self.canvas_ready.set(true);
                    }))
                }),

                html!("empty-fragment", {
                    .child_signal(_self.canvas_ready.signal().map(clone!(_self => move |ready| {
                        if ready {
                            let canvas = _self.canvas.borrow();
                            let canvas = canvas.as_ref().unwrap_throw();
                            let ctx = Rc::new(get_2d_context(&canvas, None).unwrap_throw());

                            let module = _self.module.take().unwrap_throw();

                            Some(html!("empty-fragment", {
                                .children(&mut [
                                    DesignRenderer::render(_self.clone(), ctx.clone(), module.design),
                                    ActivityRenderer::render(_self.clone(), ctx.clone(), module.activity)
                                ])
                            }))
                        } else {
                            None
                        }
                    })))
                })
            ])
        })
    }
}
