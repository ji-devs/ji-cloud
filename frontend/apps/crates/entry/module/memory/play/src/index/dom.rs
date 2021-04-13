use dominator::{html, Dom, clone};
use components::module::page::*;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt, self},
    signal_vec::{MutableVec, SignalVec, SignalVecExt, self}
};
use wasm_bindgen::prelude::*;
use utils::events;
use crate::{
    data::{state::State, raw::GameData as RawData},
};
use super::loader::*;
use shared::domain::jig::{JigId, module::ModuleId};
use crate::player::dom::PlayerDom;

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
        signal::always(ModulePageKind::Iframe)
    }

    fn children_signal(state: Rc<State>, kind:ModulePageKind) -> Self::ChildrenSignal {
        signal_vec::always(vec![PlayerDom::render(state)])
    }
}
