use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use components::module::page::*;
use super::{sidebar, main::MainDom, header, footer};


type LoadedData = (String, String, raw::Poster);

pub fn render(jig_id: String, module_id: String) -> Dom {
    ModulePage::<EditorRenderer, _>::render(move || async move {
        if let Some(raw_data) = debug::settings().poster {
            (jig_id, module_id, raw_data)
        } else {
            let raw_data = raw::Poster::load(jig_id.clone(), module_id.clone()).await;
            (jig_id, module_id, raw_data)
        }
    })
}

struct EditorRenderer {
    pub state: Rc<State>
}

impl ModuleRenderer for EditorRenderer {
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
        always(ModulePageKind::EditResize)
    }

    fn sidebar_signal(_self: Rc<Self>) -> Self::SidebarSignal {
        always(Some(sidebar::render(_self.state.clone())))
    }
    fn header_signal(_self: Rc<Self>) -> Self::HeaderSignal {
        always(Some(header::render(_self.state.clone())))
    }
    fn footer_signal(_self: Rc<Self>) -> Self::FooterSignal {
        always(Some(footer::render(_self.state.clone())))
    }
    fn main_signal(_self: Rc<Self>) -> Self::MainSignal {

        always(Some(MainDom::render(MainDom::new(_self.state.clone()))))
    }
}
