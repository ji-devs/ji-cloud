use dominator::{html, Dom, clone};
use components::module::page::*;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use wasm_bindgen::prelude::*;
use utils::events;
use crate::{
    data::{state::*, raw::GameData as RawData},
    steps,
    choose
};
use super::state::*;
pub type Page = Rc<ModulePage<PageRenderer, PageLoader, RawData, LocalState>>;

pub struct IndexDom {}
impl IndexDom {
    pub fn render(jig_id: String, module_id: String) -> Page {
        ModulePage::<PageRenderer, PageLoader, RawData, LocalState>::render(
            PageRenderer{},
            PageLoader{jig_id, module_id}
        )
    }
}



pub struct PageRenderer { 
}

impl ModuleRenderer<LocalState> for PageRenderer {
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type ChildrenSignal = impl SignalVec<Item = Dom>;


    fn page_kind_signal(state: Rc<LocalState>) -> Self::PageKindSignal {
        state.page_kind_signal()
    }

    fn children_signal(state: Rc<LocalState>, kind:ModulePageKind) -> Self::ChildrenSignal {
        state.data
            .signal_cloned()
            .map(clone!(state => move |raw_data| {
                log::info!("{:?}", kind);
                let state = Rc::new(State::new(state.clone(), raw_data));
                vec![
                    Self::sidebar(state.clone(), kind),
                    Self::header(state.clone(), kind),
                    Self::main(state.clone(), kind),
                    Self::footer(state.clone(), kind),
                ]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap_throw())
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
    fn sidebar(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> {
        if kind == ModulePageKind::GridResizePreview {
            None
        } else {
            state.game_mode.get()
                .map(|game_mode| {
                    steps::sidebar::dom::SidebarDom::render(state)
                })
        }
    }

    fn header(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> { 
        if kind == ModulePageKind::GridResizePreview {
            Some(steps::header::dom::HeaderPreviewDom::render(state))
        } else {
            state.game_mode.get()
                .map(|game_mode| {
                    steps::header::dom::HeaderDom::render(state)
                })
        }
    }

    fn main(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> { 
        Some(match state.game_mode.get() {
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

    fn footer(state: Rc<State>, kind: ModulePageKind) -> Option<Dom> { 
        if kind == ModulePageKind::GridResizePreview {
            None
        } else {
            state.game_mode.get()
                .map(|game_mode| {
                    steps::footer::dom::FooterDom::render(state)
                })
        }
    }
}
