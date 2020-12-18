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
use dominator_helpers::{elem, with_data_id,futures::{spawn_future, AsyncLoader}};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use super::templates;
use components::module::page::*;
use std::pin::Pin;

const INITIAL_MODE:ModulePageKind = ModulePageKind::GridResize;
pub type Page = Rc<ModulePage<PageRenderer, RawData, State>>;

pub fn render() -> Page {
    ModulePage::<PageRenderer, RawData, State>::render(|| async {
    })
}

pub type RawData = ();

pub struct State {
    pub kind: Mutable<ModulePageKind>
}
impl State {
    fn new(data:RawData) -> Self {
        Self { 
            kind: Mutable::new(INITIAL_MODE) 
        }
    }
}

pub struct PageRenderer { 
}

impl ModuleRenderer<RawData, State> for PageRenderer {
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type SidebarSignal = impl Signal<Item = Option<Dom>>;
    type HeaderSignal = impl Signal<Item = Option<Dom>>;
    type MainSignal = impl Signal<Item = Option<Dom>>;
    type FooterSignal = impl Signal<Item = Option<Dom>>;

    fn derive_state(data:RawData) -> State { 
        State::new(data)
    }

    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal {
        state.kind.signal()
    }

    fn sidebar_signal(state: Rc<State>, kind:ModulePageKind) -> Self::SidebarSignal {
        state.kind.signal().map(move |kind| {

            templates::sidebar(kind).map(|el| {
                elem!(el, {
                    .child(html!("div", {
                        .style("display", "flex")
                        .children(vec![
                            html!("button", {
                                .text("empty")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::Empty);
                                }))
                            }),
                            html!("button", {
                                .text("edit-plain")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::GridPlain);
                                }))
                            }),
                            html!("button", {
                                .text("edit-resize")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::GridResize);
                                }))
                            }),
                            html!("button", {
                                .text("edit-resize-scrollable")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::GridResizeScrollable);
                                }))
                            }),
                            html!("button", {
                                .text("iframe")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::Iframe);
                                }))
                            }),
                        ])
                    }))
                })
            })
        })
    }
    fn header_signal(state: Rc<State>, kind: ModulePageKind) -> Self::HeaderSignal { 
        state.kind.signal().map(|kind| {
            templates::header(kind).map(|el| {
                elem!(el, {})
            })
        })
    }

    fn main_signal(state: Rc<State>, kind: ModulePageKind) -> Self::MainSignal { 
        state.kind.signal().map(|kind| {
            templates::main(kind).map(|el| {
                elem!(el, {})
            })
        })
    }
    fn footer_signal(state: Rc<State>, kind: ModulePageKind) -> Self::FooterSignal { 
        state.kind.signal().map(|kind| {
            templates::footer(kind).map(|el| {
                elem!(el, {})
            })
        })
    }
}
