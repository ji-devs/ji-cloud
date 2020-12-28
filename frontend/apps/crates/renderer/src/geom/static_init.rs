use awsm_web::webgl::{WebGl2Renderer, Id};
use super::quad::Quad;

pub struct StaticGeometry {
    pub unit_quad: Quad 
}

impl StaticGeometry {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {
        let unit_quad = Quad::new_unit(gl)?;
        
        Ok(Self {
           unit_quad 
        })
    }
}
