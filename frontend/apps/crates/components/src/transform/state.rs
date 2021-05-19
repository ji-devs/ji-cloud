use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use dominator::clone;
use shared::domain::jig::module::body::Transform;
use utils::{prelude::*, drag::Drag};
use std::cell::RefCell;
use utils::{
    resize::{resize_info_signal, ResizeInfo},
    math::{self, BoundsF64}
};
use web_sys::HtmlElement;

//If coordinates should be from the center of the stage
//instead of top-left
const COORDS_IN_CENTER:bool = true;

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

    pub fn left_center_rem_signal(&self) -> impl Signal<Item = String> {
        self.center_rem_signal()
            .map(|center| {
                match center {
                    None => "0".to_string(),
                    Some(center) => format!("{}rem", center.0)
                }
            })
    }
    pub fn top_center_rem_signal(&self) -> impl Signal<Item = String> {
        self.center_rem_signal()
            .map(|center| {
                match center {
                    None => "0".to_string(),
                    Some(center) => format!("{}rem", center.1)
                }
            })
    }

    fn center_rem_signal(&self) -> impl Signal<Item = Option<(f64, f64)>> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let size = self.size.signal_cloned()
                => {
                    size.map(|(width, height)| {
                        let (full_width, full_height) = resize_info.full_size();
                        
                        (
                            (full_width - width) / 2.0,
                            (full_height - height) / 2.0,
                        )

                    })
                }
        }
    }


    pub fn get_inner_clone(&self) -> Transform {
        self.transform.get_cloned()
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

    //Gives us the bounds of the box itself without rotation
    pub fn bounds_px_signal(&self) -> impl Signal<Item = BoundsF64> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let transform = self.transform.signal_cloned(),
            let size = self.size.signal_cloned()
            => move {
                

                if let Some(size) = size {
                    let (mut x, mut y) = transform
                        .map(|t| {
                            let mut t = t.clone();
                            t.set_rotation_identity();
                            t.denormalize_translation(resize_info);
                            t.get_translation_2d()
                        });

                    let (scale_x, scale_y) =  transform.get_scale_2d(); 
                    let (native_width, native_height) = *size;

                    //Uhhh.... I don't know... it works though
                    //change at your own risk!
                    let rel_width = native_width * resize_info.scale;
                    let width = rel_width * scale_x;

                    let rel_height = native_height * resize_info.scale;
                    let height = rel_height * scale_y;

                    x -= ((width - rel_width)/2.0);
                    y -= ((height - rel_height)/2.0);

                    //only if we want to put it at center
                    if COORDS_IN_CENTER {
                        let center_x = (resize_info.width - rel_width)/2.0;
                        let center_y = (resize_info.height - rel_height)/2.0;
                        x += center_x;
                        y += center_y;
                    }

                    BoundsF64 {
                        x,
                        y,
                        width,
                        height,
                        invert_y: false
                    }
                } else {
                    BoundsF64 {
                        x: 0.0,
                        y: 0.0,
                        width: 0.0,
                        height: 0.0,
                        invert_y: false
                    }
                }
            }
        }
    }

    pub fn x_px_signal(&self) -> impl Signal<Item = f64> {
        self.bounds_px_signal().map(|bounds| {
            //log::info!("{}", bounds.x);
            bounds.x
        })
    }
    pub fn y_px_signal(&self) -> impl Signal<Item = f64> {
        self.bounds_px_signal().map(|bounds| bounds.y)
    }
    pub fn width_px_signal(&self) -> impl Signal<Item = f64> {
        self.bounds_px_signal().map(|bounds| bounds.width)
    }
    pub fn height_px_signal(&self) -> impl Signal<Item = f64> {
        self.bounds_px_signal().map(|bounds| bounds.height)
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

    pub fn matrix_string_signal(&self) -> impl Signal<Item = String> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let transform = self.transform.signal_cloned()
            => {
                transform
                    .map(|t| {
                        let mut t = t.clone();
                        t.denormalize_translation(resize_info);
                        t.to_mat4().as_matrix_string()
                    })
            }
        }
    }


    //CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
    //but set the rotation and translation to identity
    pub fn scale_matrix_string_signal(&self) -> impl Signal<Item = String> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let transform = self.transform.signal_cloned()
            => {
                transform
                    .map(|t| {
                        let mut t = t.clone();
                        t.set_rotation_identity();
                        t.set_translation_identity();
                        t.to_mat4().as_matrix_string()
                    })
            }
        }
    }
    //CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
    //but set the scale and translation to identity
    pub fn rotation_matrix_string_signal(&self) -> impl Signal<Item = String> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let transform = self.transform.signal_cloned()
            => {
                transform
                    .map(|t| {
                        let mut t = t.clone();
                        t.set_scale_identity();
                        t.set_translation_identity();
                        t.to_mat4().as_matrix_string()
                    })
            }
        }
    }
    pub fn invert_rotation_matrix_string_signal(&self) -> impl Signal<Item = String> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let transform = self.transform.signal_cloned()
            => {
                transform
                    .map(|t| {
                        let mut t = t.clone();
                        t.set_scale_identity();
                        t.set_translation_identity();
                        t.rotation.0 = math::quat::invert(&t.rotation.0);
                        t.to_mat4().as_matrix_string()
                    })
            }
        }
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
