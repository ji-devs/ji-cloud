use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::templates;
use components::module::page::*;
use std::pin::Pin;
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTemplateElement, DocumentFragment, Document};

pub struct Page { 
    pub toggle: Mutable<bool>,
}

fn template_signal<S: Signal<Item = Dom> + 'static>(child: S) -> Dom {

    let template:HtmlTemplateElement = web_sys::window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .create_element("template")
        .unwrap_throw()
        .unchecked_into();

    DomBuilder::new(template)
        .child_signal(child.map(|x| Some(x)))
        .into_dom()
        //.into_dom_template()

}

/*
impl DomBuilder<HtmlTemplateElement> {
    #[inline]
    fn into_dom_template(self) -> Dom {
        let frag:DocumentFragment = template.content();
        let el:HtmlElement = frag.first_child().unwrap().unchecked_into();

        Dom {
            element: el.into(),
            callbacks: self.callbacks,
        }
    }
}
*/

impl Page {
    pub fn render() -> Dom {
        Self::_render(Self::new())
    }

    fn new() -> Rc<Self> {
        Rc::new(Self {toggle: Mutable::new(false) } )
    }


    fn _render(_self:Rc<Self>) -> Dom {
        template_signal(Self::child_dom(_self))
    }

    fn child_dom(_self:Rc<Self>) -> impl Signal<Item = Dom> {
        _self.toggle.signal().map(clone!(_self => move |flag| {
                html!("button", {
                    .text(if flag { "true" } else { "false" })
                    .event(clone!(_self => move |evt:events::Click| {
                        let mut toggle = _self.toggle.lock_mut();
                        *toggle = !*toggle;
                    }))
                })
        }))
    }
}
