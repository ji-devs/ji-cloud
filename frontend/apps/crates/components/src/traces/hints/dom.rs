use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}, math::bounds::BoundsF64};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, ReadOnlyMutable},
    signal_vec::{self, SignalVec, SignalVecExt},
};
use crate::traces::{
    svg::{self, ShapeStyle, ShapeStyleBase, SvgCallbacks}, 
    edit::state::*,
    utils::*
};

use shared::domain::jig::module::body::{Transform, _groups::design::{Trace, TraceShape}};
use web_sys::{SvgElement, HtmlCanvasElement};
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;

pub fn render(traces: Vec<Trace>) -> Dom { 

    let traces = Rc::new(traces);

    let mask_children = resize_info_signal()
        .map(clone!(traces => move |resize_info| {
            traces.
                iter()
                .map(move |trace| {
                    let style = ShapeStyle::new(ShapeStyleBase::Mask);
                    let callbacks = SvgCallbacks::new(
                        None::<fn()>, 
                        None::<fn(web_sys::SvgElement)>, 
                        None::<fn(web_sys::SvgElement)>, 
                    );
                    render_trace(&style, &resize_info, &trace, callbacks)
                })
                .collect::<Vec<Dom>>()
        }))
        .to_signal_vec();

    html!("empty-fragment", {
        .child(
            svg::render_masks(
                mask_children,
                signal_vec::always(Vec::new()),
                |x, y| {
                },
                |x, y| {
                },
                |x, y| {
                },
            )
        )
    })
}

pub fn render_trace(style: &ShapeStyle, resize_info:&ResizeInfo, trace:&Trace, callbacks: SvgCallbacks) -> Dom {

    let transform_size = trace.calc_size(resize_info)
        .map(|size| (&trace.transform, size));


    match trace.shape {

        TraceShape::Path(ref path) => {
            svg::render_path(&style, &resize_info, transform_size, &path, callbacks)
        },

        TraceShape::Rect(width, height) => {
            svg::render_rect(&style, &resize_info, transform_size, width, height, callbacks)
        }
        TraceShape::Ellipse(radius_x, radius_y) => {
            svg::render_ellipse(&style, &resize_info, transform_size, radius_x, radius_y, callbacks)
        }
    }
}
