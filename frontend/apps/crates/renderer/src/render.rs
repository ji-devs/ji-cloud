use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };

pub type Gl<'a> = NonSendSync<UniqueView<'a, WebGl2Renderer>>;
pub type GlMut<'a> = NonSendSync<UniqueViewMut<'a, WebGl2Renderer>>;

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
