use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use shared::{domain::{image::ImageId, jig::module::body::{Sprite as RawSprite, Transform}}, media::MediaLibrary};
use std::cell::RefCell;
use crate::transform::state::{TransformState, TransformCallbacks};
use utils::resize::resize_info_signal;

#[derive(Clone)]
pub struct Sprite {
    pub id: ImageId,
    pub lib: MediaLibrary,
    pub transform: Rc<TransformState>,
}

impl Sprite {
    pub fn new(raw:&RawSprite, on_transform_finished: Option<impl Fn(Transform) + 'static>) -> Self {
        let raw = raw.clone();
        let transform_callbacks = TransformCallbacks::new(
            on_transform_finished,
            None::<fn()>
        );
        Self {
            id: raw.id,
            lib: raw.lib,
            transform: Rc::new(TransformState::new(raw.transform, None, true, transform_callbacks)),
        }
    }

    pub fn to_raw(&self) -> RawSprite {
        RawSprite {
            id: self.id,
            lib: self.lib,
            transform: self.transform.get_inner_clone()
        }
    }

    pub fn width_signal(&self) -> impl Signal<Item = String> {
        width_signal(self.transform.size.signal_cloned())
    }
    pub fn height_signal(&self) -> impl Signal<Item = String> {
        height_signal(self.transform.size.signal_cloned())
    }


    pub fn loaded_signal(&self) -> impl Signal<Item = bool> {
        self.transform.size.signal_cloned().map(|size| size.is_some())
    }

}

pub fn width_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| {
        match size {
            None => "0".to_string(),
            Some(size) => format!("{}rem", size.0)
        }
        
    })
}

pub fn height_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| {
        match size {
            None => "0".to_string(),
            Some(size) => format!("{}rem", size.1)
        }
        
    })
}
