use super::state::*;
use std::rc::Rc;

use components::traces::show::{TracesShow, TracesShowMode};
use dominator::{clone, html, Dom};
use gloo_timers::future::TimeoutFuture;

pub fn render(state: Rc<Hints>) -> Dom {
    html!("empty-fragment", {
        .future(clone!(state => async move {
            TimeoutFuture::new(crate::config::HINT_TIME).await;
            state.finish();
        }))
        .child(TracesShow::render(TracesShow::new(
                state.game.base.traces.clone(),
                TracesShowMode::Cutout,
                TracesShow::on_select_noop()
        )))
    })
}
