use dominator::{DomBuilder, html, Dom, clone, svg, class};
use std::{f64::consts::PI, rc::Rc};
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}, math::{bounds, quat}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, Mutable, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use web_sys::{HtmlCanvasElement, SvgElement};
use awsm_web::canvas::{Canvas2dContext, get_2d_context};
use once_cell::sync::Lazy;
use std::fmt::Write;
use shared::domain::jig::module::body::{Transform, _groups::design::{Trace, TraceShape}};
use super::utils::*;
use web_sys::CanvasRenderingContext2d;

pub fn draw_trace(ctx: &CanvasRenderingContext2d, resize_info:&ResizeInfo, trace:&Trace) {
    if let Some(size) = trace.calc_size(resize_info) {
        //could use canvas size too, they should be the same
        let h_screen_width = resize_info.width / 2.0;
        let h_screen_height = resize_info.height / 2.0;
        let (tx, ty) = trace.transform.get_denormalized_translation_2d(resize_info);
        let (_, rot_rad) = quat::get_axis_angle(&trace.transform.rotation.0);
        let (scale_x, scale_y) = trace.transform.get_scale_2d();

        ctx.save();

        ctx.begin_path();

        //TODO - would be nice to just get the 2d matrix directly
        //Should be able to use it for ctx.set_transform
        //But this works for now :)

        //Move our origin to the middle of the canvas
        ctx.translate(h_screen_width, h_screen_height);
       
        //Rotate the canvas these degrees around that middle
        ctx.rotate(rot_rad);

        //Scale this amount around that middle
        ctx.scale(scale_x, scale_y);

        //Translate our 0,0 based on the transform
        ctx.translate(tx, ty);

        //Move the canvas back so it all appears normal again
        ctx.translate(-h_screen_width, -h_screen_height);
       
        match trace.shape {
            TraceShape::Path(ref path) => {
                draw_path(&ctx, &resize_info, &path)
            },

            TraceShape::Rect(width, height) => {
                draw_rect(&ctx, &resize_info, width, height)
            }
            TraceShape::Ellipse(radius_x, radius_y) => {
                draw_ellipse(&ctx, &resize_info, radius_x, radius_y)
            }
        }

        ctx.close_path();

        ctx.restore();
    }
}

pub fn draw_path(ctx: &CanvasRenderingContext2d, resize_info: &ResizeInfo, points: &[(f64, f64)]) {

    ctx.move_to(0.0, 0.0);

    for point in points {
        let (x, y) = resize_info.get_pos_denormalized(point.0, point.1);
        ctx.line_to(x, y);
    }

}


pub fn draw_rect(ctx: &CanvasRenderingContext2d, resize_info: &ResizeInfo, width: f64, height: f64) {
    let (width, height) = resize_info.get_size_denormalized(width, height);
    ctx.rect(0.0, 0.0, width, height);
}

pub fn draw_ellipse(ctx: &CanvasRenderingContext2d, resize_info: &ResizeInfo, radius_x: f64, radius_y: f64) {

    let (radius_x, radius_y) = resize_info.get_pos_denormalized(radius_x, radius_y);

    ctx.ellipse(
        radius_x,
        radius_y,
        radius_x,
        radius_y,
        0.0,
        0.0,
        PI * 2.0
    );

}
