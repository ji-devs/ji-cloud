use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal, always, self},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{Url, HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem,dynamic_class_signal ,with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use utils::{
    iframe::*,
    settings::SETTINGS,
};
use gloo_timers::future::TimeoutFuture;
use crate::{debug, data::*};
use std::future::Future;
use async_trait::async_trait;
use std::{
    pin,
    future,
    marker
};
use dominator::animation::{easing, Percentage, MutableAnimation, AnimatedMapBroadcaster};
use shared::{
    api::endpoints::{ApiEndpoint, self},
    domain,
    error,
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};

use uuid::Uuid;
use components::{
    image::data::*,
    module::page::*
};

type LoadedData = (String, String, raw::Poster);

pub fn render(jig_id: String, module_id: String) -> Dom {
    ModulePage::<PlayerRenderer, _>::render(move || async move {
        if let Some(raw_data) = debug::settings().poster {
            (jig_id, module_id, raw_data)
        } else {
            let raw_data = raw::Poster::load(jig_id.clone(), module_id.clone()).await;
            (jig_id, module_id, raw_data)
        }
    })
}



struct PlayerRenderer {
    pub state: Rc<State>
}

impl ModuleRenderer for PlayerRenderer {
    type Data = LoadedData;
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type SidebarSignal = impl Signal<Item = Option<Dom>>;
    type HeaderSignal = impl Signal<Item = Option<Dom>>;
    type MainSignal = impl Signal<Item = Option<Dom>>;
    type FooterSignal = impl Signal<Item = Option<Dom>>;

    fn new((jig_id, module_id, raw_data):LoadedData) -> Self {
        Self { 
            state: State::new(jig_id, module_id, raw_data)
        }
    }


    fn page_kind_signal(_self: Rc<Self>) -> Self::PageKindSignal {
        always(ModulePageKind::PlayIframe)
    }

    fn sidebar_signal(_self: Rc<Self>) -> Self::SidebarSignal { 
        always(None)
    }
    fn header_signal(_self: Rc<Self>) -> Self::HeaderSignal { 
        always(None)
    }
    fn footer_signal(_self: Rc<Self>) -> Self::FooterSignal { 
        always(None)
    }

    fn main_signal(_self: Rc<Self>) -> Self::MainSignal { 

        always(Some(html!("h1", { .text("hello world") })))
    }

}