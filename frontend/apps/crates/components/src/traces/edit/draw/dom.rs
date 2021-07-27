use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use wasm_bindgen::prelude::*;
use futures_signals::{map_ref, signal::{ReadOnlyMutable, Signal, SignalExt}, signal_vec::{MutableVecLockRef, SignalVec, SignalVecExt}};
use super::{
    state::*,
    trace::state::*,
    menu::dom::render_draw_menu
};
use crate::{traces::{edit::{state::*, all::trace::state::*}, svg::{self, SvgCallbacks, ShapeStyle, ShapeStyleBase}}, transform::state::ResizeLevel};

use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;
use crate::transform::dom::render_transform;

pub fn render_traces_draw(state:Rc<Draw>, full_list: MutableVecLockRef<Rc<AllTrace>>) -> Dom { 

    let shadow_traces:Vec<Rc<AllTrace>> = 
        full_list
            .iter()
            .enumerate()
            .filter(|(idx, value)| {
                Some(*idx) != state.init_index 
            })
            .map(|(_, value)| value.clone())
            .collect();

    let trace_signal = map_ref! {
        let resize_info = resize_info_signal(),
        let shape = state.trace.shape.signal_cloned(),
        let draw_points = state.draw_points.signal_cloned(),
        let transform = state.trace.transform.get_inner_signal_cloned(),
        let size = state.trace.transform.size.signal_cloned(),
        let display_trace = state.display_trace.signal()
            => {
                (resize_info.clone(), size.clone(), display_trace.clone(), draw_points.clone(), shape.clone(), transform.clone())
            }
    };

    let mask_children = 
        trace_signal.map(clone!(state, shadow_traces => move |(resize_info, size, display_trace, draw_points, shape, transform)| {

            let mut elements:Vec<Dom> = Vec::new();
            elements.push({
                let style = ShapeStyle::new(ShapeStyleBase::Mask);

                if !display_trace {
                    svg::render_path(&style, &resize_info, None, &draw_points, SvgCallbacks::none())
                } else {
                    let transform_size = size.map(|size| (&transform, size));
                    match shape {

                        TraceShape::Path(path) => {
                            svg::render_path_signal(&style, resize_info.clone(), transform_size, &path)
                        },

                        TraceShape::Rect(width, height) => {
                            svg::render_rect(&style, &resize_info, transform_size, width, height,SvgCallbacks::none())
                        }
                        TraceShape::Ellipse(radius_x, radius_y) => {
                            svg::render_ellipse(&style, &resize_info, transform_size, radius_x, radius_y,SvgCallbacks::none())
                        }
                    }
                }
            });

            for trace in shadow_traces.iter() {
                let style = ShapeStyle::new(ShapeStyleBase::Mask);
                elements.push(crate::traces::edit::all::dom::render_trace(&style, &resize_info, trace,SvgCallbacks::none()))
            }

            elements
        }))
        .to_signal_vec();

    let shadow_children = resize_info_signal()
        .map(move |resize_info| {
            shadow_traces
                .iter()
                .map(|trace| {
                    let style = ShapeStyle::new(ShapeStyleBase::Shadow);
                    crate::traces::edit::all::dom::render_trace(&style, &resize_info, trace,SvgCallbacks::none())
                })
                .collect::<Vec<Dom>>()
        })
        .to_signal_vec();

    let menu_signal = map_ref! {
        let resize_info = resize_info_signal(),
        let menu = state.menu.signal_cloned()
            => {
                (resize_info.clone(), menu.clone()) 
            }
    };

    html!("empty-fragment", {
        //.child(svg::render_simple(shadow_children))
        .child(
            svg::render_masks(
                mask_children,
                shadow_children,
                clone!(state => move |x, y| {
                    state.start_draw(x, y);
                }),
                clone!(state => move |x, y| {
                    state.end_draw(x, y);
                }),
                clone!(state => move |x, y| {
                    state.move_draw(x, y);
                }),
            )
        )
        .children_signal_vec(
            menu_signal.map(clone!(state => move |(resize_info, menu)| {
                let mut children:Vec<Dom> = Vec::new();
                if let Some(menu) = menu {
                    children.push(render_draw_menu(state.clone(), menu, &resize_info));
                    children.push(render_transform(
                            state.trace.transform.clone(),
                            ResizeLevel::Full,
                            None as Option<Box<dyn Fn() -> Dom>>
                    ));
                }

                children
            }))
            .to_signal_vec()
        )
             
    })
}
