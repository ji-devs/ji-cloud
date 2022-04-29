use crate::base::state::*;

use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;
use std::rc::Rc;
use utils::drag::Drag;

use components::stickers::dom::TransformOverride;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};

pub struct MainDrag {
    pub base: Rc<Base>,
    pub items: Vec<DragItem>,
    pub placed_items: MutableVec<DragItem>,
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
                item: Item {
                    // Recreate the full Item because we don't want any data in the
                    // transform_target to initially affect the layout of the draggable items.
                    sticker: item.sticker.clone(),
                    kind: Mutable::new(match item.kind.get_cloned() {
                        ItemKind::Static => ItemKind::Static,
                        ItemKind::Interactive(data) => ItemKind::Interactive(Interactive {
                            audio: data.audio.clone(),
                            target_transform: Mutable::new(None),
                        }),
                    }),
                },
                index,
                drag: Mutable::new(None),
                base: base.clone(),
                is_placed_item: false,
                trace_idx: Mutable::new(None),
            })
            .collect();

        let placed_items = base
            .sticker_targets
            .lock_ref()
            .iter()
            .map(|sticker_target| DragItem {
                item: sticker_target.item.clone(),
                index: sticker_target.sticker_idx,
                drag: Mutable::new(None),
                base: base.clone(),
                is_placed_item: true,
                trace_idx: Mutable::new(Some(sticker_target.trace_idx)),
            })
            .collect();

        Rc::new(Self {
            base,
            items,
            placed_items: MutableVec::new_with_values(placed_items),
        })
    }
}

#[derive(Clone)]
pub struct DragItem {
    pub item: Item,
    /// Index of this item in the list of stickers
    pub index: usize,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub base: Rc<Base>,
    /// Whether this is an item which has been placed in a trace
    pub is_placed_item: bool,
    /// Index of the trace where this item has been placed
    pub trace_idx: Mutable<Option<usize>>,
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
