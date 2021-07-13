use crate::base::state::*;
use dominator_helpers::signals::{DefaultSignal, OptionSignal};
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;
use utils::{drag::Drag, math::PointI32};
use std::rc::Rc;
use dominator::clone;
use futures_signals::{map_ref, signal::{Mutable, ReadOnlyMutable, Signal, SignalExt}, signal_vec::{SignalVec, SignalVecExt}};

pub struct MainDrag {
    pub base: Rc<Base>,
    pub items: Vec<DragItem>,
}

impl MainDrag {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let items = base.stickers.list.lock_ref()
            .iter()
            .enumerate()
            .map(|(index, item)| {
                DragItem {
                    item: item.clone(),
                    index,
                    drag: Mutable::new(None),
                    base: base.clone(),
                }
            })
            .collect();

        Rc::new(Self {
            base,
            items
        })
    }
}

#[derive(Clone)]
pub struct DragItem {
    pub item: Item,
    pub index: usize,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub base: Rc<Base>,
}

impl DragItem {
    pub fn raw_sticker(&self) -> RawSticker {
        self.item.sticker.to_raw()
    }

    pub fn get_is_interactive(&self) -> bool {
        match &*self.item.kind.lock_ref() {
            ItemKind::Interactive(_) => true,
            _ => false
        }

    }

    pub fn get_offset_mutable(&self) -> ReadOnlyMutable<(f64, f64)> {
        self.item.get_interactive_unchecked().target_offset.read_only()
    }
}
