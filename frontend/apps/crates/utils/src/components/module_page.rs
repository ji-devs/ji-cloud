/* There are a few fundamental concepts going on here...
 * 1. The serialized data does _not_ need to be Clone.
 *    rather, it's passed completely to the child
 *    and then the child is free to split it up for Mutable/etc.
 *    (here it is held and taken from an Option)
 * 2. The loader will be skipped if the url has ?iframe_data=true
 *    in this case, iframe communication is setup and the parent
 *    is expected to post a message with the data (via IframeInit)
 */
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{Url, HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem,dynamic_class_signal ,with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use serde::de::DeserializeOwned;
use crate::{
    iframe::*,
    resize::*,
};
use std::future::Future;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ModuleRenderer {
    type Data: DeserializeOwned;

    async fn load(_self:Rc<Self>) -> Self::Data;
    fn render(_self: Rc<Self>, data: Self::Data) -> Dom;
}

pub struct ModulePage<T, R> 
where
    T: DeserializeOwned,
    R: ModuleRenderer<Data = T>,
{
    renderer: Rc<R>,
    loaded_data: RefCell<Option<T>>, 
    has_loaded_data: Mutable<bool>, 
    wait_iframe_data: bool,
    loader: AsyncLoader,
}

impl <T, R> ModulePage <T, R> 
where
    T: DeserializeOwned + 'static,
    R: ModuleRenderer<Data = T> + 'static,
{
    pub fn new(renderer:Rc<R>) -> Rc<Self> {

        let wait_iframe_data = should_get_iframe_data();

        let _self = Rc::new(Self { 
            renderer, 
            loaded_data: RefCell::new(None),
            has_loaded_data: Mutable::new(false), 
            loader: AsyncLoader::new(),
            wait_iframe_data,
        });

        let _self_clone = _self.clone();
        _self_clone.loader.load(async move {
            if !wait_iframe_data {
                let data:T = ModuleRenderer::load(_self.renderer.clone()).await;
                *_self.loaded_data.borrow_mut() = Some(data);
                _self.has_loaded_data.set(true);
            }
        });
        
        _self_clone
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::module_page(), {
            .with_data_id!("module-page", {
                .with_data_id!("module-content", {
                    .child_signal(_self.has_loaded_data.signal().map(clone!(_self => move |ready| {
                        if ready {
                            let data = _self.loaded_data.borrow_mut().take().unwrap_throw();
                            Some(ModuleRenderer::render(_self.renderer.clone(),data))
                        } else {
                            None
                        }
                    })))
                })

                .with_node!(elem => {
                    .global_event(clone!(_self => move |evt:events::Resize| {
                        ModuleBounds::set(
                            elem.client_width() as f64, 
                            elem.client_height() as f64
                        );
                    }))
                })
                .after_inserted(clone!(_self => move |elem| {
                    ModuleBounds::set(
                        elem.client_width() as f64, 
                        elem.client_height() as f64
                    );
                }))
            })

            .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {

                if let Ok(msg) = evt.try_serde_data::<IframeInit<T>>() {
                    if !_self.wait_iframe_data {
                        log::warn!("weird... shouldn't have gotten iframe data!");
                    }
                    *_self.loaded_data.borrow_mut() = Some(msg.data.unwrap_throw());
                    _self.has_loaded_data.set(true);
                } else {
                    log::info!("hmmm got other iframe message...");
                }
            }))
            .after_inserted(clone!(_self => move |elem| {
                if _self.wait_iframe_data {
                    //On mount - send an empty IframeInit message to let the parent know we're ready
                    let target = web_sys::window().unwrap_throw().parent().unwrap_throw().unwrap_throw();
                    let msg = IframeInit::empty();

                    target.post_message(&msg.into(), "*");
                }
            }))
        })
    }

}
