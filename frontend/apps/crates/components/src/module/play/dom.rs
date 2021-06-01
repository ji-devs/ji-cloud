use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal, always},
    signal_vec::{self, MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{Url, HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{dynamic_class_signal, futures::{spawn_future, AsyncLoader}, make_custom_event_serde, signals::{DefaultSignal, OptionSignal}, with_data_id};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use utils::{
    iframe::*,
    resize::*,
    events::ModuleResizeEvent,
    prelude::*,
};
use awsm_web::dom::resize::*;
use std::future::Future;
use async_trait::async_trait;
use std::pin::Pin;
use std::marker::Unpin;
use std::task::{Context, Poll};
use discard::DiscardOnDrop;
use super::state::*;
use crate::module::page::*;
use shared::domain::jig::module::body::BodyExt;

pub fn render_page_body<RawData, Main> (state:Rc<GenericState<RawData, Main>>)
where
    Main: MainExt + 'static,
    RawData: BodyExt + 'static, 
{

    let sig =
            state.phase.signal_cloned().map(clone!(state => move |phase| {
                let page_kind = match phase.as_ref() {
                    Phase::Init | Phase::WaitingIframe(_) => ModulePageKind::GridPlain,
                    Phase::Playing(_) => ModulePageKind::Iframe
                };

                let has_resized_once = Mutable::new(!page_kind.is_resize());

                html!(page_kind.element_name(), {
                        .apply_if(page_kind.add_scrollable_attribute(), |dom| {
                            dom.property("scrollable", true)
                        })
                        .apply_if(page_kind.add_preview_attribute(), |dom| {
                            dom.property("preview", true)
                        })
                        .event(clone!(has_resized_once => move |event:ModuleResizeEvent| {
                            //in utils / global static
                            set_resize_info(event.data());
                            has_resized_once.set_neq(true);
                        }))
                        .children_signal_vec({
                            has_resized_once.signal()
                                .map(clone!(state, phase => move |has_resized_once| {
                                    if !has_resized_once {
                                        vec![]
                                    } else {
                                        match phase.as_ref() {
                                            Phase::WaitingIframe(on_raw) => {
                                                vec![render_iframe_wait(state.clone(), on_raw.clone())]
                                            },
                                            Phase::Playing(main) => {
                                                vec![

                                                    add_slot_to_dom(Main::render(main.clone()), "main")
                                                ]
                                            },
                                            Phase::Init => vec![]
                                        }
                                    }
                                }))
                                .to_signal_vec()
                        })
                })
            }));

    state.page_body_switcher.load(sig.for_each(|dom| {

        let body = dominator::body();
        body.set_inner_html("");
        dominator::append_dom(&body, dom);
        async move {}
    }));


}


//This is just a placeholder to get messages
//It'll be replaced when the iframe data arrives
fn render_iframe_wait<RawData, Main> (state:Rc<GenericState<RawData, Main>>, on_raw: Rc<Box<dyn Fn(RawData)>>) -> Dom
where
    Main: MainExt + 'static,
    RawData: BodyExt + 'static, 
{

    html!("empty-fragment", {
        .global_event(clone!(state, on_raw => move |evt:dominator_helpers::events::Message| {
            if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                log::info!("got iframe data!");

                let raw_data = msg.data.expect_ji("couldn't decode iframe data");

                //on_raw was stashed from the original State::new()
                on_raw(raw_data);
            } else {
                log::info!("hmmm got other iframe message...");
            }
        }))
        .after_inserted(clone!(state => move |elem| {
            let parent = web_sys::window()
                .unwrap_ji()
                .parent()
                .unwrap_ji()
                .unwrap_ji();
            //On mount - send an empty IframeInit message to let the parent know we're ready
            let msg = IframeInit::empty();

            parent.post_message(&msg.into(), "*");
        }))
    })

}


fn add_slot_to_dom(dom:Dom, slot:&str) -> Dom {
    //there might be a better way, like Dom->DomBuilder->Dom
    html!("empty-fragment", {
        .property("slot", slot)
        .child(dom)
    })
}
