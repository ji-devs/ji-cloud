use crate::base::state::*;
use components::stickers::dom::TransformOverride;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;
use std::rc::Rc;
use utils::drag::Drag;

pub struct MainSelect {
    pub base: Rc<Base>,
    pub items: Vec<SelectItem>,
}

impl MainSelect {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let items = base
            .stickers
            .list
            .lock_ref()
            .iter()
            .enumerate()
            .map(|(index, item)| SelectItem {
                item: item.clone(),
                index,
                drag: Mutable::new(None),
                base: base.clone(),
            })
            .collect();

        Rc::new(Self { base, items })
    }

    pub fn item_kinds(&self) -> impl SignalVec<Item = (usize, ItemKind)> {
        self.base
            .stickers
            .list
            .signal_vec_cloned()
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
        self.base
            .drag_item_selected_index
            .signal_cloned()
            .map(clone!(index => move |selected| {
                match selected {
                    None => false,
                    Some(i) => i == index
                }
            }))
    }
}

#[derive(Clone)]
pub struct SelectItem {
    pub item: Item,
    pub index: usize,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub base: Rc<Base>,
}

impl SelectItem {
    pub fn raw_sticker(&self) -> RawSticker {
        self.item.sticker.to_raw()
    }

    pub fn kind_signal_cloned(&self) -> impl Signal<Item = ItemKind> {
        self.item.kind.signal_cloned()
    }

    pub fn get_transform_override(&self) -> TransformOverride {
        TransformOverride::Always(
            self.item
                .sticker
                .transform()
                .get_inner_mutable()
                .read_only(),
        )
    }
}
