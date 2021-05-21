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
use shared::domain::jig::module::body::{TraceShape, Transform};
use web_sys::CanvasRenderingContext2d;
use awsm_web::canvas::get_2d_context;

pub fn render(
    ctx:&CanvasRenderingContext2d, 
    resize_info: &ResizeInfo, 
    shape: &TraceShape,
    transform: &Transform,
) {

    ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, .5)"));
    ctx.fill_rect(0.0, 0.0, resize_info.width, resize_info.height);


    ctx.set_fill_style(&JsValue::from_str("white"));
    ctx.set_stroke_style(&JsValue::from_str("blue"));

    match shape {
        TraceShape::Path(path) => {
            draw_path(ctx, resize_info, path, transform);
        }
        _ => {
            unimplemented!("don't know how to handle other shapes yet!!")
        }

    }

    //debug_draw_square(ctx, resize_info);

}

fn draw_path(
    ctx:&CanvasRenderingContext2d, 
    resize_info: &ResizeInfo, 
    path: &[(f64, f64)],
    transform: &Transform,
) {
    ctx.begin_path();

    if let Some((x, y)) = path.first() {
        let (x, y) = resize_info.get_pos_denormalized(*x, *y);
        ctx.move_to(x, y);
    }

    for (x, y) in &path[1..]  {
        let (x, y) = resize_info.get_pos_denormalized(*x, *y);
        ctx.line_to(x, y);
    }

    ctx.close_path();
    ctx.fill();
    ctx.stroke();
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
