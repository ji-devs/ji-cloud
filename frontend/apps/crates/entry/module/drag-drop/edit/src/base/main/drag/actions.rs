use super::state::*;
use components::collision::stickers_traces::pixels::{
    get_hit_index, StickerBoundsKind, StickerHitSource,
};
use shared::domain::jig::module::body::_groups::design::Trace;
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

    pub fn try_end_drag(&self, _x: i32, _y: i32, size: Option<(f64, f64)>) {
        let target_transform = self
            .item
            .get_interactive_unchecked()
            .target_transform
            .get_cloned();

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

                    let traces: Vec<&Trace> = self
                        .base
                        .target_areas
                        .iter()
                        .map(|area| &area.trace)
                        .collect();

                    if get_hit_index(hit_source, &traces).is_some() {
                        // The hit_index doesn't matter - We don't actually save the target trace,
                        // so we only care that the sticker has been placed inside a target.
                        self.base
                            .set_drag_item_target_transform(self.index, transform);
                    } else {
                        // Reset to starting position
                        if let Some(sticker) = self.base.stickers.get(self.index) {
                            self.base.set_drag_item_target_transform(
                                self.index,
                                sticker.transform().get_inner_clone(),
                            );
                        }
                    }
                }
            }
        }
    }
}
