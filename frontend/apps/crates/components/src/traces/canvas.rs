use std::f64::consts::PI;
use utils::{math::quat, prelude::*, resize::ResizeInfo};

use super::utils::*;
use shared::domain::jig::module::body::_groups::design::{PathCommand, Trace, TraceShape};
use web_sys::CanvasRenderingContext2d;

pub fn draw_trace(ctx: &CanvasRenderingContext2d, resize_info: &ResizeInfo, trace: &Trace) {
    if let Some(bounds) = trace.calc_bounds(false) {
        let (width, height) = resize_info.get_size_denormalized(bounds.width, bounds.height);
        let h_width = width / 2.0;
        let h_height = height / 2.0;
        let (tx, ty) = trace.transform.get_denormalized_translation_2d(resize_info);
        let (_, rot_rad) = quat::get_axis_angle(&trace.transform.rotation.0);
        let (scale_x, scale_y) = trace.transform.get_scale_2d();

        ctx.save();

        //TODO - would be nice to just get the 2d matrix directly
        //Should be able to use it for ctx.set_transform
        //But this works for now :)

        let _ = ctx.translate(tx, ty);

        let _ = ctx.translate(h_width, h_height);

        //Rotate the canvas these degrees around that middle
        let _ = ctx.rotate(rot_rad);

        let _ = ctx.scale(scale_x, scale_y);

        //Move the canvas back so it all appears normal again
        let _ = ctx.translate(-h_width, -h_height);

        ctx.begin_path();
        match &trace.shape {
            TraceShape::Rect(width, height) => {
                draw_rect(ctx, resize_info, *width, *height);
            }
            TraceShape::Ellipse(radius_x, radius_y) => {
                draw_ellipse(ctx, resize_info, *radius_x, *radius_y);
            }
            TraceShape::Path(points) => {
                draw_path(ctx, resize_info, points);
            }
            TraceShape::PathCommands(commands) => {
                draw_path_commands(ctx, resize_info, commands);
            }
        }
        ctx.close_path();

        ctx.restore();
    }
}

pub fn draw_path_commands(
    _ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    commands: &[(PathCommand, bool)],
) {
    for (command, _absolute) in commands {
        let _command = denormalize_command(command, resize_info);
        unimplemented!("TODO - support path commands in canvas drawing!")
    }
}
pub fn draw_path(ctx: &CanvasRenderingContext2d, resize_info: &ResizeInfo, points: &[(f64, f64)]) {
    ctx.move_to(0.0, 0.0);

    for point in points {
        let (x, y) = resize_info.get_pos_denormalized(point.0, point.1);
        ctx.line_to(x, y);
    }
}

pub fn draw_rect(
    ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    width: f64,
    height: f64,
) {
    let (width, height) = resize_info.get_size_denormalized(width, height);
    ctx.rect(0.0, 0.0, width, height);
}

pub fn draw_ellipse(
    ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    radius_x: f64,
    radius_y: f64,
) {
    let (radius_x, radius_y) = resize_info.get_pos_denormalized(radius_x, radius_y);

    let _ = ctx.ellipse(radius_x, radius_y, radius_x, radius_y, 0.0, 0.0, PI * 2.0);
}
