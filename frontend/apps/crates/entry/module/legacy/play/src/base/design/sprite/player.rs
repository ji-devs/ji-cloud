use futures::io::Cursor;
use gif::Decoder;
use web_sys::CanvasRenderingContext2d;
use utils::prelude::*;
use super::{animation::Animation, SpriteData, SpritePhase};

pub struct SpritePlayer {

    pub ctx: CanvasRenderingContext2d, 
    pub data: SpriteData,
    pub phase: SpritePhase
}

impl SpritePlayer {
    pub fn new(ctx: CanvasRenderingContext2d, data: SpriteData, phase: SpritePhase) -> Self {

        ctx.clear_rect(0.0, 0.0, ctx.canvas().unwrap_ji().width() as f64, ctx.canvas().unwrap_ji().height() as f64);

        match &data {
            SpriteData::Static(img) => {
                ctx.draw_image_with_html_image_element(&img, 0.0, 0.0).unwrap_ji();
            },
            SpriteData::Animation(anim) => {
                paint_decoder_frame(&ctx, &anim);
            }
        }

        Self {
            ctx,
            data,
            phase
        }
    }
}

fn paint_decoder_frame(ctx: &CanvasRenderingContext2d, animation: &Animation) {
     let start = web_sys::window().unwrap().performance().unwrap_ji().now();

     if let Some(frame) = animation.frames.iter().next() {
        let img_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&frame.buffer().as_raw()),
            frame.buffer().width(),
            frame.buffer().height(),
        )
        .unwrap_ji();

        ctx.put_image_data(&img_data, frame.left() as f64, frame.top() as f64);
        //ctx.draw_image_with_html_image_element(&img, 0.0, 0.0).unwrap_ji();
     }
        let end = web_sys::window().unwrap().performance().unwrap_ji().now();
        log::info!("painter time: {} secs", (end - start) * 0.001);

    // let global_palette = decoder.global_palette().map(|x| x.to_vec());

    // if let Some(frame) = decoder.read_next_frame().unwrap_ji() {
    //     let end = web_sys::window().unwrap().performance().unwrap_ji().now();
    //     log::info!("decoder time: {} secs", (end - start) * 0.001);
    //     let start = end;

    //     let img_data = ctx.get_image_data(frame.left.into(), frame.top.into(), frame.width.into(), frame.height.into()).unwrap_ji();

    //     let end = web_sys::window().unwrap().performance().unwrap_ji().now();
    //     log::info!("get data time: {} secs", (end - start) * 0.001);
    //     let start = end;

    //     let palette = match frame.palette.as_ref() {
    //         Some(p) => p,
    //         None => global_palette.as_ref().unwrap_ji()
    //     };

    //     let mut data = img_data.data();

    //     assert_eq!(data.len(), frame.buffer.len());
    
    //     for (index, lookup) in frame.buffer.iter().enumerate() {
    //         if Some(*lookup) != frame.transparent {
    //             let pixel = palette[*lookup as usize];

    //             data[index] = pixel;
    //         }
    //     }

    //     let end = web_sys::window().unwrap().performance().unwrap_ji().now();
    //     log::info!("pixels time: {} secs", (end - start) * 0.001);
    //     let start = end;

    //     let img_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
    //         wasm_bindgen::Clamped(&data),
    //         frame.width as u32,
    //         frame.height as u32,
    //     )
    //     .unwrap_ji();

    //     ctx.put_image_data(&img_data, frame.left.into(), frame.top.into());

    //     let end = web_sys::window().unwrap().performance().unwrap_ji().now();
    //     log::info!("put image time: {} secs", (end - start) * 0.001);
    //     let start = end;
    // }
    // let end = web_sys::window().unwrap().performance().unwrap_ji().now();
}