use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}, math::bounds::BoundsF64};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, ReadOnlyMutable},
    signal_vec::{SignalVec, SignalVecExt},
};
use crate::traces::{
    svg::{self, ShapeStyle, ShapeStyleBase, SvgCallbacks}, 
    edit::state::*
};
use super::{
    trace::state::*,
    select_box::dom::render_select_box
};

use shared::domain::jig::module::body::{Transform, _groups::design::{Trace as RawTrace, TraceShape}};
use web_sys::{SvgElement, HtmlCanvasElement};
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;

pub fn render_traces_all(state:Rc<TracesEdit>) -> Dom { 

    let mask_children = resize_info_signal()
        .switch_signal_vec(clone!(state => move |resize_info| {
            state.list
                .signal_vec_cloned()
                .map(clone!(resize_info, state => move |trace| {
                    let style = ShapeStyle::new(ShapeStyleBase::Mask);
                    let callbacks = SvgCallbacks::new(
                        None::<fn()>, 
                        None::<fn(web_sys::SvgElement)>, 
                        None::<fn(web_sys::SvgElement)>, 
                    );
                    render_trace(&style, &resize_info, &trace, callbacks)
                }))
        }));

    let click_children = resize_info_signal()
        .switch_signal_vec(clone!(state => move |resize_info| {
            state.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(resize_info, state => move |(index, trace)| {
                    let style = ShapeStyle::new(ShapeStyleBase::Transparent);
                    let callbacks = SvgCallbacks::new(
                        Some(clone!(state, index => move || {
                            if let Some(index) = index.get_cloned() {
                                state.select_index(index);
                            }
                        })),
                        Some(clone!(trace, resize_info => move |elem:SvgElement| {
                            let rect = elem.get_bounding_client_rect();
                            trace.bounds.set(Some(BoundsF64::new_from_dom_normalized(&rect, &resize_info)));
                            *trace.elem.borrow_mut() = Some(elem);
                        })),
                        Some(clone!(trace => move |elem| {
                            trace.bounds.set(None);
                            *trace.elem.borrow_mut() = None; 
                        })),
                    );
                    render_trace(&style, &resize_info, &trace, callbacks)
                }))
        }));
    let menu_children = resize_info_signal()
        .switch_signal_vec(clone!(state => move |resize_info| {
            state.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(state, resize_info => move |(index, trace)| {
                    render_select_box(state.clone(), index, &trace, &resize_info)
                }))
        }));
    html!("empty-fragment", {
        .child(
            svg::render_masks(
                mask_children,
                click_children,
                clone!(state => move |x, y| {
                    TracesEdit::start_draw(state.clone(), None, Some((x, y)));
                }),
                clone!(state => move |x, y| {
                }),
                clone!(state => move |x, y| {
                }),
            )
        )
        .children_signal_vec(menu_children)
    })
}

pub fn render_trace(style: &ShapeStyle, resize_info:&ResizeInfo, trace:&AllTrace, callbacks: SvgCallbacks) -> Dom {
    let transform_size = Some((&trace.transform, trace.size.clone())); 


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
