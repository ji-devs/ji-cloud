use shared::{
    domain::{
        image::ImageId,
        jig::module::body::{Sprite, Transform},
    },
    media::MediaLibrary
};
use utils::prelude::*;

pub trait SpriteExt {
    fn new(id: ImageId, lib: MediaLibrary) -> Self;
}

impl SpriteExt for Sprite {
    /// Create a new Sprite
    fn new(id: ImageId, lib: MediaLibrary) -> Self {
        Self {
            id,
            lib,
            transform: Transform::identity(),
        }
    }
}
