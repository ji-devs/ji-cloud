use dominator::{html, Dom, clone};
use components::module::page::*;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{self, MutableVec, SignalVec, SignalVecExt}
};
use wasm_bindgen::prelude::*;
use utils::prelude::*;
use crate::{
    data::{state::*, actions, raw::ModuleData as RawData},
    steps,
    overlay,
};
use shared::domain::jig::{JigId, module::ModuleId};
use super::loader::*;
pub type Page = Rc<ModulePage<PageRenderer, PageLoader, RawData, State>>;

pub struct IndexDom {}
impl IndexDom {
    pub fn render(jig_id: JigId, module_id: ModuleId) -> Page {
        ModulePage::<PageRenderer, PageLoader, RawData, State>::render(
            PageRenderer{},
            PageLoader{jig_id, module_id}
        )
    }
}



pub struct PageRenderer { 
}

impl ModuleRenderer<State> for PageRenderer {
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type ChildrenSignal = impl SignalVec<Item = Dom>;


    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal {
        state.page_kind_signal()
    }

    fn children_signal(state: Rc<State>, kind:ModulePageKind) -> Self::ChildrenSignal {
        signal_vec::always(
                vec![
                    Self::sidebar(state.clone(), kind),
                    Self::header(state.clone(), kind),
                    Self::main(state.clone(), kind),
                    Self::footer(state.clone(), kind),
                    Self::overlay(state.clone(), kind),
                ]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap_ji())
                .collect()
        )
    }
}

/*
 * if mode is None, then it's the Choose page - and it's only main
 * otherwise it's the Steps sections
 */
impl PageRenderer {
    fn sidebar(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            None
        } else {
            Some(steps::sidebar::dom::SidebarDom::render(state))
        }
    }

    fn header(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            Some(steps::header::dom::HeaderPreviewDom::render(state))
        } else {
            Some(steps::header::dom::HeaderDom::render(state))
        }
    }

    fn main(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> {

        Some(
                if kind == ModulePageKind::GridResizePreview {
                    steps::preview::dom::PreviewDom::render(state)
                } else {
                    steps::main::dom::MainDom::render(state)
                }
        )
    }

    fn footer(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            None
        } else {
            Some(steps::footer::dom::FooterDom::render(state))
        }
    }
    fn overlay(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> {
        Some(overlay::dom::OverlayDom::render(state))
    }
}
