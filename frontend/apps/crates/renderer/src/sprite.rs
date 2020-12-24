use crate::{
    Renderer,
    aliases::*,
};
use awsm_web::webgl::Id;
use shipyard::*;
use shipyard_scenegraph::prelude::*;

pub struct Sprite {
    pub texture_id: Id
}

impl Renderer {
    pub fn add_sprite(&self, texture_id:Id, parent: Option<Parent<SceneGraph>>) {
        log::info!("TODO - add {:?}", texture_id);
    }
}