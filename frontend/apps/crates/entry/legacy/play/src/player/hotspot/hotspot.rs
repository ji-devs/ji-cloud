use web_sys::CanvasRenderingContext2d;
use legacy::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use utils::resize::ResizeInfo;

pub struct Hotspot {
}

impl Hotspot {
// https://github.com/ji-devs/jitap-create/blob/ac8decdebdf2537ee8b2c186a3a00a9f2deb074b/src/components/trace/view/Trace-View.tsx#L80
    pub fn render(ctx:&CanvasRenderingContext2d, path: &[PathPoint], resize_info:ResizeInfo) {
        ctx.set_fill_style(&JsValue::from_str("rgba(0,0,0,1)"));
        Self::draw_path(ctx, path, resize_info);
        //Self::draw_debug_square(ctx, resize_info);
        ctx.fill();
    }

    fn draw_debug_square(ctx:&CanvasRenderingContext2d, resize_info:ResizeInfo) {
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
    }

    fn draw_path(ctx:&CanvasRenderingContext2d, path:&[PathPoint], resize_info:ResizeInfo) {
        ctx.begin_path();

        let ResizeInfo {scale, height, ..} = resize_info;


        for point in path.iter() {
            let PathPoint {kind, x, y, cp1x, cp1y, cp2x, cp2y} = point;
            let x = x * scale;
            let y = y * scale;

            match kind {
                PathElementKind::MoveToPoint => {
                    ctx.move_to(x, y);
                },
                PathElementKind::AddLineToPoint => {
                    ctx.line_to(x, y);
                },
                _ => {
                    panic!("Don't know how to {:?}", kind);
                }
            }

        }
        ctx.close_path();
    }
}
