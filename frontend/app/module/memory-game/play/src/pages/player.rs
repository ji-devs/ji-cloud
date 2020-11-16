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
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use core::iframe::*;

pub struct PlayerPage {
    messaging_ready: Mutable<bool>,
    data_loaded: Mutable<bool>
}

impl PlayerPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            messaging_ready: Mutable::new(false),
            data_loaded: Mutable::new(true),
        });

        _self
    }

    fn ready(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let messaging_ready = self.messaging_ready.signal(),
            let data_loaded = self.data_loaded.signal()
            => {
                if *messaging_ready && *data_loaded {
                    true
                } else {
                    false
                }
            }
        }
    }

    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::module_page(), {
            .with_data_id!("module-page", {
                .child_signal(_self.ready().map(clone!(_self => move |ready| {
                    if ready {
                        Some(Self::render_player(_self.clone()))
                    } else {
                        None
                    }
                })))
            })

            .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {

                if let Ok(msg) = evt.try_serde_data::<IframeInit<String>>() {
                    //Parent sent us a message! 
                    log::info!("{}", msg.data.unwrap_throw());
                    _self.messaging_ready.set(true);
                } else {
                    log::info!("hmmm got other iframe message...");
                }
                let hello:IframeInit<String> = evt.serde_data_unchecked();
            }))
            .after_inserted(clone!(_self => move |elem| {
                //On mount - send an empty IframeInit message to let the parent know we're ready
                let target = web_sys::window().unwrap_throw().parent().unwrap_throw().unwrap_throw();
                let msg = core::iframe::IframeInit::empty();

                target.post_message(&msg.into(), "*");
            }))
        })
    }

    fn render_player(_self: Rc<Self>) -> Dom {
        elem!(templates::player(), {
        })
    }
}
