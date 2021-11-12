use shared::domain::jig::module::body::legacy::design::Sticker as RawSticker;

use std::rc::Rc;

use crate::base::state::Base;

use super::animation::AnimationPlayer;
use super::image::ImagePlayer;

pub enum Sticker {
    Image(Rc<ImagePlayer>),
    Animation(Rc<AnimationPlayer>),
}

impl Sticker {
    pub fn new(base: Rc<Base>, raw: RawSticker) -> Self {
        match raw.animation.clone() {
            Some(animation) => Self::Animation(AnimationPlayer::new(base, raw, animation)),
            None => Self::Image(ImagePlayer::new(base, raw)),
        }
    }
}
