use components::{
    instructions::player::InstructionsPlayer,
    stickers::dom::{
        mixin_sticker_button_signal, render_sticker_raw, BaseRawRenderOptions,
        StickerRawRenderOptions, TransformOverride,
    },
    traces::show::{TracesShow, TracesShowMode},
};
use dominator::{apply_methods, clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use utils::prelude::*;

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {
    let theme_id = state.game.base.theme_id;

    let targets_ready = Mutable::new(false);

    html!("empty-fragment", {
        // display block for touch action to work
        .style("display", "block")
        .style("touch-action", "none")
        .future(state.all_interactive_items_have_sizes().for_each(clone!(state, targets_ready => move |x| {
            clone!(state, targets_ready => async move {
                if x {
                    state.set_targets().await;
                    targets_ready.set_neq(true);
                }
            })
        })))
        .child_signal(state.feedback_player.signal_cloned().map(|feedback| {
            feedback.map(InstructionsPlayer::render)
        }))
        .child(TracesShow::render(TracesShow::new(
                state.game.base.target_areas
                    .iter()
                    .map(|area| area.trace.clone())
                    .collect(),
                TracesShowMode::Hidden,
                TracesShow::on_select_noop()
        )))
        .children( {
            state.items
                .iter()
                .map(|item| {
                    match item {
                        PlayItem::Static(sticker) => {
                            render_sticker_raw(sticker, theme_id, None)
                        },
                        PlayItem::Interactive(item) => {
                            let mut opts = BaseRawRenderOptions::default();

                            opts.set_size(item.size.clone());

                            opts.set_transform_override(TransformOverride::Always(item.curr_transform.read_only()));

                            opts.set_mixin(
                                clone!(state, item, targets_ready => move |dom| {
                                    apply_methods!(dom, {
                                        .apply(mixin_sticker_button_signal(item.completed.signal().map(|locked| !locked)))
                                        .apply(|dom| {
                                            dom
                                                .style_signal("display", targets_ready.signal().map(|ready| {
                                                    if ready { "block" } else { "none" }
                                                }))
                                        })
                                        .event(clone!(item => move |evt:events::PointerDown| {
                                            item.start_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event(clone!(item => move |evt:events::PointerMove| {
                                            item.try_move_drag(evt.x() as i32, evt.y() as i32);
                                        }))
                                        .global_event(clone!(state, item => move |evt:events::PointerUp| {
                                            if item.try_end_drag(evt.x() as i32, evt.y() as i32) {
                                                PlayState::evaluate(state.clone(), item.clone());
                                            }
                                        }))
                                        .global_event(clone!(state, item => move |evt:events::PointerCancel| {
                                            if item.try_end_drag(evt.x() as i32, evt.y() as i32) {
                                                PlayState::evaluate(state.clone(), item.clone());
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
