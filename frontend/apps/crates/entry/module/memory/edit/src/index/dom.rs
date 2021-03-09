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
use super::{state::*, actions::*};

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
    type ChildrenSignal = impl SignalVec<Item = ModuleDom>;


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

impl PageRenderer {
    fn sidebar(state: Rc<crate::data::State>) -> Option<ModuleDom> {
        state.game_mode.get()
            .map(|game_mode| {
                ModuleDom::Sidebar(Box::new(move |mixin:HtmlMixinPtr| {
                    html!("div", {
                        .apply(|dom| mixin(dom))
                        .text("duplicate!")
                    })
                }))
            })
    }

    fn header(state: Rc<crate::data::State>) -> Option<ModuleDom> { 
        state.game_mode.get()
            .map(|game_mode| {
                ModuleDom::Header(Box::new(move |mixin:HtmlMixinPtr| {
                    html!("div", {
                        .apply(|dom| mixin(dom))
                    })
                }))
            })
    }

    fn main(state: Rc<crate::data::State>) -> Option<ModuleDom> { 
            Some(ModuleDom::Main(Box::new(move |mixin:HtmlMixinPtr| {
                match state.game_mode.get() {
                    None => {
                        crate::choose::dom::ChooseDom::render(state)
                    },
                    Some(mode) => {
                        html!("div", {
                            .apply(|dom| mixin(dom))
                        })
                    }
                }
            })))
    }

    fn footer(state: Rc<crate::data::State>) -> Option<ModuleDom> { 
        state.game_mode.get()
            .map(|game_mode| {
                ModuleDom::Footer(Box::new(move |mixin:HtmlMixinPtr| {
                    html!("div", {
                        .apply(|dom| mixin(dom))
                    })
                }))
            })
    }
}
