use crate::base::state::*;
use std::rc::Rc;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{SignalVec, SignalVecExt}
};

pub struct MainSelect {
    pub base: Rc<Base>,
}

impl MainSelect {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        Rc::new(Self {
            base,
        })
    }

    pub fn item_kinds(&self) -> impl SignalVec<Item = (usize, ItemKind)> {
        self.base.stickers.list.signal_vec_cloned()
            .enumerate()
            .map_signal(|(index, item)| {
                map_ref! {
                    let index = index.signal(),
                    let kind = item.kind.signal_cloned()
                        => (index.unwrap_or_default(), kind.clone())
                }
            })
    }

    pub fn is_selected(&self, index: usize) -> impl Signal<Item = bool> {
        self.base.drag_item_selected_index.signal_cloned()
            .map(clone!(index => move |selected| {
                match selected {
                    None => false,
                    Some(i) => i == index
                }
            }))
    }
}

