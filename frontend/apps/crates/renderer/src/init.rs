use crate::{
    Renderer,
    aliases::*,
    textures::Textures
};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::{
        get_webgl_context_2, 
        WebGlContextOptions, 
        WebGl2Renderer,
        get_texture_size,
        WebGlTextureSource,
        ResizeStrategy
    }
};

impl Drop for Renderer {
    fn drop(&mut self) {
        log::info!("renderer dropped!");
    }
}
impl Renderer {
    pub fn new(canvas:HtmlCanvasElement, world: Option<Rc<World>>) -> Self {

        let world = world.unwrap_or_else(|| Rc::new(World::new()));

        // Prep renderer
        let gl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw();

        let gl = WebGl2Renderer::new(gl).unwrap_throw();
        world.add_unique_non_send_sync(gl).unwrap_throw();

        // Resizing
        let world_clone = world.clone();
        let resize_observer = ResizeObserver::new(move || {
            log::info!("resized!");
        });
        resize_observer.observe(&canvas);

        // Create self
        Self {
            resize_observer,
            world,
            textures: Textures::new()
        }
    }

}