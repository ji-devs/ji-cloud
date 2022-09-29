use super::state::*;
use std::rc::Rc;

use components::{
    module::_common::play::prelude::ModulePlayPhase,
    stickers::dom::render_stickers_raw,
    traces::show::{TracesShow, TracesShowMode},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use gloo_timers::future::TimeoutFuture;
pub fn render(state: Rc<Hints>) -> Dom {
    html!("empty-fragment", {
        .future(state.game.base.module_phase.signal_cloned().for_each(clone!(state => move |phase| {
            clone!(state => async move {
                // Only start the timer if the activity is in the Playing phase.
                if let ModulePlayPhase::Playing = phase {
                    TimeoutFuture::new(crate::config::HINT_TIME).await;
                    state.finish();
                }
            })
        })))
        .child(render_stickers_raw(
            &state.game.base.items
                .iter()
                .map(|item| {
                    item.sticker.clone()
                })
                .collect::<Vec<_>>(),
            state.game.base.theme_id
        ))
        .child_signal(state.game.base.module_phase.signal_cloned().map(clone!(state => move |phase| {
            match phase {
                // Only render the hints if the activity is in the Playing phase.
                ModulePlayPhase::Playing => Some(TracesShow::render(TracesShow::new(
                    state.game.base.target_areas
                        .iter()
                        .map(|area| area.trace.clone())
                        .collect(),
                    TracesShowMode::Cutout,
                    TracesShow::on_select_noop()
                ))),
                _ => None,
            }
        })))
    })
}
