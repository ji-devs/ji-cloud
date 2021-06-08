use std::rc::Rc;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use shared::domain::jig::module::body::tapping_board::TappingTrace;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use components::traces::{
    utils::TraceExt,
    svg::{render_single_trace, ShapeStyle, ShapeStyleBase, SvgCallbacks},
    select::{
        dom::render as render_select_traces,
        trace::SelectTrace
    },
    bubble::{
        dom::render as render_bubble,
        state::*
    }
};

use super::state::*;

//pub fn render(state: Rc<PlayState>, index: usize, full_trace: TappingTrace) -> Dom {
pub fn render(state: Rc<PlayState>) -> Dom {
    html!("empty-fragment", {
        .children(
            state.traces
                .iter()
                .enumerate()
                .map(|(index, trace)| {
                    render_trace(state.clone(), trace.clone(), index)
                })
                .collect::<Vec<Dom>>()
        )
        
    })
}

pub fn render_trace(state: Rc<PlayState>, trace: Rc<PlayTrace>, index: usize) -> Dom {
    let sig = map_ref! {
        let play_phase = trace.phase.signal_cloned(),
        let resize_info = resize_info_signal()
            => (play_phase.clone(), resize_info.clone())
    };

    html!("empty-fragment", {
        .children_signal_vec(
            sig.map(move |(play_phase, resize_info)| {
                let mut children:Vec<Dom> = Vec::new();

                let callbacks = SvgCallbacks::select(clone!(state, trace => move || {
                    state.select(index);
                }));
                match play_phase {
                    PlayPhase::Waiting => {
                        let shape_style = ShapeStyle::new(ShapeStyleBase::Transparent);

                        if let Some(dom) = render_single_trace(&shape_style, &resize_info, &trace.inner, callbacks) {
                            children.push(dom);
                        }
                    },

                    _ => {
                        let shape_style = ShapeStyle::new(ShapeStyleBase::Outline);

                        if let Some(dom) = render_single_trace(&shape_style, &resize_info, &trace.inner, callbacks) {
                            children.push(dom);
                        }

                        match play_phase {
                            PlayPhase::Playing(bubble) => {
                                children.push(render_bubble(bubble, &state.game.base.audio_mixer));
                            },
                            _ => {
                            }
                        }
                    }
                }

                children
            })
            .to_signal_vec()
        )
    })
}
