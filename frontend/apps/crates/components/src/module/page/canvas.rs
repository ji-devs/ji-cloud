/* Not being used anywhere because the types are crazy
    And I couldn't figure out how to get the return type of RenderPage below.
    It would be something _like_ this:

    type RenderPage = 
        Rc<
            ModulePage<
                CanvasPage<
                    (), 
                    Foo, 
                    impl FnMut(HtmlCanvasElement, ()) -> Rc<Foo>,
                    impl FnOnce() -> impl Future<Output = ()>,
                    impl Future<Output = ()>,
                >, 
                (), 
                CanvasState<
                    (), 
                    Foo, 
                    impl FnMut(HtmlCanvasElement, ()) -> Rc<Foo>,
                >
            >
        >;

    That said, this example would otherwise work:

*/
/*
use super::page::*;
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
use web_sys::{Url, HtmlElement, HtmlCanvasElement, Element, HtmlInputElement};
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
use super::load::StateLoader;
use super::page::*;

/*
fn render() -> RenderPage {
    CanvasPage::render(
        //Load whatever data
        || async {
            ()
        },
        //create an app from the data
        |canvas, data| {
            Rc::new(Foo::new(canvas, data))
        }
    )
}

struct Foo {
    canvas: HtmlCanvasElement,
    data: ()
}
impl Foo {
    fn new(canvas:HtmlCanvasElement, data: ()) -> Self {
        Self {canvas, data}
    }
}
impl CanvasResize for Foo {
    fn resize(&self, size:ResizeInfo) {
    }
}
*/

pub trait CanvasResize {
    fn resize(&self, size:ResizeInfo);
}

pub struct CanvasState<RawData, App, F> 
where
    F: FnMut(HtmlCanvasElement, RawData) -> Rc<App>,
    App: CanvasResize
{
    app: RefCell<Option<Rc<App>>>,
    raw_data: RefCell<Option<RawData>>,
    module_size: Mutable<ResizeInfo>,
    get_app: RefCell<F>,
}

impl <RawData, App, GetApp> CanvasState<RawData, App, GetApp> 
where
    GetApp: FnMut(HtmlCanvasElement, RawData) -> Rc<App>,
    App: CanvasResize

{
    pub fn new(get_app: GetApp) -> Self {
        Self {
            app: RefCell::new(None),
            raw_data: RefCell::new(None),
            module_size: get_resize_info(),
            get_app: RefCell::new(get_app)
        }
    }
}

impl <RawData, App, GetApp> CanvasState<RawData, App, GetApp> 
where
    GetApp: FnMut(HtmlCanvasElement, RawData) -> Rc<App>,
    App: CanvasResize
{
    pub fn stash_app(&self, canvas:HtmlCanvasElement) {

        let mut f = self.get_app.borrow_mut();
        let app = f(canvas, self.raw_data.take().unwrap_throw());
        *self.app.borrow_mut() = Some(app);
    }
    pub fn resize_app(&self, size:ResizeInfo) {
        log::info!("{:?}", size);
        if let Some(app) = self.app.borrow().as_ref() {
            app.resize(size);
        }
    }
}


pub struct CanvasPage<RawData, App, GetApp, RawDataLoader, RawDataFut> 
where
    GetApp: FnMut(HtmlCanvasElement, RawData) -> Rc<App>,
    App: CanvasResize,
    RawDataLoader: FnOnce() -> RawDataFut,
    RawDataFut: Future<Output = RawData>
{
    state: Rc<CanvasState<RawData, App, GetApp>>,
    data_loader: RefCell<Option<RawDataLoader>>
}

impl <RawData, App, GetApp, RawDataLoader, RawDataFut> CanvasPage<RawData, App, GetApp, RawDataLoader, RawDataFut>
where
    RawData: DeserializeOwned + 'static,
    GetApp: FnMut(HtmlCanvasElement, RawData) -> Rc<App> + 'static,
    App: CanvasResize + 'static,
    RawDataLoader: FnOnce() -> RawDataFut + 'static,
    RawDataFut: Future<Output = RawData> + 'static
{
    pub fn render(data_loader: RawDataLoader, get_app: GetApp) -> Rc<ModulePage<CanvasPage<RawData, App, GetApp, RawDataLoader, RawDataFut>, RawData, CanvasState<RawData, App, GetApp>>> {
        ModulePage::<CanvasPage<RawData, App, GetApp, RawDataLoader, RawDataFut>, RawData, CanvasState<RawData, App, GetApp>>::render(Self {
            state: Rc::new(CanvasState::new(get_app)),
            data_loader: RefCell::new(Some(data_loader))
        })
    }
}

impl <RawData, App, GetApp, RawDataLoader, RawDataFut>  StateLoader<RawData, CanvasState<RawData, App, GetApp>> for CanvasPage<RawData, App, GetApp, RawDataLoader, RawDataFut> 
where
    GetApp: FnMut(HtmlCanvasElement, RawData) -> Rc<App>,
    App: CanvasResize,
    RawDataLoader: FnOnce() -> RawDataFut,
    RawDataFut: Future<Output = RawData>

{
    type FutureState = impl Future<Output = Option<Rc<CanvasState<RawData, App, GetApp>>>>;

    fn load_state(&self) -> Self::FutureState {
        let state = self.state.clone();
        let data_loader = self.data_loader.borrow_mut().take().unwrap();
        async move {
            let data = data_loader().await;
            *state.raw_data.borrow_mut() = Some(data);
            Some(state)
        }
    }
    fn derive_state(&self, data: RawData) -> Rc<CanvasState<RawData, App, GetApp>> {
        *self.state.raw_data.borrow_mut() = Some(data);
        self.state.clone()
    }
}

impl <RawData, App, GetApp, RawDataLoader, RawDataFut>  ModuleRenderer<CanvasState<RawData, App, GetApp>> for CanvasPage<RawData, App, GetApp, RawDataLoader, RawDataFut>

where
    RawData: 'static,
    GetApp: FnMut(HtmlCanvasElement, RawData) -> Rc<App> + 'static,
    App: CanvasResize + 'static,
    RawDataLoader: FnOnce() -> RawDataFut + 'static,
    RawDataFut: Future<Output = RawData> + 'static
{
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type ChildrenSignal = impl SignalVec<Item = ModuleDom>;

    fn page_kind_signal(state: Rc<CanvasState<RawData, App, GetApp>>) -> Self::PageKindSignal { 
        always(ModulePageKind::Iframe)
    }

    fn children_signal(state: Rc<CanvasState<RawData, App, GetApp>>, kind: ModulePageKind) -> Self::ChildrenSignal {
        always(vec![
            ModuleDom::Main(Box::new(move |mixin:HtmlMixinPtr| {
                html!("div", {
                    .future(
                        state.module_size.signal_cloned().for_each(clone!(state => move |size| {
                            state.resize_app(size);
                            async {}
                        }))
                     )
                    .style("width", "100%")
                    .style("height", "100%")
                    .apply(|dom| mixin(dom))
                    .child(html!("canvas" => HtmlCanvasElement, {
                        .after_inserted(clone!(state => move |dom| {
                            state.stash_app(dom);
                        }))
                    }))
                })
            }))

        ]).to_signal_vec()
    }
}
*/