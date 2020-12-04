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
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::templates;
use components::module::page::*;
use std::pin::Pin;


struct ExampleRenderer { 
    pub data: Mutable<bool>,
}

impl ModuleRenderer for ExampleRenderer {
    type Data = bool;
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type SidebarSignal = impl Signal<Item = Option<Dom>>;
    type HeaderSignal = impl Signal<Item = Option<Dom>>;
    type MainSignal = impl Signal<Item = Option<Dom>>;
    type FooterSignal = impl Signal<Item = Option<Dom>>;

    fn new(data:bool) -> Self {
        Self { 
            data: Mutable::new(data) 
        }
    }
    fn page_kind_signal(_self: Rc<Self>) -> Self::PageKindSignal {
        always(ModulePageKind::EditPlain)
    }

    fn sidebar_signal(_self: Rc<Self>) -> Self::SidebarSignal { 
        always(None)
    }
    fn header_signal(_self: Rc<Self>) -> Self::HeaderSignal { 
        always(Some(html!("h1", { .text ("header from signal!") } )))
    }

    fn main_signal(_self: Rc<Self>) -> Self::MainSignal { 
            _self.data.signal()
                .map(|x| {
                    if x {
                        Some(html!("h1", { .text ("main from signal!") } ))
                    } else {
                        None
                    }
                })
    }
    fn footer_signal(_self: Rc<Self>) -> Self::FooterSignal { 
        always(None)
    }
}
struct ExampleStaticRenderer { 
    pub data: bool,
}

impl StaticModuleRenderer for ExampleStaticRenderer {
    type Data = bool;

    fn new(data:bool) -> Self {
        Self { 
            data
        }
    }
    fn page_kind(_self: Rc<Self>) -> ModulePageKind { 
        ModulePageKind::EditPlain
    }

    fn sidebar(_self: Rc<Self>) -> Option<Dom> {
        None
    }
    fn header(_self: Rc<Self>) -> Option<Dom> {
        Some(html!("h1", { .text ("header from static!") } ))
    }
    fn main(_self: Rc<Self>) -> Option<Dom> {
        Some(html!("h1", { .text ("main from static!") } ))
    }
    fn footer(_self: Rc<Self>) -> Option<Dom> {
        None
    }
}

pub fn render_signals() -> Dom {

    let hello = Rc::new("hello".to_string());

    ModulePage::<ExampleRenderer, _>::render(clone!(hello => move || async move {
        if *hello == "hello" { true } else {false}
    }))
}

pub fn render_static() -> Dom {
    let hello = Rc::new("hello".to_string());

    ModulePage::<ExampleStaticRenderer, _>::render(clone!(hello => move || async move {
        if *hello == "hello" { true } else {false}
    }))
}

pub fn render() -> Dom {
    html!("div", {.text("hello")})
}

