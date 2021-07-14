use std::rc::Rc;
use dominator::{clone, html, Dom, with_node, apply_methods};
use futures_signals::{
    map_ref,
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use components::{
    traces::hints::dom::render_traces_hint,
    stickers::dom::{render_sticker_raw, render_sticker_raw_offset_mixin, mixin_sticker_button}
};

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {


    let theme_id = state.game.base.theme_id;

    html!("empty-fragment", {
        .child(render_traces_hint(
                state.game.base.target_areas
                    .iter()
                    .map(|area| area.trace.clone())
                    .collect()
        ))
        .children( {
            state.items
                .iter()
                .map(|item| {
                    match item {
                        PlayItem::Static(sticker) => {
                            render_sticker_raw(&sticker, theme_id, None)
                        },
                        PlayItem::Interactive(item) => {
                            render_sticker_raw_offset_mixin(
                                &item.sticker, 
                                theme_id, 
                                Some(item.size.clone()),
                                item.curr_offset.read_only(), 
                                clone!(state, item => move |dom| {
                                    apply_methods!(dom, {
                                        .apply(mixin_sticker_button)
                                        .event(clone!(item => move |evt:events::MouseDown| {
                                            item.start_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event_preventable(clone!(item => move |evt:events::MouseMove| {
                                            item.try_move_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event_preventable(clone!(state, item => move |evt:events::MouseUp| {
                                            if item.try_end_drag(evt.x() as i32, evt.y() as i32) {
                                                state.evaluate(&item);
                                            }
                                        }))
                                    })
                                })
                            )
                        }
                    }
                })
                .collect::<Vec<Dom>>()
        })
    })
}
