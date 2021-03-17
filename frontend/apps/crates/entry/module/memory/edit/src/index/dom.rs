use dominator::{html, Dom, clone};
use components::module::page::*;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use wasm_bindgen::prelude::*;
use utils::events;
use crate::data::raw::GameData as RawData;
use super::state::*;

pub type Page = Rc<ModulePage<PageRenderer, PageLoader, RawData, State>>;

pub struct IndexDom {}
impl IndexDom {
    pub fn render(jig_id: String, module_id: String) -> Page {
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
        state.data
            .signal_cloned()
            .map(clone!(state => move |raw_data| {
                let state = Rc::new(crate::data::State::new(state.clone(), raw_data));
                vec![
                    Self::sidebar(state.clone()),
                    Self::header(state.clone()),
                    Self::main(state.clone()),
                    Self::footer(state.clone()),
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
    fn sidebar(state: Rc<crate::data::State>) -> Option<Dom> {
        state.game_mode.get()
            .map(|game_mode| {
                crate::steps::sidebar::dom::SidebarDom::render(state)
            })
    }

    fn header(state: Rc<crate::data::State>) -> Option<Dom> { 
        state.game_mode.get()
            .map(|game_mode| {
                html!("div", { 
                    .text("header here!")
                    .property("slot", "header")
                })
            })
    }

    fn main(state: Rc<crate::data::State>) -> Option<Dom> { 
        Some(match state.game_mode.get() {
            None => {
                crate::choose::dom::ChooseDom::render(state)
            },
            Some(mode) => {
                html!("div", {
                    .text("main here!")
                    .property("slot", "main")
                })
            }
        })
    }

    fn footer(state: Rc<crate::data::State>) -> Option<Dom> { 
        state.game_mode.get()
            .map(|game_mode| {
                html!("div", { 
                    .text("footer here!")
                    .property("slot", "footer")
                })
            })
    }
}
