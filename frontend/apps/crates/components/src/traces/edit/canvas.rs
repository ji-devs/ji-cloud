use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::{prelude::*, resize::ResizeInfo};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::state::*;
use shared::domain::jig::module::body::{PathPoint, Transform};
use web_sys::CanvasRenderingContext2d;
use awsm_web::canvas::get_2d_context;

pub fn render(
    ctx:&CanvasRenderingContext2d, 
    resize_info: &ResizeInfo, 
    path: &[PathPoint],
    transform: &Transform,
) {

    ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, .5)"));
    ctx.fill_rect(0.0, 0.0, resize_info.width, resize_info.height);


    ctx.set_fill_style(&JsValue::from_str("white"));
    ctx.set_stroke_style(&JsValue::from_str("blue"));
    draw_path(ctx, resize_info, path, transform);

    //debug_draw_square(ctx, resize_info);

}

fn draw_path(
    ctx:&CanvasRenderingContext2d, 
    resize_info: &ResizeInfo, 
    path: &[PathPoint],
    transform: &Transform,
) {
    ctx.begin_path();

    for point in path.iter() {
        plot_point(ctx, resize_info, point.clone(), transform);
    }

    //close the loop?
    /*
    if let Some(point) = path.first() {
        plot_point(ctx, resize_info, point.clone(), transform);
    }
    */
    ctx.close_path();
    ctx.fill();
    ctx.stroke();
}

fn plot_point(ctx:&CanvasRenderingContext2d, resize_info:&ResizeInfo, point:PathPoint, transform:&Transform) {
    match point {
        PathPoint::MoveTo(x, y) => {
            let (x, y) = resize_info.get_pos_denormalized(x, y);
            ctx.move_to(x, y);
        },
        PathPoint::LineTo(x, y) => {
            let (x, y) = resize_info.get_pos_denormalized(x, y);
            ctx.line_to(x, y);
        }
        _ => {}
    }
}

fn debug_draw_square(ctx:&CanvasRenderingContext2d, resize_info:&ResizeInfo) {
    let ResizeInfo {scale, width, height, ..} = resize_info;
    let size = 100.0 * scale;
    let orig_x = width / 2.0;
    let orig_y = height / 2.0;

    ctx.begin_path();
    ctx.move_to(orig_x - size, orig_y - size);
    ctx.line_to(orig_x + size, orig_y - size);
    ctx.line_to(orig_x + size, orig_y + size);
    ctx.line_to(orig_x - size, orig_y + size);
    ctx.line_to(orig_x - size, orig_y - size);
    ctx.close_path();
    ctx.fill();
    log::info!("drew square...");
}
