use dominator::{html, Dom, clone};
use components::module::page::*;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use wasm_bindgen::prelude::*;
use utils::prelude::*;
use crate::{
    data::{state::*, actions, raw::ModuleData as RawData},
    steps,
    choose
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
        state.game_mode
            .signal()
            .map(clone!(state => move |game_mode| {
                vec![
                    Self::sidebar(state.clone(), game_mode, kind),
                    Self::header(state.clone(), game_mode, kind),
                    Self::main(state.clone(), game_mode, kind),
                    Self::footer(state.clone(), game_mode, kind),
                ]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap_ji())
                .collect()
            }))
            .to_signal_vec()
    }
}

/*
 * if game_mode is None, then it's the Choose page - and it's only main
 * otherwise it's the Steps sections
 */
impl PageRenderer {
    fn sidebar(state: Rc<State>, game_mode: Option<GameMode>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            None
        } else {
            game_mode
                .map(|game_mode| {
                    steps::sidebar::dom::SidebarDom::render(state)
                })
        }
    }

    fn header(state: Rc<State>, game_mode: Option<GameMode>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            Some(steps::header::dom::HeaderPreviewDom::render(state))
        } else {
            game_mode.map(|_| steps::header::dom::HeaderDom::render(state))
        }
    }

    fn main(state: Rc<State>, game_mode: Option<GameMode>, kind: ModulePageKind) -> Option<Dom> {

        Some(match game_mode {
            None => {
                choose::dom::ChooseDom::render(state)
            },
            Some(mode) => {
                if kind == ModulePageKind::GridResizePreview {
                    steps::preview::dom::PreviewDom::render(state)
                } else {
                    steps::main::dom::MainDom::render(state)
                }
            }
        })
    }

    fn footer(state: Rc<State>, game_mode: Option<GameMode>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            None
        } else {
            game_mode
                .map(|game_mode| {
                    steps::footer::dom::FooterDom::render(state)
                })
        }
    }
}
