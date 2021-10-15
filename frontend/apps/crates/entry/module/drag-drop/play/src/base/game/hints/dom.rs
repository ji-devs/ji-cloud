use super::state::*;
use std::rc::Rc;

use components::{
    stickers::dom::render_stickers_raw,
    traces::show::{TracesShow, TracesShowMode},
};
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use gloo_timers::future::TimeoutFuture;
pub fn render(state: Rc<Hints>) -> Dom {
    html!("empty-fragment", {
        .future(clone!(state => async move {
            TimeoutFuture::new(crate::config::HINT_TIME).await;
            state.finish();
        }))
        .child(render_stickers_raw(
            &state.game.base.items
                .iter()
                .map(|item| {
                    item.sticker.clone()
                })
                .collect::<Vec<_>>(),
            state.game.base.theme_id
        ))
        .child(TracesShow::render(TracesShow::new(
                state.game.base.target_areas
                    .iter()
                    .map(|area| area.trace.clone())
                    .collect(),
                TracesShowMode::Cutout,
                TracesShow::on_select_noop()
        )))
    })
}
