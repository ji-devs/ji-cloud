use crate::base::state::*;

use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;
use std::rc::Rc;
use utils::drag::Drag;

use components::stickers::dom::TransformOverride;
use futures_signals::signal::Mutable;

pub struct MainDrag {
    pub base: Rc<Base>,
    pub items: Vec<DragItem>,
}

impl MainDrag {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let items = base
            .stickers
            .list
            .lock_ref()
            .iter()
            .enumerate()
            .map(|(index, item)| DragItem {
                item: item.clone(),
                index,
                drag: Mutable::new(None),
                base: base.clone(),
            })
            .collect();

        Rc::new(Self { base, items })
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
            _ => false,
        }
    }

    pub fn get_transform_override(&self) -> TransformOverride {
        TransformOverride::Sometimes(
            self.item
                .get_interactive_unchecked()
                .target_transform
                .read_only(),
        )
    }
}
