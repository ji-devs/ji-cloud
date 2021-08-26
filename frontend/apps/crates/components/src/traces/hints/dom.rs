use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::resize::{resize_info_signal, ResizeInfo};

use crate::traces::{
    svg::{self, ShapeStyleVar, ShapeStyle, ShapeStyleKind, SvgCallbacks, TransformSize},
    utils::*,
};
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec,
};

use shared::domain::jig::module::body::_groups::design::{Trace, TraceShape};

pub fn render_traces_hint(traces: Vec<Trace>) -> Dom {
    let traces = Rc::new(traces);

    let mask_children = resize_info_signal()
        .map(clone!(traces => move |resize_info| {
            traces.
                iter()
                .map(move |trace| {
                    let shape_style = ShapeStyleVar::new_static(ShapeStyle::new_mask());

                    let callbacks = SvgCallbacks::new(
                        None::<fn()>,
                        None::<fn(web_sys::SvgElement)>,
                        None::<fn(web_sys::SvgElement)>,
                    );
                    render_trace_hint(shape_style, &resize_info, &trace, callbacks)
                })
                .collect::<Vec<Dom>>()
        }))
        .to_signal_vec();

    html!("empty-fragment", {
        .style("pointer-events", "none")
        .child(
            svg::render_masks(
                mask_children,
                signal_vec::always(Vec::new()),
                |_x, _y| {
                },
                |_x, _y| {
                },
                |_x, _y| {
                },
            )
        )
    })
}

pub fn render_trace_hint<S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    trace: &Trace,
    callbacks: Rc<SvgCallbacks>,
) -> Dom 
where
      S: Signal<Item = ShapeStyle> + 'static
{
    let transform_size = trace
        .calc_size(resize_info)
        .map(|size| TransformSize::new_static(&trace.transform, size));

    match trace.shape {
        TraceShape::Path(ref path) => svg::render_path(
            shape_style, 
            &resize_info, 
            transform_size, 
            &path, 
            callbacks
        ),

        TraceShape::Rect(width, height) => svg::render_rect(
            shape_style,
            &resize_info,
            transform_size,
            width,
            height,
            callbacks,
        ),
        TraceShape::Ellipse(radius_x, radius_y) => svg::render_ellipse(
            shape_style,
            &resize_info,
            transform_size,
            radius_x,
            radius_y,
            callbacks,
        ),
    }
}
