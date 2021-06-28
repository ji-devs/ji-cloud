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
use super::trace::*;

pub fn render(traces: Vec<SelectTrace>) -> Dom { 

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

