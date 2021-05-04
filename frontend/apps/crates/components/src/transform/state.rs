use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use dominator::clone;
use shared::domain::jig::module::body::Transform;
use utils::{prelude::*, drag::Drag};
use std::cell::RefCell;
use utils::resize::{resize_info_signal, ResizeInfo};

pub struct TransformState {
    pub visible: Mutable<bool>,
    pub transform: Mutable<Transform>,
    pub drag: Mutable<Option<Drag>>, 
    pub action: RefCell<Option<Action>>,
    pub rot_stash: RefCell<Option<InitRotation>>,
    pub scale_stash: RefCell<Option<InitScale>>,
    pub size: Mutable<Option<(f64, f64)>>,
}

pub struct InitRotation {
    pub vec_to_center: [f64;2],
}
pub struct InitScale {
    pub vec_to_tp: [f64;2],
    pub transform: Transform,
}

impl TransformState {
    pub fn new(transform:Transform, size: Option<(f64, f64)>) -> Self {
        Self {
            visible: Mutable::new(true),
            size: Mutable::new(size),
            transform: Mutable::new(transform),
            drag: Mutable::new(None),
            action: RefCell::new(None),
            rot_stash: RefCell::new(None),
            scale_stash: RefCell::new(None),
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
                transform.to_screen_mat4(resize_info).as_matrix_string()
            }
        }
    }

    pub fn get_center(&self, resize_info:&ResizeInfo) -> (f64, f64) {
        let transform = self.transform.lock_ref();

        let (pos_x, pos_y) = resize_info.get_pos_denormalized(transform.translation.0[0], transform.translation.0[1]);
        let size = self.size.get_cloned().unwrap_ji();

        let (size_x, size_y) = (size.0 * resize_info.scale, size.1 * resize_info.scale);
        let mid_x = pos_x + (size_x / 2.0);
        let mid_y = pos_y + (size_y / 2.0);

        (mid_x, mid_y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Move,
    Rotate,
    Scale(ScaleFrom, Maintain)
}

pub type Maintain = bool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleFrom {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
