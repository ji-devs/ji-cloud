use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use dominator::clone;
use shared::domain::jig::module::body::Transform;
use utils::{prelude::*, drag::Drag};
use std::cell::RefCell;
use utils::resize::resize_info_signal;

pub struct TransformState {
    pub visible: Mutable<bool>,
    pub transform: Mutable<Transform>,
    pub size: Mutable<Option<(f64, f64)>>,
    pub drag: Mutable<Option<Drag>>, 
    pub action: RefCell<Option<Action>>
}

impl TransformState {
    pub fn new(transform:Transform, size: Option<(f64, f64)>) -> Self {
        Self {
            visible: Mutable::new(true),
            size: Mutable::new(size),
            transform: Mutable::new(transform),
            drag: Mutable::new(None),
            action: RefCell::new(None),
        }
    }

    pub fn width_signal(&self) -> impl Signal<Item = f64> {
        self.size.signal_cloned().map(|size| {
            match size {
                None => 0.0, 
                Some(size) => size.0, 
            }
        })
    }
    pub fn height_signal(&self) -> impl Signal<Item = f64> {
        self.size.signal_cloned().map(|size| {
            match size {
                None => 0.0, 
                Some(size) => size.1, 
            }
        })
    }

    pub fn matrix_string_signal(&self) -> impl Signal<Item = String> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let transform = self.transform.signal_cloned()
            => {
                transform.denormalize_2d(resize_info).to_matrix_string()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Move
}
