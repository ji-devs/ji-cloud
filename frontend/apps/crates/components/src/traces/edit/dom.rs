use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::state::*;
use shared::domain::jig::module::body::Sticker as RawSticker;
use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;

pub fn render(state:Rc<Edit>) -> Dom {
    let canvas_signal = map_ref! {
        let resize_info = resize_info_signal(),
        let canvas = state.canvas.signal_cloned()
            => {
                canvas.as_ref().map(|canvas| (resize_info.clone(), canvas.clone()))
            }
    };

    let trace_signal = map_ref! {
        let resize_info = resize_info_signal(),
        let path = state.trace.path.signal_vec_cloned().to_signal_cloned(),
        let ctx = state.ctx.signal_cloned(),
        let transform = state.trace.transform.get_inner_signal_cloned()
            => {
                ctx.as_ref().map(|ctx| {
                    (ctx.clone(), resize_info.clone(), path.clone(), transform.clone())
                })
            }
    };

    html!("canvas" => HtmlCanvasElement, {
        .future(canvas_signal.for_each(clone!(state => move |data| {
            if let Some((resize_info, canvas)) = data {
                canvas.set_width(resize_info.width.round() as u32);
                canvas.set_height(resize_info.height.round() as u32);
            }
            async {}
        })))
        .future(trace_signal.for_each(clone!(state => move |data| {
            if let Some((ctx, resize_info, path, transform)) = data {
                super::canvas::render(&ctx, &resize_info, &path, &transform);
            }
            async {}
        })))
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style_signal("width", resize_info_signal().map(|info| { format!("{}px", info.width) }))
        .style_signal("height", resize_info_signal().map(|info| { format!("{}px", info.height) }))
        .after_inserted(clone!(state => move |canvas| {
            let ctx = get_2d_context(&canvas, None).unwrap_ji();
            state.canvas.set(Some(Rc::new(canvas)));
            state.ctx.set(Some(Rc::new(ctx)));
        }))
        .global_event_preventable(clone!(state => move |evt:events::MouseUp| {
            state.end_draw(evt.x() as i32, evt.y() as i32);
        }))
        .global_event_preventable(clone!(state => move |evt:events::MouseMove| {
            state.move_draw(evt.x() as i32, evt.y() as i32);
        }))
    })
}
