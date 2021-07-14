use components::module::_common::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_sticker_raw, StickerRawRenderOptions, mixin_sticker_button},
    traces::edit::dom::render_traces_edit
};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl MainDrag {
    pub fn render(state: Rc<Self>) -> Dom {
        let theme_id = state.base.theme_id.get();

        html!("empty-fragment", {
            .children( {
                state.clone().items
                    .iter()
                    .map(|item| {
                        let opts = {
                            if item.get_is_interactive() {
                                let mut opts = StickerRawRenderOptions::new();

                                opts.set_transform_override(item.get_transform_override());

                                opts.set_mixin(clone!(state, item => move |dom| {
                                    dom
                                        .apply(mixin_sticker_button)
                                        .event(clone!(item => move |evt:events::MouseDown| {
                                            item.start_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event_preventable(clone!(item => move |evt:events::MouseUp| {
                                            item.try_end_drag(evt.x() as i32, evt.y() as i32);

                                        }))
                                        .global_event_preventable(clone!(item => move |evt:events::MouseMove| {
                                            item.try_move_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                }));

                                Some(opts)
                            } else {
                                None
                            }
                        };

                        render_sticker_raw(&item.raw_sticker(), theme_id, opts)
                    })
                    .collect::<Vec<Dom>>()
            })
        })
    }
}
 
