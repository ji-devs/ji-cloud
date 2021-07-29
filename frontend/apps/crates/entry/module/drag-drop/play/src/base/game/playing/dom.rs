use std::rc::Rc;
use dominator::{clone, html, Dom, with_node, apply_methods};
use futures_signals::{
    map_ref,
    signal_vec::SignalVecExt,
    signal::{Mutable, SignalExt}
};
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use components::{
    traces::hints::dom::render_traces_hint,
    stickers::dom::{render_sticker_raw, StickerRawRenderOptions,BaseRawRenderOptions ,TransformOverride, mixin_sticker_button}
};

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {


    let theme_id = state.game.base.theme_id;

    let targets_ready = Mutable::new(false);

    html!("empty-fragment", {
        .future(state.all_interactive_items_have_sizes().for_each(clone!(state, targets_ready => move |x| {
            clone!(state, targets_ready => async move {
                if x {
                    state.set_targets().await;
                    targets_ready.set_neq(true);
                }
            })
        })))
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
                            let mut opts = BaseRawRenderOptions::default();

                            opts.set_size(item.size.clone());

                            opts.set_transform_override(TransformOverride::Always(item.curr_transform.read_only()));

                            opts.set_mixin(
                                clone!(state, item, targets_ready => move |dom| {
                                    apply_methods!(dom, {
                                        .apply(mixin_sticker_button)
                                        .apply(|dom| {
                                            dom
                                                .style_signal("display", targets_ready.signal().map(|ready| {
                                                    if ready { "block" } else { "none" }
                                                }))
                                        })
                                        .event(clone!(item => move |evt:events::MouseDown| {
                                            item.start_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event_preventable(clone!(item => move |evt:events::MouseMove| {
                                            item.try_move_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event_preventable(clone!(state, item => move |evt:events::MouseUp| {
                                            if item.try_end_drag(evt.x() as i32, evt.y() as i32) {
                                                state.evaluate(item.clone());
                                            }
                                        }))
                                    })
                                })
                            );
                            
                            let opts = StickerRawRenderOptions::new(&item.sticker, Some(opts));

                            render_sticker_raw(&item.sticker, theme_id, Some(opts))
                        }
                    }
                })
                .collect::<Vec<Dom>>()
        })
    })
}
