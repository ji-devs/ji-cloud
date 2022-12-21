use components::traces::{
    bubble::TraceBubble,
    show::{TracesShow, TracesShowMode},
};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{self, SignalVecExt},
};
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {
    html!("empty-fragment", {
        .child(TracesShow::render(TracesShow::new(
                state.traces
                    .iter()
                    .map(|t| t.inner.clone())
                    .collect(),
                TracesShowMode::HiddenSolidMap(state.current_set.clone()),
                Some(clone!(state => move |index| {
                    PlayState::select(state.clone(), index);
                }))
        )))
        .child(html!("overlay-container", {
            .children_signal_vec(
                resize_info_signal()
                    .switch_signal_vec(clone!(state => move |_resize_info| {
                        signal_vec::always(state.traces.clone())
                            .map_signal(|trace| {
                                trace.phase.signal_cloned()
                            })
                            .map(|phase| {
                                match phase {
                                    PlayPhase::Playing(bubble) => {
                                        Some(TraceBubble::render(bubble))
                                    },
                                    _ => None
                                }
                            })
                            .filter(|x| x.is_some())
                            .map(|x| x.unwrap_ji())
                    }))
            )
        }))

    })
}
