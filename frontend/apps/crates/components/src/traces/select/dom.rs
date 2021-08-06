use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::resize::resize_info_signal;

use crate::traces::svg::{self, ShapeStyle, ShapeStyleBase, SvgCallbacks};
use futures_signals::signal::SignalExt;

use super::trace::*;

pub fn render_traces_select(traces: Vec<SelectTrace>) -> Dom {
    let traces = Rc::new(traces);

    let children = resize_info_signal()
        .map(clone!(traces => move |resize_info| {
            traces.
                iter()
                .map(move |trace| {
                    let style = ShapeStyle::new(ShapeStyleBase::Transparent);
                    let on_select = trace.on_select.clone();
                    let callbacks = SvgCallbacks::new(
                        Some(move || {
                            on_select();
                        }),
                        None::<fn(web_sys::SvgElement)>,
                        None::<fn(web_sys::SvgElement)>,
                    );
                    svg::render_single_shape(&style, &resize_info, &trace.inner, None, callbacks)
                })
                .collect::<Vec<Dom>>()
        }))
        .to_signal_vec();

    html!("empty-fragment", {
        .child(
            svg::render_simple(
                children,
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
