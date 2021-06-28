use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use shared::{domain::{image::ImageId, jig::module::body::{Image, Transform, _groups::design::{SpriteEffect, Sprite as RawSprite}}}, media::MediaLibrary};
use std::cell::RefCell;
use crate::transform::state::{TransformState, TransformCallbacks};
use utils::resize::resize_info_signal;

#[derive(Clone)]
pub struct Sprite {
    pub image: Mutable<Image>,
    pub transform: Rc<TransformState>,
    pub src: Mutable<Option<String>>, //Updated dynamically based on effects
    pub effects: Mutable<Vec<SpriteEffect>>,
    pub flip_horizontal: Mutable<bool>,
    pub flip_vertical: Mutable<bool>,
}



impl Sprite {
    pub fn new(raw:&RawSprite, on_transform_finished: Option<impl Fn(Transform) + 'static>) -> Self {
        let raw = raw.clone();
        let transform_callbacks = TransformCallbacks::new(
            on_transform_finished,
            None::<fn()>
        );
        Self {
            image: Mutable::new(raw.image),
            transform: Rc::new(TransformState::new(raw.transform, None, true, transform_callbacks)),
            src: Mutable::new(None),
            effects: Mutable::new(raw.effects),
            flip_horizontal: Mutable::new(raw.flip_horizontal),
            flip_vertical: Mutable::new(raw.flip_vertical),
        }
    }

    pub fn to_raw(&self) -> RawSprite {
        RawSprite {
            image: self.image.get_cloned(),
            transform: self.transform.get_inner_clone(),
            effects: self.effects.get_cloned(),
            flip_horizontal: self.flip_horizontal.get(),
            flip_vertical: self.flip_vertical.get(),
        }
    }

    pub fn width_signal(&self) -> impl Signal<Item = String> {
        width_signal(self.transform.size.signal_cloned())
    }
    pub fn height_signal(&self) -> impl Signal<Item = String> {
        height_signal(self.transform.size.signal_cloned())
    }

    pub fn inner_transform_signal(&self) -> impl Signal<Item = String> {
        map_ref! {
            let flip_horizontal = self.flip_horizontal.signal(),
            let flip_vertical = self.flip_vertical.signal()
                => {
                    let x = if *flip_horizontal { -1 } else { 1 };
                    let y = if *flip_vertical { -1 } else { 1 };

                    format!("scaleX({}) scaleY({})", x, y)
                }
        }
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
