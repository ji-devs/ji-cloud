use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use super::state::*;
use crate::traces::{svg, trace::state::*};

use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;
use crate::transform;

pub fn render(state:Rc<Draw>) -> Dom { 
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

    let children = 
        trace_signal.map(clone!(state => move |(resize_info, size, display_trace, draw_points, shape, transform)| {
            if !display_trace {
                svg::render_path(&resize_info, None, &draw_points)
            } else {
                let transform_size = size.map(|size| (&transform, size));
                match shape {

                    TraceShape::Path(path) => {
                        svg::render_path_signal(resize_info.clone(), transform_size, &path)
                    },

                    TraceShape::Rect(width, height) => {
                        svg::render_rect(&resize_info, transform_size, width, height)
                    }
                    TraceShape::Ellipse(radius_x, radius_y) => {
                        svg::render_ellipse(&resize_info, transform_size, radius_x, radius_y)
                    }
                }
            }
        }))
        .map(|dom| vec![dom])
        .to_signal_vec();

    let menu_signal = map_ref! {
        let resize_info = resize_info_signal(),
        let menu = state.menu.signal_cloned()
            => {
                (resize_info.clone(), menu.clone()) 
            }
    };

    html!("empty-fragment", {
        .child(
            svg::render(
                children,
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
                    children.push(super::menu::dom::render(state.clone(), menu, &resize_info));
                    children.push(transform::dom::render(
                            state.trace.transform.clone(),
                            None as Option<Box<dyn Fn() -> Dom>>
                    ));
                }

                children
            }))
            .to_signal_vec()
        )
             
    })
}
