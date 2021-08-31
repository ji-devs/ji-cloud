use std::rc::Rc;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal_vec::{self, SignalVecExt},
    signal::{SignalExt}
};
use shared::domain::jig::module::body::tapping_board::TappingTrace;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use components::traces::{
    utils::TraceExt,
    //svg::{render_single_trace, ShapeStyle, ShapeStyleBase, SvgCallbacks},
    show::{TracesShow, TracesShowMode},
    bubble::TraceBubble,
};

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {

    html!("empty-fragment", {
        .child(TracesShow::render(TracesShow::new(
                state.game.base.traces
                    .iter()
                    .map(|t| t.trace.clone())
                    .collect(),
                TracesShowMode::HiddenSolidMap(state.selected_set.clone()),
                Some(clone!(state => move |index| {
                    state.select(index);
                }))
        )))

        .children_signal_vec(
            resize_info_signal()
                .switch_signal_vec(clone!(state => move |resize_info| {
                    signal_vec::always(state.traces.clone())
                        .map_signal(|trace| {
                            trace.phase.signal_cloned()
                        })
                        .map(clone!(state => move |phase| {
                            match phase {
                                PlayPhase::Playing(bubble) => {
                                    Some(TraceBubble::render(bubble, &state.game.base.audio_mixer))
                                },
                                _ => None
                            }
                        }))
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap_ji())
                }))
        )

    })
}
