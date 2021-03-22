use dominator::{html, Dom, clone};
use components::module::page::*;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt, self},
    signal_vec::{MutableVec, SignalVec, SignalVecExt}
};
use wasm_bindgen::prelude::*;
use utils::events;
use crate::{
    data::{state::State, raw::GameData as RawData},
};
use super::state::*;
use crate::player::dom::PlayerDom;

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
        signal::always(ModulePageKind::Iframe)
    }

    fn children_signal(state: Rc<LocalState>, kind:ModulePageKind) -> Self::ChildrenSignal {
        state.data
            .signal_cloned()
            .map(clone!(state => move |raw_data| {
                let state = Rc::new(State::new(state.clone(), raw_data));
                vec![
                    PlayerDom::render(state) 
                ]
            }))
            .to_signal_vec()
    }
}

