use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use dominator::clone;
use shared::domain::jig::module::body::Transform;
use utils::{prelude::*, drag::Drag, math::bounds};
use std::cell::RefCell;
use utils::{
    resize::{resize_info_signal, ResizeInfo},
    math::{self, BoundsF64, transform_signals}
};
use web_sys::HtmlElement;

//If coordinates should be from the center of the stage
//instead of top-left
pub const COORDS_IN_CENTER:bool = true;

pub struct TransformState {
    pub size: Mutable<Option<(f64, f64)>>,
    pub hide_on_dbl_click: RefCell<bool>,
    pub menu_pos: Mutable<Option<(f64, f64)>>, 
    pub(super) transform: Mutable<Transform>,
    pub(super) drag: Mutable<Option<Drag>>, 
    pub(super) action: RefCell<Option<Action>>,
    pub(super) rot_stash: RefCell<Option<InitRotation>>,
    pub(super) scale_stash: RefCell<Option<InitScale>>,
    pub(super) alt_pressed: RefCell<bool>,
    pub(super) rect_hidden: Mutable<bool>, 
    pub(super) menu_button_visible: Mutable<bool>,
    pub(super) on_action_finished: Option<Box<dyn Fn(Transform)>>,
}

pub struct InitRotation {
    pub vec_to_point: [f64;2],
}
pub struct InitScale {
    pub basis_vec_x: [f64;2],
    pub basis_vec_y: [f64;2],
    pub vec_to_point: [f64;2],
    pub scale: (f64, f64) 
}

impl TransformState {
    pub fn new(transform:Transform, size: Option<(f64, f64)>, on_action_finished: Option<impl Fn(Transform) + 'static>) -> Self {
        Self {
            rect_hidden: Mutable::new(false),
            size: Mutable::new(size),
            transform: Mutable::new(transform),
            drag: Mutable::new(None),
            action: RefCell::new(None),
            rot_stash: RefCell::new(None),
            scale_stash: RefCell::new(None),
            hide_on_dbl_click: RefCell::new(false),
            alt_pressed: RefCell::new(false),
            menu_button_visible: Mutable::new(true),
            menu_pos: Mutable::new(None),
            //map doesn't work for some reason..
            on_action_finished: match on_action_finished {
                Some(on_action_finished) => Some(Box::new(on_action_finished)),
                None => None
            }
        }
    }


    pub fn get_inner_clone(&self) -> Transform {
        self.transform.get_cloned()
    }
    pub fn get_inner_signal_cloned(&self) -> impl Signal<Item = Transform> {
        self.transform.signal_cloned()
    }

    pub fn menu_pos_signal(
        &self, 
        active_signal: impl Signal<Item = bool>
    ) -> impl Signal<Item = Option<(f64, f64)>> {
        map_ref! {
            let active = active_signal,
            let pos = self.menu_pos.signal_cloned()
                => {
                    if !*active {
                        None
                    } else {
                        *pos
                    }
                }
        }
    }


    pub fn x_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::x_px(COORDS_IN_CENTER, self.transform.signal_cloned(), self.size.signal_cloned())
    }

    pub fn y_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::y_px(COORDS_IN_CENTER, self.transform.signal_cloned(), self.size.signal_cloned())
    }
    pub fn width_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::width_px(COORDS_IN_CENTER, self.transform.signal_cloned(), self.size.signal_cloned())
    }
    pub fn height_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::height_px(COORDS_IN_CENTER, self.transform.signal_cloned(), self.size.signal_cloned())
    }
    pub fn native_width_signal(&self) -> impl Signal<Item = f64> {
        self.size.signal_cloned().map(|size| {
            match size {
                None => 0.0, 
                Some(size) => size.0, 
            }
        })
    }
    pub fn native_height_signal(&self) -> impl Signal<Item = f64> {
        self.size.signal_cloned().map(|size| {
            match size {
                None => 0.0, 
                Some(size) => size.1, 
            }
        })
    }

    pub fn denormalize_matrix_string_signal(&self) -> impl Signal<Item = String> {
        transform_signals::denormalize_matrix_string(self.transform.signal_cloned())
    }


    //CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
    //but set the rotation and translation to identity
    pub fn scale_matrix_string_signal(&self) -> impl Signal<Item = String> {
        transform_signals::scale_matrix_string(self.transform.signal_cloned())
    }
    //CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
    //but set the scale and translation to identity
    pub fn rotation_matrix_string_signal(&self) -> impl Signal<Item = String> {
        transform_signals::rotation_matrix_string(self.transform.signal_cloned())
    }
    pub fn invert_rotation_matrix_string_signal(&self) -> impl Signal<Item = String> {
        transform_signals::invert_rotation_matrix_string(self.transform.signal_cloned())
    }

    pub fn get_center(&self, resize_info:&ResizeInfo) -> (f64, f64) {
        let transform = self.transform.lock_ref();

        let (pos_x, pos_y) = resize_info.get_pos_denormalized(transform.translation.0[0], transform.translation.0[1]);
        let size = self.size.get_cloned().unwrap_ji();

        let (width, height) = (size.0 * resize_info.scale, size.1 * resize_info.scale);
        let mut mid_x = pos_x + (width / 2.0);
        let mut mid_y = pos_y + (height / 2.0);


        if COORDS_IN_CENTER {
            mid_x += ((resize_info.width - width)/2.0);
            mid_y += ((resize_info.height - height)/2.0);
        }

        (mid_x, mid_y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Move,
    Rotate,
    Scale(ScaleFrom, LockAspect)
}

pub type LockAspect = bool;

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
