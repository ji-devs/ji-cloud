use super::svg::helpers::path_command_to_string;
use super::utils::*;
use shared::domain::jig::module::body::{
    Transform,
    _groups::design::{PathCommand, Trace, TraceShape},
};
use std::f64::consts::PI;
use utils::{
    math::{mat4::Matrix4, quat, BoundsF64},
    prelude::*,
    resize::ResizeInfo,
};
use web_sys::CanvasRenderingContext2d;

pub fn draw_trace(ctx: &CanvasRenderingContext2d, resize_info: &ResizeInfo, trace: &Trace) {
    if let Some(bounds) = trace.calc_bounds(false) {
        ctx.save();

        apply_transform(ctx, resize_info, &trace.transform, &bounds);

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

/// unfortunately, canvas doesn't support the full 16-element matrix
/// so we need to revert the original 6-element matrix
/// but even with that, the order isn't the same as css 6-element matrix
pub fn apply_transform_mat4(ctx: &CanvasRenderingContext2d, mat: &Matrix4) {
    let translate_x = mat[12];
    let translate_y = mat[13];
    let scale_x = mat[0];
    let scale_y = mat[5];
    let skew_x = mat[4].atan();
    let skew_y = mat[1].atan();

    let _ = ctx
        .transform(scale_x, skew_y, skew_x, scale_y, translate_x, translate_y)
        .unwrap_ji();
}

//this was written before the above apply_transform_mat4
//it might be able to be simplified by just building on that
pub fn apply_transform(
    ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    transform: &Transform,
    bounds: &BoundsF64,
) {
    let (width, height) = resize_info.get_size_denormalized(bounds.width, bounds.height);
    let h_width = width / 2.0;
    let h_height = height / 2.0;
    let (tx, ty) = transform.get_denormalized_translation_2d(resize_info);
    let (_, rot_rad) = quat::get_axis_angle(&transform.rotation.0);
    let (scale_x, scale_y) = transform.get_scale_2d();

    let _ = ctx.translate(tx, ty);

    let _ = ctx.translate(h_width, h_height);

    //Rotate the canvas these degrees around that middle
    let _ = ctx.rotate(rot_rad);

    let _ = ctx.scale(scale_x, scale_y);

    //Move the canvas back so it all appears normal again
    let _ = ctx.translate(-h_width, -h_height);
}

pub fn clip_single_shape(
    ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    shape: &TraceShape,
) {
    let path = shape.as_path2d(resize_info);
    ctx.clip_with_path_2d(&path);
}

//returns whether it already filled - an unfortunate necessity for PathCommands
pub fn draw_single_shape(
    ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    shape: &TraceShape,
) -> bool {
    match shape {
        TraceShape::PathCommands(ref commands) => draw_path_commands(ctx, resize_info, commands),
        TraceShape::Path(ref path) => draw_path(ctx, resize_info, path),

        TraceShape::Rect(width, height) => draw_rect(ctx, resize_info, *width, *height),
        TraceShape::Ellipse(radius_x, radius_y) => {
            draw_ellipse(ctx, resize_info, *radius_x, *radius_y)
        }
    }

    match shape {
        TraceShape::PathCommands(_) => true,
        _ => false,
    }
}

pub fn draw_path_commands(
    ctx: &CanvasRenderingContext2d,
    resize_info: &ResizeInfo,
    commands: &[(PathCommand, bool)],
) {
    //log::warn!("canvas draw for path commands inherently fills!!!");

    let path_string = path_command_to_string(
        commands
            .iter()
            .map(|(command, absolute)| (denormalize_command(command, resize_info), *absolute)),
    );

    let path_2d = web_sys::Path2d::new_with_path_string(&path_string).unwrap_ji();

    ctx.fill_with_path_2d(&path_2d);
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
