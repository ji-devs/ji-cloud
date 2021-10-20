use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::{canvas::{get_2d_context, CanvasToBlobFuture}, data::ArrayBufferExt};
use image::{ImageFormat, io::Reader as ImageReader};
use std::io::Cursor;
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
pub struct Animation {
    pub frames: Vec<Frame>,
    pub width: f64,
    pub height: f64
}

impl Animation {
    pub async fn load_gif(url: &str) -> Self {
        let bytes = match awsm_web::loaders::fetch::fetch_url(&url).await {
            Ok(resp) => resp
                .array_buffer()
                .await
                .expect_ji("could not load image!!")
                .to_vec_u8(),
            Err(_) => {
                panic!("could not load image!");
            }
        };

        let start = web_sys::window().unwrap().performance().unwrap_ji().now();
        let mut decoder = GifDecoder::new(Cursor::new(bytes)).unwrap_ji();
        let (width, height) = decoder.dimensions();
        let frame = decoder.into_frames().next().unwrap_ji().unwrap_ji();
        let frames = vec![frame];
        //let frames = decoder.into_frames().collect_frames().unwrap_ji();
        let end = web_sys::window().unwrap().performance().unwrap_ji().now();
        log::info!("decoder time: {} secs", (end - start) * 0.001);


        Self {
            frames,
            width: width as f64,
            height: height as f64
        }
        // let start = web_sys::window().unwrap().performance().unwrap_ji().now();

        // let mut options = gif::DecodeOptions::new();
        // options.set_color_output(gif::ColorOutput::RGBA);

        // let mut decoder = options.read_info(bytes.as_slice()).unwrap();

        // let global_palette = decoder.global_palette().map(|x| x.to_vec());
        // loop {
        //     match decoder.read_next_frame().unwrap_ji() {
        //         Some(frame) => {
                    
        //         },
        //         None => { break; }
        //     }
        // }

        // let end = web_sys::window().unwrap().performance().unwrap_ji().now();
        // log::info!("decoder time: {} secs", (end - start) * 0.001);

    //     let start = end;

    // let start = web_sys::window().unwrap().performance().unwrap_ji().now()

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
    }
}