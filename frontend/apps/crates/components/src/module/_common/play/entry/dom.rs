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
use crate::module::_common::play::prelude::*;
use shared::domain::jig::module::body::{ModeExt, BodyExt, StepExt};

pub fn render_page_body<RawData, Mode, Step, Base> (state:Rc<GenericState<RawData, Mode, Step, Base>>)
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{

    let sig =
            state.phase.signal_cloned().map(clone!(state => move |phase| {
                let page_kind = match phase.as_ref() {
                    Phase::Init | Phase::WaitingIframeRaw(_) => ModulePageKind::GridPlain,
                    Phase::Playing(_, _) => ModulePageKind::Iframe
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
                                            Phase::WaitingIframeRaw(on_raw) => {
                                                vec![render_iframe_wait_raw(state.clone(), on_raw.clone())]
                                            },
                                            Phase::Playing(base, raw_direct) => {
                                                vec![render_player(state.clone(), base.clone(), *raw_direct)]
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


//This is just a placeholder to get messages
//It'll be replaced when the iframe data arrives
fn render_iframe_wait_raw<RawData, Mode, Step, Base> (state:Rc<GenericState<RawData, Mode, Step, Base>>, on_raw: Rc<Box<dyn Fn(RawData)>>) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static
{

    html!("empty-fragment", {
        .global_event(clone!(state, on_raw => move |evt:dominator_helpers::events::Message| {
            if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                log::info!("got iframe data!");
                //on_raw was stashed from the original State::new()
                on_raw(msg.data);
            } else {
                log::info!("hmmm got other iframe message...");
            }
        }))
        .after_inserted(clone!(state => move |elem| {
            let parent = web_sys::window()
                .unwrap_ji()
                .parent()
                .unwrap_ji()
                .unwrap_ji();
            //On mount - send an empty IframeInit message to let the parent know we're ready
            let msg = IframeInit::empty();

            parent.post_message(&msg.into(), "*");
        }))
    })

}

fn render_player<RawData, Mode, Step, Base> (state:Rc<GenericState<RawData, Mode, Step, Base>>, base: Rc<Base>, raw_direct: bool) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static
{

    html!("empty-fragment", {
        .property("slot", "main")
        .child(Base::render(base.clone()))
        //raw_direct generally means "preview window"
        //and we already got the init event with the raw data
        //plus there's no jig player to send more messages
        //so bypass the bootstrapping cycle
        .apply_if(!raw_direct, |dom| {
            dom
                .global_event(clone!(state, base => move |evt:dominator_helpers::events::Message| {
                    if let Ok(msg) = evt.try_serde_data::<IframeAction<JigToModuleMessage>>() {
                        match msg.data {
                            JigToModuleMessage::Play => {
                                state.get_audio_mixer().play_all();
                            },
                            JigToModuleMessage::Pause => {
                                state.get_audio_mixer().pause_all();
                            },
                            JigToModuleMessage::TimerDone => {
                            }
                        }
                    } else {
                        log::info!("hmmm got other iframe message...");
                    }
                }))
                .after_inserted(clone!(state => move |elem| {
                    let parent = web_sys::window()
                        .unwrap_ji()
                        .parent()
                        .unwrap_ji()
                        .unwrap_ji();
                    //On mount - send an empty IframeInit message to let the parent know we're ready
                    let msg = IframeInit::empty();

                    parent.post_message(&msg.into(), "*");
                }))
        })
    })

}

