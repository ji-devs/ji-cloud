use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use shared::{domain::{image::ImageId, jig::module::body::{Sprite, Transform}}, media::MediaLibrary};
use std::cell::RefCell;
use components::{
    transform::state::TransformState,
    renderables::sprite::*,
};

#[derive(Clone)]
pub struct Sticker {
    pub id: ImageId,
    pub lib: MediaLibrary,
    pub transform: Rc<TransformState>,
    pub is_new: RefCell<bool>,
}

impl Sticker {
    pub fn new(sprite:&Sprite) -> Self {
        let sprite = sprite.clone();
        Self {
            id: sprite.id,
            lib: sprite.lib,
            transform: Rc::new(TransformState::new(sprite.transform, None)),
            is_new: RefCell::new(true),
        }
    }

    pub fn loaded_signal(&self) -> impl Signal<Item = bool> {
        self.transform.size.signal_cloned().map(|size| size.is_some())
    }

    pub fn width_signal(&self) -> impl Signal<Item = String> {
        self.transform.size.signal_cloned().map(|size| {
            match size {
                None => "0".to_string(),
                Some(size) => format!("{}rem", size.0)
            }
            
        })
    }

    pub fn height_signal(&self) -> impl Signal<Item = String> {
        self.transform.size.signal_cloned().map(|size| {
            match size {
                None => "0".to_string(),
                Some(size) => format!("{}rem", size.1)
            }
        })
    }
}

