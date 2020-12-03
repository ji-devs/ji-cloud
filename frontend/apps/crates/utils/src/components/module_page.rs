/* There are a few fundamental concepts going on here...
 * 1. The serialized data does _not_ need to be Clone.
 *    rather, it's passed completely to the renderer
 *    and then the renderer is free to split it up for Mutable/etc.
 *    (here it is held and taken from an Option)
 * 2. The loader will be skipped if the url has ?iframe_data=true
 *    in this case, iframe communication is setup and the parent
 *    is expected to post a message with the data (via IframeInit)
 * 3. The core mechanism is build around ModuleRenderer, however
 *    Boxing and pinning can be a bit annoying, so StaticModuleRenderer
 *    is provided as a helper in cases where the top-level containers don't change. 
 *    It does not prevent dynamic dispatch, however
 *    (in fact there's a small performance overhead since it creates a new signal)
 */
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal, always},
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
use std::pin::Pin;
use std::marker::Unpin;
use std::task::{Context, Poll};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModulePageKind {
    Empty,
    EditPlain,
    EditResize,
    PlayIframe,
    PlayIframePreview,
}

impl ModulePageKind {
    pub fn is_resize(&self) -> bool {
        match self {
            Self::EditResize | Self::PlayIframe | Self::PlayIframePreview => true,
            Self::EditPlain | Self::Empty => false
        }
    }
}

pub trait ModuleRenderer<Data> {
    fn new(data:Data) -> Self;
    fn page_kind_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = ModulePageKind>>>;
    fn sidebar_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>>;
    fn header_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>>;
    fn main_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>>;
    fn footer_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>>;
}

pub trait StaticModuleRenderer<Data> {
    fn new(data:Data) -> Self;
    fn page_kind(_self: Rc<Self>) -> ModulePageKind; 
    fn sidebar(_self: Rc<Self>) -> Option<Dom>; 
    fn header(_self: Rc<Self>) -> Option<Dom>; 
    fn main(_self: Rc<Self>) -> Option<Dom>; 
    fn footer(_self: Rc<Self>) -> Option<Dom>; 
}

impl <Data, T: StaticModuleRenderer<Data>> ModuleRenderer<Data> for T {
    fn new(data:Data) -> Self {
        T::new(data)
    }
    fn page_kind_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = ModulePageKind>>> {
        Box::pin(always(T::page_kind(_self)))
    }
    fn sidebar_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> {
        Box::pin(always(T::sidebar(_self)))
    }
    fn header_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> {
        Box::pin(always(T::header(_self)))
    }
    fn main_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> {
        Box::pin(always(T::main(_self)))
    }
    fn footer_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> {
        Box::pin(always(T::footer(_self)))
    }
}

pub struct ModulePage<Renderer, Data> 
where
    Renderer: ModuleRenderer<Data>,
    Data: DeserializeOwned,
{
    module_renderer: RefCell<Option<Rc<Renderer>>>,
    has_loaded_data: Mutable<bool>, 
    wait_iframe_data: bool,
    loader: AsyncLoader,
    phantom: PhantomData<Data>
}

impl <Renderer, Data> ModulePage <Renderer, Data> 
where
    Renderer: ModuleRenderer<Data> + 'static,
    Data: DeserializeOwned + 'static,
{
    pub fn render<Loader, F>(load: Loader) -> Dom 
        where Loader: FnOnce() -> F + 'static,
              F: Future<Output = Data>
    {
        html!("div", {
              .class("w-full")
              .class("h-full")
              .child_signal(Self::dom_signal(Self::init(load)))
        })
    }

    fn init<Loader, F>(load: Loader) -> Rc<Self> 
        where Loader: FnOnce() -> F + 'static,
              F: Future<Output = Data>
    {

        let wait_iframe_data = should_get_iframe_data();

        let _self = Rc::new(Self { 
            module_renderer: RefCell::new(None),
            has_loaded_data: Mutable::new(false), 
            loader: AsyncLoader::new(),
            wait_iframe_data,
            phantom: PhantomData
        });

        let _self_clone = _self.clone();

        if !wait_iframe_data {
            _self_clone.loader.load(async move {
                let data = load().await; 
                Self::stash(_self, data);
            });
        }
        
        _self_clone
    }

    fn stash(_self: Rc<Self>, data:Data) {
        *_self.module_renderer.borrow_mut() = Some(Rc::new(Renderer::new(data)));
        _self.has_loaded_data.set(true);
    }

    fn dom_signal(_self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {

        _self.has_loaded_data.signal().map(clone!(_self => move |has_loaded| {
            if !has_loaded {
                None
            } else {
                let renderer = _self.module_renderer.borrow();
                let renderer = renderer.as_ref().unwrap_throw();

                Some(
                    html!("div", {
                        .class("w-full")
                        .class("h-full")
                        .child_signal(Renderer::page_kind_signal(renderer.clone())
                            .map(clone!(_self, renderer => move |page_kind| {Some(
                                elem!(templates::module_page(page_kind), {
                                    .with_data_id!("sidebar", { .child_signal( 
                                        Renderer::sidebar_signal(renderer.clone())
                                    )})
                                    .with_data_id!("header", { .child_signal( 
                                        Renderer::header_signal(renderer.clone())
                                    )})
                                    .with_data_id!("main", { .child_signal( 
                                        Renderer::main_signal(renderer.clone())
                                    )})
                                    .with_data_id!("footer", { .child_signal( 
                                        Renderer::footer_signal(renderer.clone())
                                    )})
                                    .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {

                                        if let Ok(msg) = evt.try_serde_data::<IframeInit<Data>>() {
                                            if !_self.wait_iframe_data {
                                                //log::warn!("weird... shouldn't have gotten iframe data!");
                                                //log::warn!("{:?}", msg);
                                            } else {
                                                Self::stash(_self.clone(), msg.data.unwrap_throw());
                                            }
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

                                    .apply_if(page_kind.is_resize(), |dom| {
                                        apply_methods!(dom, {
                                            .with_data_id!("module-outer", {
                                                .with_data_id!("module-content", {
                                                })

                                                .with_node!(elem => {
                                                    .global_event(move |evt:events::Resize| {
                                                        ModuleBounds::set_elem(&elem);
                                                    })
                                                })
                                                .after_inserted(|elem| {
                                                    ModuleBounds::set_elem(&elem);
                                                })
                                            })
                                        })
                                    })
                                })
                            )}))
                        )
                    })
                )
            }
        }))
    }
}


//////////// EXAMPLE

/*struct ExampleRenderer { 
    pub data: Mutable<bool>,
}

impl ModuleRenderer<bool> for ExampleRenderer {
    fn new(data:bool) -> Self {
        Self { 
            data: Mutable::new(data) 
        }
    }
    fn page_kind_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = ModulePageKind>>> { 
        Box::pin(always(ModulePageKind::EditPlain))
    }

    fn sidebar_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> { 
        Box::pin(always(None))
    }
    fn header_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> { 
        Box::pin(always(None))
    }
    fn main_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> { 
        Box::pin(
            _self.data.signal()
                .map(|x| {
                    if x {
                        Some(html!("h1", { .text ("it works!") } ))
                    } else {
                        None
                    }
                })
        )
    }
    fn footer_signal(_self: Rc<Self>) -> Pin<Box<dyn Signal<Item = Option<Dom>>>> { 
        Box::pin(always(None))
    }
}
struct ExampleStaticRenderer { 
    pub data: bool,
}

impl StaticModuleRenderer<bool> for ExampleStaticRenderer {
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
        None
    }
    fn main(_self: Rc<Self>) -> Option<Dom> {
        Some(html!("h1", { .text ("it works!") } ))
    }
    fn footer(_self: Rc<Self>) -> Option<Dom> {
        Some(html!("h1", { .text ("it works!") } ))
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
}*/
