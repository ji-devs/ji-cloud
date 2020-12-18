
/* There are a few fundamental concepts going on here...
 * 1. The serialized RawData is passed to the trait in order to get a State object 
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
use dominator_helpers::{make_custom_event_serde,dynamic_class_signal ,with_data_id, futures::{spawn_future, AsyncLoader}, signals::OptionSignal};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use serde::{Deserialize, de::DeserializeOwned};
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



make_custom_event_serde!("module-resize", ModuleResizeEvent, ResizeInfo);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModulePageKind {
    Empty,
    GridPlain,
    GridResize,
    GridResizeScrollable,
    Iframe,
}

impl ModulePageKind {
    pub fn is_resize(&self) -> bool {
        match self {
            Self::GridResize | Self::GridResizeScrollable | Self::Iframe => true,
            _ => false, 
        }
    }
    pub fn add_scrollable_attribute(&self) -> bool {
        match self {
            Self::GridResizeScrollable => true,
            _ => false
        }
    }

    pub fn element_name(&self) -> &str {
        match self {
            Self::GridResize => "module-page-grid-resize",
            Self::GridResizeScrollable => "module-page-grid-resize",
            Self::GridPlain => "module-page-grid-plain",
            Self::Iframe => "module-page-iframe",
            Self::Empty => "div"
        }
    }
}

pub trait ModuleRenderer <RawData, State> {
    type PageKindSignal: Signal<Item = ModulePageKind>;
    type SidebarSignal: Signal<Item = Option<Dom>>;
    type HeaderSignal: Signal<Item = Option<Dom>>;
    type MainSignal: Signal<Item = Option<Dom>>;
    type FooterSignal: Signal<Item = Option<Dom>>;
    type FutureState: Future<Output = Option<State>>;

    
    fn load_state() -> Self::FutureState; 
    fn derive_state(data: RawData) -> State; 
    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal; 
    fn sidebar_signal(state: Rc<State>, kind: ModulePageKind) -> Self::SidebarSignal; 
    fn header_signal(state: Rc<State>, kind: ModulePageKind) -> Self::HeaderSignal; 
    fn main_signal(state: Rc<State>, kind: ModulePageKind) -> Self::MainSignal; 
    fn footer_signal(state: Rc<State>, kind: ModulePageKind) -> Self::FooterSignal; 
}

pub trait StaticModuleRenderer <RawData, State> {
    type FutureState: Future<Output = Option<State>>;

    fn load_state() -> Self::FutureState; 
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
    type FutureState = impl Future<Output = Option<State>>;
    
    fn load_state() -> Self::FutureState { 
        T::load_state()
    }

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

    pub fn render() -> Rc<Self> 
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
                if let Some(state) = Renderer::load_state().await {
                    Self::render_data(_self, Rc::new(state));
                }
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
        //This div is just a placeholder to get messages
        //It'll be replaced when the iframe data arrives
        let dom = html!("div", {
            .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {
                //Get iframe data if we're supposed to
                if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                    if !_self.wait_iframe_data {
                        //log::warn!("weird... shouldn't have gotten iframe data!");
                        //log::warn!("{:?}", msg);
                    } else {
                        let raw_data = msg.data.unwrap_throw();
                        let state = Rc::new(Renderer::derive_state(raw_data));
                        Self::render_data(_self.clone(), state);
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

    fn render_data(_self: Rc<Self>, state: Rc<State>) {

        _self.switcher.load(
            Renderer::page_kind_signal(state.clone())
                .for_each(clone!(state => move |page_kind| {
                    let dom = html!(page_kind.element_name(), {
                        .apply_if(page_kind.add_scrollable_attribute(), |dom| {
                            dom.property("scrollable", true)
                        })
                        .event(|event:ModuleResizeEvent| {
                            //in utils / global static
                            set_resize_info(event.data());
                        })
                       
                        //TODO - get rid of nesting
                        //See: https://github.com/Pauan/rust-dominator/issues/45
                        .child(html!("div", {
                            .style("width", "100%")
                            .style("height", "100%")
                            .attribute("slot", "sidebar")
                            .child_signal(Renderer::sidebar_signal(state.clone(), page_kind))
                        }))
                        .child(html!("div", {
                            .style("width", "100%")
                            .style("height", "100%")
                            .attribute("slot", "header")
                            .child_signal(Renderer::header_signal(state.clone(), page_kind))
                        }))
                        .child(html!("div", {
                            .style("width", "100%")
                            .style("height", "100%")
                            .attribute("slot", "main")
                            .child_signal(Renderer::main_signal(state.clone(), page_kind))
                        }))
                        .child(html!("div", {
                            .style("width", "100%")
                            .style("height", "100%")
                            .attribute("slot", "footer")
                            .child_signal(Renderer::footer_signal(state.clone(), page_kind))
                        }))
                    });

                    Self::switch_body(dom); 
                    async {}
                }))
        );
    }
}
