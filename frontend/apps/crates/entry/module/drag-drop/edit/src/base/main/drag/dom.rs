use super::state::*;
use components::stickers::dom::{
    mixin_sticker_button, render_sticker_raw, BaseRawRenderOptions, StickerRawRenderOptions,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use utils::prelude::*;

impl MainDrag {
    pub fn render(state: Rc<Self>) -> Dom {
        let theme_id = state.base.theme_id.get();

        html!("empty-fragment", {
            .child(html!("empty-fragment", {
                .style("opacity", "50%")
                .children( {
                    state.items
                        .iter()
                        .map(|item| {
                            render_sticker_raw(&item.raw_sticker(), theme_id, None)
                        })
                        .collect::<Vec<Dom>>()
                })
            }))
            .children( {
                state.items
                    .iter()
                    .map(|item| {

                        let raw_sticker = &item.raw_sticker();

                        let opts = {
                            if item.get_is_interactive() {
                                let mut opts = BaseRawRenderOptions::default();
                                opts.set_size(Mutable::new(None));
                                let size_signal = opts.size.clone().unwrap_or_else(|| Mutable::new(None));

                                opts.set_transform_override(item.get_transform_override());

                                opts.set_mixin(clone!(item => move |dom| {
                                    dom
                                        .apply(mixin_sticker_button)
                                        .event(clone!(item => move |evt: events::PointerDown| {
                                            item.start_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event(clone!(item => move |evt:events::PointerMove| {
                                            item.try_move_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event(clone!(item, size_signal => move |evt: events::PointerUp| {
                                            item.try_end_drag(evt.x() as i32, evt.y() as i32, size_signal.get());
                                        }))
                                        .global_event(clone!(item, size_signal => move |evt: events::PointerCancel| {
                                            item.try_end_drag(evt.x() as i32, evt.y() as i32, size_signal.get());
                                        }))
                                }));

                                let opts = StickerRawRenderOptions::new(raw_sticker, Some(opts));
                                Some(opts)
                            } else {
                                None
                            }
                        };

                        render_sticker_raw(raw_sticker, theme_id, opts)
                    })
                    .collect::<Vec<Dom>>()
            })
        })
    }
}
