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
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use super::templates;
use components::module::page::*;
use std::pin::Pin;


const INITIAL_MODE:ModulePageKind = ModulePageKind::EditResize;

pub fn render() -> Dom {
    ModulePage::<ResizeRenderer, _>::render(|| async {
    })
}

struct ResizeRenderer { 
    pub kind: Mutable<ModulePageKind>
}

impl ModuleRenderer for ResizeRenderer {
    type Data = ();
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type SidebarSignal = impl Signal<Item = Option<Dom>>;
    type HeaderSignal = impl Signal<Item = Option<Dom>>;
    type MainSignal = impl Signal<Item = Option<Dom>>;
    type FooterSignal = impl Signal<Item = Option<Dom>>;

    fn new(data:()) -> Self {
        log::info!("first render");
        Self { 
            kind: Mutable::new(INITIAL_MODE) 
        }
    }
    fn page_kind_signal(_self: Rc<Self>) -> Self::PageKindSignal {
        _self.kind.signal()
    }

    fn sidebar_signal(_self: Rc<Self>) -> Self::SidebarSignal {
        _self.kind.signal().map(|kind| {
            templates::sidebar(kind).map(|el| {
                elem!(el, {})
            })
        })
    }
    fn header_signal(_self: Rc<Self>) -> Self::HeaderSignal { 
        _self.kind.signal().map(|kind| {
            templates::header(kind).map(|el| {
                elem!(el, {})
            })
        })
    }

    fn main_signal(_self: Rc<Self>) -> Self::MainSignal { 
        _self.kind.signal().map(|kind| {
            templates::main(kind).map(|el| {
                elem!(el, {})
            })
        })
    }
    fn footer_signal(_self: Rc<Self>) -> Self::FooterSignal { 
        _self.kind.signal().map(|kind| {
            templates::footer(kind).map(|el| {
                elem!(el, {})
            })
        })
    }
}
