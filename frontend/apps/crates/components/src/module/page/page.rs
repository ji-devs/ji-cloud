
/* There are a few fundamental concepts going on here...
  1. The serialized RawData is passed to the trait in order to get a State object 
  2. The loader will be skipped if the url has ?iframe_data=true
     in this case, iframe communication is setup and the parent
     is expected to post a message with the data (via IframeInit)
     and then the state is derived via derive_state()
  3. The whole mechanism assumes that this is the top-level page - it replaces body contents
     completely
  4. The resize mechanism is dealt with in a custom element, which dispatches a custom event (this
     is just because it's easier to see in Storybook that way)
    
  For a page to render, it must provide something that satisfies the ModuleRenderer trait
 
  Then it can merely call: 
  ```
    ModulePage::<MyPageRenderer, MyRawData, MyState>::render()
  ```
    
   and it will not need to worry about any of the top-level page things such as:
    * differentiating between loaded data and iframe data
    * dealing with page resizing

   it _sortof_ needs to worry about rendering to different grid areas - solving that in the trait
   turned out to be way too verbose/tricky and didn't help much

   To target a grid area, simply set `.attribute("slot", gridname)` where gridname is 
   one of "sidebar", "header", "main" or "footer"
 */

/*
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
use dominator_helpers::{make_custom_event_serde,dynamic_class_signal ,with_data_id, futures::{spawn_future, AsyncLoader}, signals::OptionSignal};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use serde::{Deserialize, de::DeserializeOwned};
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
use super::load::StateLoader;
*/


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModulePageKind {
    Empty,
    GridPlain,
    GridResize,
    GridResizeScrollable,
    GridResizePreview,
    Iframe,
}

impl ModulePageKind {
    pub fn is_resize(&self) -> bool {
        match self {
            Self::GridResize | Self::GridResizeScrollable | Self::GridResizePreview | Self::Iframe => true,
            _ => false, 
        }
    }
    pub fn add_scrollable_attribute(&self) -> bool {
        match self {
            Self::GridResizeScrollable => true,
            _ => false
        }
    }

    pub fn add_preview_attribute(&self) -> bool {
        match self {
            Self::GridResizePreview => true,
            _ => false
        }
    }
    pub fn element_name(&self) -> &str {
        match self {
            Self::GridResize => "module-page-grid-resize",
            Self::GridResizeScrollable => "module-page-grid-resize",
            Self::GridResizePreview => "module-page-grid-resize",
            Self::GridPlain => "module-page-grid-plain",
            Self::Iframe => "module-page-iframe",
            Self::Empty => "div"
        }
    }
}

/*
pub trait ModuleRenderer <State> {
    type PageKindSignal: Signal<Item = ModulePageKind>;
    type ChildrenSignal: SignalVec<Item = Dom>;

    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal;

    fn children_signal(state: Rc<State>, kind: ModulePageKind) -> Self::ChildrenSignal;
}

// The page renderer
pub struct ModulePage<Renderer, Loader, RawData, State> 
where
    Renderer: ModuleRenderer<State>,
    RawData: DeserializeOwned,
    Loader: StateLoader<RawData, State>,
{
    wait_iframe_data: bool,
	renderer: Renderer,
    loader: Loader,
    async_loader: AsyncLoader,
    async_switcher: AsyncLoader,
    phantom: PhantomData<(RawData, State)>
}

impl <Renderer, Loader, RawData, State> ModulePage <Renderer, Loader, RawData, State> 
where
    Renderer: ModuleRenderer<State> + 'static,
    Loader: StateLoader<RawData, State> + 'static,
    RawData: DeserializeOwned + 'static,
    State: 'static,
{

    pub fn render(renderer:Renderer, loader: Loader) -> Rc<Self> 
    {

        let wait_iframe_data = should_get_iframe_data();

        let _self = Rc::new(Self { 
			renderer,
            loader,
            async_loader: AsyncLoader::new(),
            async_switcher: AsyncLoader::new(), 
            wait_iframe_data,
            phantom: PhantomData
        });

        let _self_clone = _self.clone();

        if !wait_iframe_data {
            _self_clone.async_loader.load(async move {
                if let Some(state) = _self.loader.load_state().await {
                    Self::render_data(_self, state);
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
        //This is just a placeholder to get messages
        //It'll be replaced when the iframe data arrives
        let dom = html!("empty-fragment", {
            .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {
                //Get iframe data if we're supposed to
                if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                    log::info!("got iframe data!");

                    if !_self.wait_iframe_data {
                        //log::warn!("weird... shouldn't have gotten iframe data!");
                        //log::warn!("{:?}", msg);
                    } else {
                        let raw_data = msg.data.expect_ji("couldn't decode iframe data");
                        let state = _self.loader.derive_state(raw_data);
                        Self::render_data(_self.clone(), state);
                    }
                } else {
                    log::info!("hmmm got other iframe message...");
                }
            }))
            .after_inserted(clone!(_self => move |elem| {
                if _self.wait_iframe_data {
                    let parent = web_sys::window()
                        .unwrap_ji()
                        .parent()
                        .unwrap_ji()
                        .unwrap_ji();
                    //On mount - send an empty IframeInit message to let the parent know we're ready
                    let msg = IframeInit::empty();

                    parent.post_message(&msg.into(), "*");
                }
            }))
        });
        Self::switch_body(dom); 
    }

    fn render_data(_self: Rc<Self>, state: Rc<State>) {

        _self.async_switcher.load(
            Renderer::page_kind_signal(state.clone())
                .for_each(clone!(state => move |page_kind| {
                    let has_resized_once = Mutable::new(false);

                    let dom = html!(page_kind.element_name(), {
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
                        .children_signal_vec(has_resized_once.signal_cloned().switch_signal_vec(clone!(state, page_kind => move |has_resized_once| {
                            Renderer::children_signal(state.clone(), page_kind)
                                .filter(clone!(has_resized_once => move |_| has_resized_once))
                        })))
                    });

                    Self::switch_body(dom); 
                    async {}
                }))
        );
    }
}
*/
