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
use crate::module::_common::edit::prelude::*;
use super::base::state::*;
use shared::domain::jig::module::body::{ModeExt, BodyExt, StepExt};

pub fn render_page_body<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay> (state:Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>)
where
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static, 
{
    let sig = map_ref! {
        let phase = state.phase.signal_cloned(),
        let is_preview = state.is_preview_signal()
            => {
                (phase.clone(), *is_preview)
            }
    };

    let sig =
            sig.map(clone!(state => move |(phase, is_preview)| {
                let page_kind = {
                    match phase.as_ref() {
                        Phase::Init | Phase::Choose(_) => ModulePageKind::GridPlain,
                        Phase::Base(_) => {
                            if is_preview {
                                ModulePageKind::GridResizePreview
                            } else if state.opts.is_main_scrollable {
                                ModulePageKind::GridResizeScrollable
                            } else {
                                ModulePageKind::GridResize
                            }
                        },
                    }
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
                                            Phase::Choose(choose) => {
                                                super::choose::dom::render(choose.clone())
                                            },
                                            Phase::Base(app_base) => {
                                                super::base::dom::render(
                                                    is_preview,
                                                    state.opts.jig_id.clone(), 
                                                    state.opts.module_id.clone(), 
                                                    app_base.clone()
                                                )
                                            },
                                            Phase::Init => {
                                                vec![super::init::dom::render(state.clone())]
                                            }
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
