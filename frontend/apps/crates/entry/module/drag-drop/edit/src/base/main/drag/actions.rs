use crate::base::state::{Interactive, Item, ItemKind, StickerTarget};

use super::state::*;
use components::collision::stickers_traces::pixels::{
    get_hit_index, StickerBoundsKind, StickerHitSource,
};
use futures_signals::signal::Mutable;
use shared::domain::module::body::{_groups::design::Trace, drag_drop::TargetTransform};
use std::{borrow::Cow, rc::Rc};
use utils::{drag::*, prelude::*, resize::get_resize_info};

impl DragItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        let data = self.item.get_interactive_unchecked();

        if data.target_transform.lock_ref().is_none() {
            data.target_transform
                .set(Some(self.item.sticker.transform().get_inner_clone()));
        }

        self.drag
            .set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.item
                    .get_interactive_unchecked()
                    .target_transform
                    .replace_with(|t| {
                        t.as_ref().map(|t| {
                            let mut t = t.clone();
                            t.add_translation_2d(diff_x * -1.0, diff_y * -1.0);

                            t
                        })
                    });
            }
        }
    }

    pub fn try_end_drag(&self, state: Rc<MainDrag>, idx: usize, size: Option<(f64, f64)>) {
        let interactive_item = self.item.get_interactive_unchecked();
        let target_transform = interactive_item.target_transform.get_cloned();

        if let Some(transform) = target_transform {
            if self.drag.lock_ref().is_some() {
                let _drag = self.drag.lock_mut().take().unwrap_ji();

                if let Some(size) = size {
                    let raw_sticker = self.item.sticker.to_raw();
                    let hit_source = StickerHitSource {
                        sticker: Cow::Borrowed(&raw_sticker),
                        size,
                        transform_override: Some(Cow::Borrowed(&transform)),
                        bounds_kind: StickerBoundsKind::Auto,
                    };

                    let target_areas = self.base.target_areas.lock_ref();
                    let traces: Vec<&Trace> = target_areas.iter().map(|area| &area.trace).collect();

                    if let Some(hit_idx) = get_hit_index(hit_source, &traces) {
                        // Only allow the teacher to place a sticker in a target trace if the
                        // sticker isn't already present in that trace.
                        let sticker_exists = state
                            .placed_items
                            .lock_ref()
                            .iter()
                            .enumerate()
                            .find(|(item_index, item)| {
                                let same_trace = item.trace_idx.get() == Some(hit_idx);
                                let same_sticker = item.index == self.index;
                                // Only compare the indexes of the items if the item being dragged
                                // is from the placed_items list.
                                let same_drag_item = self.is_placed_item && *item_index == idx;

                                if same_trace && same_drag_item {
                                    // If the sticker is being moved around inside the same trace
                                    // area, then we don't want to mark it as existing otherwise it
                                    // will be removed from the trace area when the teacher cancels
                                    // the drag.
                                    false
                                } else {
                                    // Otherwise, if the sticker is being moved around in
                                    // a different drag area, but the same sticker already exists
                                    // in it, return true
                                    same_trace && same_sticker
                                }
                            })
                            .is_some();

                        match (self.is_placed_item, sticker_exists) {
                            (false, false) => {
                                // A item is dragged from the initial list and placed inside
                                // a trace area

                                // We need to create a new Item so that it's signals are not cloned
                                // and the transforms in one don't affect the transforms in the
                                // other.
                                let create_item = || Item {
                                    sticker: self.item.sticker.clone(),
                                    kind: Mutable::new(ItemKind::Interactive(Interactive {
                                        audio: interactive_item.audio.clone(),
                                        target_transform: Mutable::new(Some(transform.clone())),
                                    })),
                                };

                                // Create a new DragItem with the current transform. This will get added to
                                // the list of targets.
                                let drag_item = DragItem {
                                    item: create_item(),
                                    index: self.index,
                                    drag: Mutable::new(None),
                                    base: self.base.clone(),
                                    is_placed_item: true,
                                    trace_idx: Mutable::new(Some(hit_idx)),
                                };

                                state.placed_items.lock_mut().push_cloned(drag_item);
                                state
                                    .base
                                    .sticker_targets
                                    .lock_mut()
                                    .push_cloned(StickerTarget {
                                        sticker_idx: self.index,
                                        trace_idx: hit_idx,
                                        item: create_item(),
                                    })
                            }
                            (true, false) => {
                                // An item is dragged from one trace area to another. Update the
                                // trace index.
                                //
                                // This will also fire whenever a sticker is moved around inside
                                // it's current trace area, but will not change anything thanks to
                                // set_neq.
                                self.trace_idx.set_neq(Some(hit_idx));
                            }
                            (true, true) => {
                                // Moving an item from a trace into another trace where that item
                                // exists. The item should be removed.
                                //
                                // We could reset it's position, but currently we use the
                                // `target_transform` field everywhere for dragging and it would
                                // require a much larger rewrite to implement.
                                state.placed_items.lock_mut().remove(idx);
                                state.base.sticker_targets.lock_mut().remove(idx);
                            }
                            _ => {
                                // Do nothing
                            }
                        }

                        // This item can be placed if it is not a placed item and does not exist in
                        // the hit trace.
                        if !self.is_placed_item && !sticker_exists {}
                    } else if self.is_placed_item {
                        state.placed_items.lock_mut().remove(idx);
                        state.base.sticker_targets.lock_mut().remove(idx);
                    }

                    // Save the target list. This will also update the transforms if a sticker has
                    // been moved inside a target
                    self.base.update_targets(
                        state
                            .placed_items
                            .lock_ref()
                            .iter()
                            .filter(|item| {
                                item.item
                                    .get_interactive_unchecked()
                                    .target_transform
                                    .get_cloned()
                                    .is_some()
                                    && item.trace_idx.get().is_some()
                            })
                            .map(|item| TargetTransform {
                                sticker_idx: item.index,
                                transform: item
                                    .item
                                    .get_interactive_unchecked()
                                    .target_transform
                                    .get_cloned()
                                    .unwrap_ji(),
                                trace_idx: item.trace_idx.get().unwrap_ji(),
                            })
                            .collect(),
                    );

                    if !self.is_placed_item {
                        // Reset the draggable item back to its initial position.
                        if let Some(sticker) = self.base.stickers.get(self.index) {
                            interactive_item
                                .target_transform
                                .set(Some(sticker.transform().get_inner_clone()));
                        }
                    }
                }
            }
        }
    }
}
