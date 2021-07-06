use std::rc::Rc;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use shared::domain::jig::module::body::drag_drop::DragDropTrace;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use components::traces::{
    utils::TraceExt,
    svg::{render_single_trace, ShapeStyle, ShapeStyleBase, SvgCallbacks},
    select::{
        dom::render_traces_select,
        trace::SelectTrace
    },
    bubble::{
        dom::render_trace_bubble,
        state::*
    }
};

use super::state::*;

pub fn render(state: Rc<PlayState>) -> Dom {
    html!("empty-fragment", {
    })
}
