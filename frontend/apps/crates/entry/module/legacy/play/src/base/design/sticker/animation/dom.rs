use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;

use wasm_bindgen::JsCast;

use super::super::helpers::*;
use super::state::*;
use awsm_web::canvas::get_2d_context;
use std::rc::Rc;
use utils::{
    math::{bounds, mat4::Matrix4},
    prelude::*,
    resize::resize_info_signal,
};
use web_sys::HtmlCanvasElement;

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
                let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix);
                let transform_signal = resize_info_signal().map(move |resize_info| {
                    let mut m = transform_matrix.clone();
                    m.denormalize(&resize_info);
                    m.as_matrix_string()
                });

                html!("canvas" => web_sys::HtmlCanvasElement, {
                    .future(state.controller.hidden.signal().for_each(clone!(state => move |hidden| {
                        state.repaint_for_hidden(hidden);
                        async move {}
                    })))
                    // doesn't work, things get weird, unfortunately...
                    // .style_signal("opacity", state.controller.hidden.signal().map(|hidden| {
                    //     log::info!("hidden: {}", hidden);
                    //     if hidden {
                    //         "0"
                    //     } else {
                    //         "1"
                    //     }
                    // }))
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
                        let paint_ctx = get_2d_context(&elem, None).unwrap_ji();
                        *state.paint_ctx.borrow_mut() = Some(paint_ctx);


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
        //             .attribute("src", &format!("{}.webm", &state.base.design_media_url(&state.raw.src)))
        //             .attribute("type", "video/webm; codecs=vp9")
        //         }),
        //         html!("source", {
        //             .attribute("src", &format!("{}.mp4", &state.base.design_media_url(&state.raw.src)))
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
