use crate::aliases::*;
use crate::Renderer;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    webgl::{
        WebGl2Renderer,
        BufferMask,
    }
};

impl Renderer {
    pub fn render(&self) {
        let renderer = self.world.borrow::<Gl>().unwrap();
        renderer.gl.clear_color(0.3, 0.0, 0.0, 1.0);

        renderer.clear(&[
            BufferMask::ColorBufferBit,
            BufferMask::DepthBufferBit,
        ]);

    }

}