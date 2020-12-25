use crate::prelude::*;
use crate::geom::static_init::StaticGeometry;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::{
        Id,
        get_webgl_context_2, 
        WebGlContextOptions, 
        WebGl2Renderer,
        get_texture_size,
        WebGlTextureSource,
        ResizeStrategy
    }
};


pub struct Renderer {
    pub world: Rc<World>,
    pub static_geometry: StaticGeometry,
    pub(crate) resize_observer: ResizeObserver,
    pub(crate) textures: Textures,
}


impl Drop for Renderer {
    fn drop(&mut self) {
        log::info!("renderer dropped!");
    }
}

impl Renderer {
    pub fn new(canvas:HtmlCanvasElement, world: Option<Rc<World>>) -> Self {

        let world = world.unwrap_or_else(|| Rc::new(World::new()));


        // create scenegraph
        init_scenegraph(&world);

        // Prep renderer
        let gl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw();

        let mut gl = WebGl2Renderer::new(gl).unwrap_throw();
        
        // Static Geometry
        let static_geometry = StaticGeometry::new(&mut gl).unwrap_throw();

        // Add it to the world
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
            static_geometry,
            textures: Textures::new()
        }
    }

}
