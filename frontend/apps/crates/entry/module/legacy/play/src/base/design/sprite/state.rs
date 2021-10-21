use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite,
    Animation
};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, window};
use crate::base::state::Base;
use std::io::Cursor;

pub enum Sprite {
    Image(Rc<ImagePlayer>),
    Animation(Rc<AnimationPlayer>),
}

impl Sprite {
    pub fn new(base: Rc<Base>, raw: RawSprite) -> Self {
        match raw.animation.clone() {
            Some(animation) => {
                Self::Animation(AnimationPlayer::new(base, raw, animation))
            },
            None => {
                Self::Image(ImagePlayer::new(base, raw))
            }
        }
    }
}

pub struct ImagePlayer {
    pub base: Rc<Base>,
    pub raw: RawSprite,
    pub size: Mutable<Option<(f64, f64)>>,
    pub hide: HideController
}

impl ImagePlayer {
    pub fn new(base: Rc<Base>, raw: RawSprite) -> Rc<Self> {

        let hide = HideController::new(&raw);

        Rc::new(Self{
            base,
            raw,
            size: Mutable::new(None),
            hide
        })
    }
}

pub struct AnimationPlayer {
    pub base: Rc<Base>,
    pub raw: RawSprite,
    pub size: Mutable<Option<(f64, f64)>>,
    pub hide: HideController,
    pub anim: AnimationController
}

impl AnimationPlayer {
    pub fn new(base: Rc<Base>, raw: RawSprite, animation: Animation) -> Rc<Self> {

        let hide = HideController::new(&raw);
        let anim = AnimationController::new(&raw, animation);

        Rc::new(Self{
            base,
            raw,
            size: Mutable::new(None),
            hide,
            anim
        })
    }
}

pub struct AnimationController {
    pub playing: Mutable<bool>,
    pub settings: Animation,
}

impl AnimationController {
    pub fn new(raw: &RawSprite, settings: Animation) -> Self {
        Self {
            playing: Mutable::new(!settings.tap),
            settings
        }
    }
}
pub struct HideController {
    pub is_hidden: AtomicBool,
    pub has_toggled_once: AtomicBool,
}

impl HideController {
    pub fn new(raw: &RawSprite) -> Self {
        Self {
            is_hidden: AtomicBool::new(raw.hide),
            has_toggled_once: AtomicBool::new(false),
        }
    }
}