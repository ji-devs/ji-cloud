use shared::domain::jig::module::body::{Image, Transform, _groups::design::Sprite};
use utils::prelude::*;

pub trait SpriteExt {
    fn new(image: Image) -> Self;
}

impl SpriteExt for Sprite {
    /// Create a new Sprite
    fn new(image: Image) -> Self {
        Self {
            image,
            transform: Transform::identity(),
            effects: Vec::new(),
            flip_horizontal: false,
            flip_vertical: false
        }
    }
}
