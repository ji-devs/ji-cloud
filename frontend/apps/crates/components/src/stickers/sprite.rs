pub mod dom;
pub mod menu;
pub mod ext;

use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use shared::{domain::{image::ImageId, jig::module::body::{Sprite as RawSprite, Transform}}, media::MediaLibrary};
use std::cell::RefCell;
use crate::transform::state::TransformState;

#[derive(Clone)]
pub struct Sprite {
    pub id: ImageId,
    pub lib: MediaLibrary,
    pub transform: Rc<TransformState>,
    pub is_new: RefCell<bool>,
}

impl Sprite {
    pub fn new(raw:&RawSprite) -> Self {
        let raw = raw.clone();
        Self {
            id: raw.id,
            lib: raw.lib,
            transform: Rc::new(TransformState::new(raw.transform, None)),
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

