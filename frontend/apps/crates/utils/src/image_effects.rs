use web_sys::{window, ImageData, HtmlImageElement, HtmlCanvasElement, CanvasRenderingContext2d, Blob};
use awsm_web::canvas::{get_2d_context, Canvas2dContextOptions};
use crate::{prelude::*, path::image_lib_url};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures::channel::oneshot::{channel, Receiver, Sender};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use shared::{
    api::{ApiEndpoint, endpoints},
    error::{EmptyError, MetadataNotFound}, 
    media::{MediaLibrary, PngImageFile},
    domain::{
        image::*,
        meta::*,
        jig::module::body::Image
    },
};

pub struct ImageEffect {
    pub src: Image,
    pub image_data_vec: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub canvas: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d
}

impl ImageEffect {
    pub async fn new(src: Image) -> Self {
        let url = image_lib_url(src.lib.clone(), PngImageFile::Resized, src.id.clone());
        let img = awsm_web::loaders::image::load(url).await.unwrap_ji();

        let width = img.natural_width() as usize;
        let height = img.natural_height() as usize;
        let canvas:HtmlCanvasElement = window()
            .unwrap_ji()
            .document()
            .unwrap_ji()
            .create_element("canvas")
            .unwrap_ji()
            .unchecked_into();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let ctx = get_2d_context(&canvas, None).unwrap_ji();

        ctx.draw_image_with_html_image_element(&img, 0.0, 0.0).unwrap_ji();

        let image_data_vec = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap_ji()
            .data()
            .to_vec();

        Self {
            src,
            image_data_vec,
            width,
            height,
            canvas,
            ctx
        }
    }

    pub fn do_remove_white(&mut self) {
        const THRESHHOLD:u8 = 250;

        let data = &mut self.image_data_vec; 

        let width = self.width;
        let height = self.height;
        for x in 0..width {
            for y in 0..height {
                let offset = y * (width * 4) + x * 4;
                let r = data[offset + 0];
                let g = data[offset + 1];
                let b = data[offset + 2];
                let a = &mut data[offset + 3];

                if r > THRESHHOLD && g > THRESHHOLD && b > THRESHHOLD {
                    *a = 0;
                }
            }
        }

    }

    pub fn do_flip_horizontal(&mut self) {
        /*
    if(dir === "horizontal") {
        for(let srcCol = 0; srcCol < width; srcCol++) {
            const dstCol = (width - srcCol);

            for(let row = 0; row < height; row++) {
                const offset = row * (width * 4) + srcCol * 4;
                const offsetFlip = row * (width * 4) + dstCol * 4;
                imageDest.data[offsetFlip + 0] = imageData.data[offset + 0] ;
                imageDest.data[offsetFlip + 1] = imageData.data[offset + 1] ;
                imageDest.data[offsetFlip + 2] = imageData.data[offset + 2] ;
                imageDest.data[offsetFlip + 3] = imageData.data[offset + 3] ;
            }
        }
    } else {
        for(let srcRow = 0; srcRow < height; srcRow++) {
            const dstRow = (height - srcRow);

            for(let col = 0; col < width; col++) {
                const offset = srcRow * (width * 4) + col * 4;
                const offsetFlip = dstRow * (width * 4) + col * 4;
                imageDest.data[offsetFlip + 0] = imageData.data[offset + 0] ;
                imageDest.data[offsetFlip + 1] = imageData.data[offset + 1] ;
                imageDest.data[offsetFlip + 2] = imageData.data[offset + 2] ;
                imageDest.data[offsetFlip + 3] = imageData.data[offset + 3] ;
            }
        }
    }
    */
        /*
        let data = &mut self.image_data_vec; 
        let Self {width, height, ..} = self;

        for src_col in 0..width {
            let dest_col = width - src_col;

            for row in 0..self.height {
                let offset = row * (width * 4) + src_col * 4;
                let offset_flip = row * (width * 4) + dst_col * 4;
                imageDest.data[offsetFlip + 0] = imageData.data[offset + 0] ;
                imageDest.data[offsetFlip + 1] = imageData.data[offset + 1] ;
                imageDest.data[offsetFlip + 2] = imageData.data[offset + 2] ;
                imageDest.data[offsetFlip + 3] = imageData.data[offset + 3] ;
            }
        }
        */

    }
    pub fn finalize(&self) {
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&self.image_data_vec), self.width as u32, self.height as u32).unwrap_ji();

        self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap_ji();
    }

    pub fn to_data_url(&self) -> String {
        self.canvas.to_data_url().unwrap_ji()
    }
    pub async fn to_blob(&self) -> Blob {
        CanvasToBlobFuture::new(self.canvas.clone()).await
    }

    pub async fn to_blob_url(&self) -> String {
        let blob = self.to_blob().await;
        web_sys::Url::create_object_url_with_blob(&blob).unwrap_ji()
    }

    pub async fn upload(&self) -> Result<Image, String> {
        let blob = self.to_blob().await;


        let req = ImageCreateRequest {
            name: "".to_string(),
            description: "".to_string(),
            is_premium: false,
            publish_at: None,
            tags: Vec::new(),
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            categories: Vec::new(),
            kind: ImageKind::Sticker,
        };

        match api_with_auth::<CreateResponse, MetadataNotFound, _>(endpoints::image::Create::PATH, endpoints::image::Create::METHOD, Some(req)).await {
            Ok(resp) => {
                let CreateResponse { id } = resp;

                let path = endpoints::image::Upload::PATH.replace("{id}", &id.0.to_string());
                match api_upload_blob(&path, &blob, endpoints::image::Upload::METHOD).await {
                    Ok(_) => {
                        //FIXME should be user lib!
                        Ok(Image { id, lib: MediaLibrary::Global})
                    },
                    Err(_) => {
                        Err("error uploading!".to_string())
                    }
                }
            },
            Err(_) => {
                Err("error creating image db!".to_string())
            }
        }
    }
}


pub struct CanvasToBlobFuture {
    pub canvas: HtmlCanvasElement,
    state: CanvasToBlobState,
    closure: Option<Closure<dyn FnMut(Blob)>>,
}

impl CanvasToBlobFuture {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self {
            canvas,
            state: CanvasToBlobState::Empty,
            closure: None
        }
    }
}

enum CanvasToBlobState {
    Empty,
    Loading {
        receiver: Receiver<(Blob)>,
    },
}

impl Future for CanvasToBlobFuture {
    type Output = Blob;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        match &mut self.state {
            CanvasToBlobState::Empty => {
                //success callback
                let waker = ctx.waker().clone();
                let (sender, receiver): (Sender<Blob>, Receiver<Blob>) = channel();
                let mut sender = Option::from(sender);
                let closure = Closure::wrap(Box::new(move |blob| {
                    sender.take().unwrap_throw().send(blob).unwrap_throw();
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(Blob)>);

                self.canvas.to_blob(closure.as_ref().unchecked_ref());

                self.state = CanvasToBlobState::Loading {
                    receiver,
                };
                self.closure = Some(closure);

                //notify the task that we're now loading
                ctx.waker().wake_by_ref();

                Poll::Pending
            }

            CanvasToBlobState::Loading { receiver } => {
                //if let Poll::Ready(value) = Receiver::poll(Pin::new(receiver_err), ctx) {

                let mut is_cancelled = false;

                let state = match receiver.try_recv() {
                    Ok(result) => result,
                    _ => {
                        is_cancelled = true;
                        None
                    }
                };

                if let Some(blob) = state {
                    Poll::Ready(blob.clone())
                } else {
                    if !is_cancelled {
                        //ctx.waker().wake_by_ref();
                    }
                    Poll::Pending
                }
            }
        }
    }
}
