//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod init;
mod aliases;
mod render;
mod textures;
mod sprite;

use awsm_web::dom::resize::ResizeObserver;
use shipyard::World;
use std::rc::Rc;
use textures::Textures;

pub struct Renderer {
    pub world: Rc<World>,
    pub(crate) resize_observer: ResizeObserver,
    pub(crate) textures: Textures,
}