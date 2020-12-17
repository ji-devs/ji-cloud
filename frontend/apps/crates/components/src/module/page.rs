
/* There are a few fundamental concepts going on here...
 * 1. The serialized data does _not_ need to be Clone.
 *    rather, it's passed completely to the renderer
 *    and then the renderer is free to split it up for Mutable/etc.
 *    (here it is held and taken from an Option)
 * 2. The loader will be skipped if the url has ?iframe_data=true
 *    in this case, iframe communication is setup and the parent
 *    is expected to post a message with the data (via IframeInit)
 * 3. The core mechanism is build around ModuleRenderer/signals, however
 *    sometimes the top-level elements are static, so StaticModuleRenderer
 *    is provided as a helper. It's not a performance saver though
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
use dominator_helpers::{elem,dynamic_class_signal ,with_data_id, futures::{spawn_future, AsyncLoader}, signals::OptionSignal};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use serde::de::DeserializeOwned;
use utils::{
    iframe::*,
    resize::*,
};
use awsm_web::dom::resize::*;
use std::future::Future;
use async_trait::async_trait;
use std::pin::Pin;
use std::marker::Unpin;
use std::task::{Context, Poll};

use discard::DiscardOnDrop;
/*
*/

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
    pub fn element_name(&self) -> &str {
        match self {
            Self::EditResize => "module-page-resize",
            Self::PlayIframe => "module-page-iframe",
            Self::PlayIframePreview => "module-page-iframe-preview",
            Self::EditPlain => "module-edit-plain",
            Self::Empty => "module-empty"
        }
    }
}

pub trait ModuleRenderer <RawData, State> {
    type PageKindSignal: Signal<Item = ModulePageKind>;
    type SidebarSignal: Signal<Item = Option<Dom>>;
    type HeaderSignal: Signal<Item = Option<Dom>>;
    type MainSignal: Signal<Item = Option<Dom>>;
    type FooterSignal: Signal<Item = Option<Dom>>;

    fn derive_state(data: RawData) -> State; 
    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal; 
    fn sidebar_signal(state: Rc<State>, kind: ModulePageKind) -> Self::SidebarSignal; 
    fn header_signal(state: Rc<State>, kind: ModulePageKind) -> Self::HeaderSignal; 
    fn main_signal(state: Rc<State>, kind: ModulePageKind) -> Self::MainSignal; 
    fn footer_signal(state: Rc<State>, kind: ModulePageKind) -> Self::FooterSignal; 
}

pub trait StaticModuleRenderer <RawData, State> {

    fn derive_state(state: RawData) -> State; 
    fn page_kind(state: Rc<State>) -> ModulePageKind; 
    fn sidebar(state: Rc<State>, kind: ModulePageKind) -> Option<Dom>; 
    fn header(state: Rc<State>, kind: ModulePageKind) -> Option<Dom>;
    fn main(state: Rc<State>, kind: ModulePageKind) -> Option<Dom>; 
    fn footer(state: Rc<State>, kind: ModulePageKind) -> Option<Dom>; 
}

impl <RawData, State, T: StaticModuleRenderer<RawData, State>> ModuleRenderer<RawData, State> for T {
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type SidebarSignal = impl Signal<Item = Option<Dom>>;
    type HeaderSignal = impl Signal<Item = Option<Dom>>;
    type MainSignal = impl Signal<Item = Option<Dom>>;
    type FooterSignal = impl Signal<Item = Option<Dom>>;
    
    fn derive_state(raw_data: RawData) -> State { 
        T::derive_state(raw_data)
    }

    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal { 
        always(T::page_kind(state))
    }
    fn sidebar_signal(state: Rc<State>, kind: ModulePageKind) -> Self::SidebarSignal { 
        always(T::sidebar(state, kind))
    }
    fn header_signal(state: Rc<State>, kind: ModulePageKind) -> Self::HeaderSignal { 
        always(T::header(state, kind))
    }
    fn main_signal(state: Rc<State>, kind: ModulePageKind) -> Self::MainSignal { 
        always(T::main(state, kind))
    }
    fn footer_signal(state: Rc<State>, kind: ModulePageKind) -> Self::FooterSignal { 
        always(T::footer(state, kind))
    }
}

pub struct ModulePage<Renderer, RawData, State> 
where
    Renderer: ModuleRenderer<RawData, State>,
    RawData: DeserializeOwned,
{
    wait_iframe_data: bool,
    loader: AsyncLoader,
    switcher: AsyncLoader, 
    phantom: PhantomData<(Renderer, RawData, State)>,
}

impl <Renderer, RawData, State> ModulePage <Renderer, RawData, State> 
where
    Renderer: ModuleRenderer<RawData, State> + 'static,
    RawData: DeserializeOwned + 'static,
    State: 'static,
{

    pub fn render<Loader, F>(load: Loader) -> Rc<Self> 
        where Loader: FnOnce() -> F + 'static,
              F: Future<Output = RawData>
    {

        let wait_iframe_data = should_get_iframe_data();

        let _self = Rc::new(Self { 
            loader: AsyncLoader::new(),
            switcher: AsyncLoader::new(), 
            wait_iframe_data,
            phantom: PhantomData,
        });

        let _self_clone = _self.clone();

        if !wait_iframe_data {
            _self_clone.loader.load(async move {
                let data = load().await; 
                Self::render_data(_self, data);
            });
        } else {
            Self::render_iframe_wait(_self);
        }
        
        _self_clone
    }

    fn switch_body(dom:Dom) {
        let body = dominator::body();
        body.set_inner_html("");
        dominator::append_dom(&body, dom);
    }

    fn render_iframe_wait(_self: Rc<Self>) {
        let dom = html!("div", {
            .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {
                //Get iframe data if we're supposed to
                if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                    if !_self.wait_iframe_data {
                        //log::warn!("weird... shouldn't have gotten iframe data!");
                        //log::warn!("{:?}", msg);
                    } else {
                        Self::render_data(_self.clone(), msg.data.unwrap_throw());
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
        });
        Self::switch_body(dom); 
    }

    fn render_data(_self: Rc<Self>, raw_data:RawData) {
        let state = Rc::new(Renderer::derive_state(raw_data));

        _self.switcher.load(
            Renderer::page_kind_signal(state.clone())
                .for_each(clone!(_self, state => move |page_kind| {
                    log::info!("hmmmm");
                    let dom = html!(page_kind.element_name(), {

                        //Note - not observing size changes on main
                        //Main is ultimately what's scaled :)
                        //.child_slot_signal("main", Renderer::main_signal(renderer.clone()))
                        //Each of these sections sets up for observing for resize
                        //and also renders the signal defined by the renderer trait
                        //
                        //
                        .child_signal( 
                            Renderer::sidebar_signal(state.clone(), page_kind)
                                .map(|dom| {
                                    dom.map(|dom| {
                                        //HRMF
                                        dom
                                            //.property("slot", "sidebar")
                                    })
                                })
                        )
                    });

                    Self::switch_body(dom); 
                    async {}
                }))
        );

        log::info!("rendered data...");
    }
}

//////////// EXAMPLE
/*
mod example {
    use super::*;
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
            always(None)
        }
        fn main_signal(_self: Rc<Self>) -> Self::MainSignal { 
                _self.data.signal()
                    .map(|x| {
                        if x {
                            Some(html!("h1", { .text ("it works!") } ))
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
    }
}
*/
