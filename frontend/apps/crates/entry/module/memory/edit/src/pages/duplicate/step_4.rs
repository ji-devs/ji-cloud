use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlIFrameElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, dynamic_class_signal, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use std::fmt::Write;
use crate::data::{*, raw::*};
use itertools::Itertools;
use crate::config;
use utils::settings::SETTINGS;
use shared::domain::jig::ModuleKind;
use utils::{
    iframe::*,
    routes::{Route, ModuleRoute}
};

pub struct Step4Page {
    state: Rc<DuplicateState>,
}

impl Step4Page {
    pub fn new(state:Rc<DuplicateState>) -> Rc<Self> {

        let _self = Rc::new(Self { 
            state,
        });

        _self
    }

    fn iframe_url(&self) -> String {
        let route:String = Route::Module(ModuleRoute::Play(ModuleKind::MemoryGame, self.state.jig_id.clone(), self.state.module_id.clone())).into();

        let url = unsafe {
            SETTINGS.get_unchecked()
            .remote_target
            .spa_iframe(&route)
        };

        format!("{}?iframe_data=true", url)
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::duplicate::step_4_page(), { 
            .with_data_id!("top-step-1", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::One);
                }))
            })
            .with_data_id!("top-step-2", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.state.step.set(Step::Two);
                }))
            })
            .with_data_id!("jig-module-iframe" => HtmlIFrameElement, {
                .property("src", &_self.iframe_url())
                .with_node!(elem => {
                    .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {

                        if let Ok(_) = evt.try_serde_data::<IframeInit<()>>() {
                            //Iframe is ready and sent us a message, let's send one back!
                            let data:GameStateRaw = (&*_self.state).into();
                            let msg:IframeInit<GameStateRaw> = IframeInit::new(data); 
                            let window = elem.content_window().unwrap_throw();
                            window.post_message(&msg.into(), &_self.iframe_url());
                        } else {
                            log::info!("hmmm got other iframe message...");
                        }
                    }))
                    
                })

            })
        })
    }
}


