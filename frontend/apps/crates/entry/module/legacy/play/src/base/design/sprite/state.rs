use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite
};
use std::{cell::RefCell, rc::Rc};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, window};
use crate::base::state::Base;
use super::player::SpritePlayer;
use super::animation::Animation;
use std::io::Cursor;

pub struct Sprite {
    pub base: Rc<Base>,
    pub raw: RawSprite,
    pub ctx: RefCell<Option<CanvasRenderingContext2d>>,
    pub player: RefCell<Option<SpritePlayer>>,
    pub data: Mutable<Option<SpriteData>>,
    pub size: Mutable<Option<(f64, f64)>>,
    pub phase: Mutable<SpritePhase>
}

impl Sprite {
    pub fn new(base: Rc<Base>, raw: RawSprite) -> Rc<Self> {
        Rc::new(Self{
            base,
            raw,
            ctx: RefCell::new(None),
            player: RefCell::new(None),
            data: Mutable::new(None),
            size: Mutable::new(None),
            phase: Mutable::new(SpritePhase::Setup),
        })
    }
}

#[derive(Clone)]
pub enum SpriteData {
    Static(HtmlImageElement),
    Animation(Rc<Animation>)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpritePhase {
    Setup,
    PlayStatic,
}