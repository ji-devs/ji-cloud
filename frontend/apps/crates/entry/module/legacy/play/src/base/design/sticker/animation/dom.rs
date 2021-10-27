use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use shared::domain::jig::module::body::legacy::design::{
    Sticker as RawSticker
};
use web_sys::HtmlCanvasElement;
use std::{borrow::Borrow, rc::Rc, cell::RefCell};
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::{canvas::{get_2d_context, CanvasToBlobFuture}, data::ArrayBufferExt};
use super::state::*;
use super::super::helpers::*;


impl Drop for AnimationPlayer { 
    fn drop(&mut self) {
        log::info!("player dropped ");
    }
}
impl AnimationPlayer { 
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .child_signal(state.size.signal_cloned().map(clone!(state => move |size| size.map(|size| {
                let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix.clone());
                let transform_signal = resize_info_signal().map(move |resize_info| {
                    let mut m = transform_matrix.clone();
                    m.denormalize(&resize_info);
                    m.as_matrix_string()
                });

                html!("canvas" => web_sys::HtmlCanvasElement, {
                    .style_signal("opacity", state.controller.hidden.signal().map(|hidden| {
                        if hidden {
                            "0"
                        } else {
                            "1"
                        }
                    }))
                    .style("cursor", if state.controller.interactive {"pointer"} else {"initial"})
                    .style("display", "block")
                    .style("position", "absolute")
                    .style_signal("width", width_signal(state.size.signal_cloned()))
                    .style_signal("height", height_signal(state.size.signal_cloned()))
                    .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
                    .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
                    .style_signal("transform", transform_signal)

                    .after_inserted(clone!(state, size => move |elem| {
                        let (natural_width, natural_height) = size; 

                        elem.set_width(natural_width as u32);
                        elem.set_height(natural_height as u32);
                        *state.paint_ctx.borrow_mut() = Some(get_2d_context(&elem, None).unwrap_ji());


                        let canvas:HtmlCanvasElement = web_sys::window().unwrap_ji().document().unwrap_ji().create_element("canvas").unwrap_ji().unchecked_into();
                        canvas.set_width(natural_width as u32);
                        canvas.set_height(natural_height as u32);
                        *state.work_ctx.borrow_mut() = Some(get_2d_context(&canvas, None).unwrap_ji());
                        *state.work_canvas.borrow_mut() = Some(canvas);

                        *state.controller.elem.borrow_mut() = Some(elem.unchecked_into());
                        state.base.insert_stage_click_listener(clone!(state => move |stage_click| {
                            state.controller.handle_click(stage_click);
                        }));

                        state.request_frame();
                    }))
                })
            }))))
        })

        // let state = self;

        // let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix.clone());
        // let transform_signal = resize_info_signal().map(move |resize_info| {
        //     let mut m = transform_matrix.clone();
        //     m.denormalize(&resize_info);
        //     m.as_matrix_string()
        // });


        // html!("video" => web_sys:: HtmlVideoElement, {
        //     .children(&mut[
        //         html!("source", {
        //             .attribute("src", &format!("{}.webm", &state.base.media_url(&state.raw.src)))
        //             .attribute("type", "video/webm; codecs=vp9")
        //         }),
        //         html!("source", {
        //             .attribute("src", &format!("{}.mp4", &state.base.media_url(&state.raw.src)))
        //             .attribute("type", "video/mp4; codecs=hvc1")
        //         }),
        //     ])
        //     .property("autoplay", true)
        //     .property("muted", true)
        //     .property("loop", true)
        //     .property("playsinline", true)
        //     .style("cursor", "pointer")
        //     .style("display", "block")
        //     .style("position", "absolute")
        //     .style_signal("width", width_signal(state.size.signal_cloned()))
        //     .style_signal("height", height_signal(state.size.signal_cloned()))
        //     .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
        //     .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
        //     .style_signal("transform", transform_signal)
        //     .with_node!(video => {
        //         .event(clone!(state => move |_evt:events::LoadedMetadata| {
        //             let width = video.video_width() as f64;
        //             let height = video.video_height() as f64;

        //             state.size.set(Some((width, height)));

        //         }))
        //     })
        //     .event(clone!(state => move |_evt:events::Click| {
        //         log::info!("clicked!")
        //     }))
        // })
    }
}