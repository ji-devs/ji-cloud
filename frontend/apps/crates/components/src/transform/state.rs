use futures_signals::signal::{Mutable, Signal, SignalExt};

use shared::domain::module::body::Transform;
use std::cell::RefCell;
use std::rc::Rc;
use utils::{drag::Drag, math::bounds, prelude::*, resize::get_resize_info};
use utils::{
    math::{transform_signals, BoundsF64, OobbF64},
    resize::ResizeInfo,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, HtmlElement};

pub const MOVE_MULTIPLIER: f64 = 10.0;
pub const MOVE_AMOUNT_PX: f64 = 1.0;

pub struct TransformState {
    pub size: Mutable<Option<(f64, f64)>>,
    pub menu_pos: Mutable<Option<(f64, f64)>>,
    pub coords_in_center: bool,
    pub is_transforming: Mutable<bool>,
    pub(super) transform: Mutable<Transform>,
    pub(super) drag: Mutable<Option<Drag>>,
    pub(super) action: RefCell<Option<Action>>,
    pub(super) rot_stash: RefCell<Option<InitRotation>>,
    pub(super) scale_stash: RefCell<Option<InitScale>>,
    pub(super) shift_pressed: RefCell<bool>,
    pub(super) alt_pressed: RefCell<bool>,
    pub(super) dom_ref: RefCell<Option<TransformBoxElement>>,
    pub(super) callbacks: TransformCallbacks,
    pub(super) overlay_drag_elem: Rc<RefCell<Option<HtmlElement>>>,
}

pub struct TransformCallbacks {
    pub on_action_finished: Option<Box<dyn Fn(Transform)>>,
    pub on_double_click: Option<Box<dyn Fn()>>,
    pub on_blur: Option<Box<dyn Fn()>>,
}

impl TransformCallbacks {
    pub fn new(
        on_action_finished: Option<impl Fn(Transform) + 'static>,
        on_double_click: Option<impl Fn() + 'static>,
        on_blur: Option<impl Fn() + 'static>,
    ) -> Self {
        Self {
            on_action_finished: on_action_finished.map(|f| Box::new(f) as _),
            on_double_click: on_double_click.map(|f| Box::new(f) as _),
            on_blur: on_blur.map(|f| Box::new(f) as _),
        }
    }
}

pub struct InitRotation {
    pub vec_to_point: [f64; 2],
}
pub struct InitScale {
    pub basis_vec_x: [f64; 2],
    pub basis_vec_y: [f64; 2],
    pub vec_to_point: [f64; 2],
    pub scale: (f64, f64),
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = TransformBox)]
    pub(super) type TransformBoxElement;

    #[wasm_bindgen(method, js_name = getDotBounds)]
    fn get_dot_bounds(this: &TransformBoxElement) -> js_sys::Array;
}

impl TransformState {
    pub fn new(
        transform: Transform,
        size: Option<(f64, f64)>,
        coords_in_center: bool,
        callbacks: TransformCallbacks,
    ) -> Self {
        Self {
            coords_in_center,
            size: Mutable::new(size),
            transform: Mutable::new(transform),
            drag: Mutable::new(None),
            action: RefCell::new(None),
            rot_stash: RefCell::new(None),
            scale_stash: RefCell::new(None),
            shift_pressed: RefCell::new(false),
            alt_pressed: RefCell::new(false),
            is_transforming: Mutable::new(false),
            menu_pos: Mutable::new(None),
            callbacks,
            dom_ref: RefCell::new(None),
            overlay_drag_elem: Rc::new(RefCell::new(None)),
        }
    }

    pub fn reset(&self) {
        self.transform.set(Transform::identity());
    }

    pub fn get_inner_clone(&self) -> Transform {
        self.transform.get_cloned()
    }
    pub fn get_inner_signal_cloned(&self) -> impl Signal<Item = Transform> {
        self.transform.signal_cloned()
    }
    pub fn get_inner_mutable(&self) -> Mutable<Transform> {
        self.transform.clone()
    }

    /// this is very slow! only used in rare cases where we need
    /// to calculate the position of the dots
    /// after the transform is applied
    /// it can probably be replaced by pure rust code
    /// and a lot of the work for that has been done in actions::get_basis_vectors
    ///
    /// as of this writing, the *only* place it's used is in the
    /// trace floating menu placement
    pub fn get_dom_rects(&self) -> Option<Vec<DomRect>> {
        self.dom_ref.borrow().as_ref().map(|element| {
            let values = element.get_dot_bounds();
            let len = values.length();
            let mut output = Vec::with_capacity(len as usize);

            for i in 0..len {
                let value = values.get(i);
                let value: DomRect = value.unchecked_into();
                output.push(value);
            }

            output
        })
    }

    /// this is also very slow! see above
    pub fn get_dom_rect_bounds(&self) -> Option<BoundsF64> {
        self.get_dom_rects().and_then(|rects| {
            //Set to inverse of max values
            let mut left: f64 = f64::MAX;
            let mut right: f64 = f64::MIN;
            let mut top: f64 = f64::MAX;
            let mut bottom: f64 = f64::MIN;
            for rect in rects.iter() {
                if rect.left() < left {
                    left = rect.left();
                }

                if rect.right() > right {
                    right = rect.right();
                }

                if rect.top() < top {
                    top = rect.top();
                }

                if rect.bottom() > bottom {
                    bottom = rect.bottom();
                }
            }

            let resize_info = get_resize_info();

            let (x, y) = resize_info.get_pos_px(left, top);

            let width = right - left;
            let height = bottom - top;

            if width > 0.0 && height > 0.0 {
                Some(BoundsF64 {
                    x,
                    y,
                    width,
                    height,
                    invert_y: true,
                })
            } else {
                None
            }
        })
    }

    pub fn get_aabb_no_rotation_bounds_px(&self, coords_in_center: bool) -> BoundsF64 {
        let resize_info = get_resize_info();
        let size = self.size.get_cloned();

        bounds::aabb_no_rotation_transform_px(
            coords_in_center,
            &self.get_inner_clone(),
            size,
            &resize_info,
        )
    }

    pub fn get_oobb_bounds_px(&self, coords_in_center: bool) -> OobbF64 {
        let resize_info = get_resize_info();
        let size = self.size.get_cloned();

        bounds::oobb_transform_px(
            coords_in_center,
            &self.get_inner_clone(),
            size,
            &resize_info,
        )
    }

    pub fn get_x_px(&self, coords_in_center: bool) -> f64 {
        self.get_aabb_no_rotation_bounds_px(coords_in_center).x
    }
    pub fn get_y_px(&self, coords_in_center: bool) -> f64 {
        self.get_aabb_no_rotation_bounds_px(coords_in_center).y
    }
    pub fn get_width_px(&self, coords_in_center: bool) -> f64 {
        self.get_aabb_no_rotation_bounds_px(coords_in_center).width
    }
    pub fn get_height_px(&self, coords_in_center: bool) -> f64 {
        self.get_aabb_no_rotation_bounds_px(coords_in_center).height
    }

    pub fn x_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::x_px(
            self.coords_in_center,
            self.transform.signal_cloned(),
            self.size.signal_cloned(),
        )
    }

    pub fn y_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::y_px(
            self.coords_in_center,
            self.transform.signal_cloned(),
            self.size.signal_cloned(),
        )
    }
    pub fn width_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::width_px(
            self.coords_in_center,
            self.transform.signal_cloned(),
            self.size.signal_cloned(),
        )
    }
    pub fn height_px_signal(&self) -> impl Signal<Item = f64> {
        transform_signals::height_px(
            self.coords_in_center,
            self.transform.signal_cloned(),
            self.size.signal_cloned(),
        )
    }
    pub fn native_width_signal(&self) -> impl Signal<Item = f64> {
        self.size.signal_cloned().map(|size| match size {
            None => 0.0,
            Some(size) => size.0,
        })
    }
    pub fn native_height_signal(&self) -> impl Signal<Item = f64> {
        self.size.signal_cloned().map(|size| match size {
            None => 0.0,
            Some(size) => size.1,
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

    pub fn get_center(&self, resize_info: &ResizeInfo) -> (f64, f64) {
        let transform = self.transform.lock_ref();

        let (pos_x, pos_y) = resize_info
            .get_pos_denormalized(transform.translation.0[0], transform.translation.0[1]);
        let size = self.size.get_cloned().unwrap_ji();

        let (width, height) = (size.0 * resize_info.scale, size.1 * resize_info.scale);
        let mut mid_x = pos_x + (width / 2.0);
        let mut mid_y = pos_y + (height / 2.0);

        if self.coords_in_center {
            mid_x += (resize_info.width - width) / 2.0;
            mid_y += (resize_info.height - height) / 2.0;
        }

        (mid_x, mid_y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Move,
    Rotate,
    Scale(ScaleFrom, LockAspect),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Key {
    ArrowLeft,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    Shift,
    Alt,
    Other,
}

impl Key {
    pub fn is_move_key(&self) -> bool {
        match self {
            Self::ArrowLeft | Self::ArrowRight | Self::ArrowUp | Self::ArrowDown => true,
            _ => false,
        }
    }
    pub fn translation_from_key(&self) -> (f64, f64) {
        let resize_info = get_resize_info();
        match self {
            Self::ArrowLeft => resize_info.get_px_normalized(-MOVE_AMOUNT_PX, 0.0),
            Self::ArrowRight => resize_info.get_px_normalized(MOVE_AMOUNT_PX, 0.0),
            Self::ArrowUp => resize_info.get_px_normalized(0.0, -MOVE_AMOUNT_PX),
            Self::ArrowDown => resize_info.get_px_normalized(0.0, MOVE_AMOUNT_PX),
            _ => (0.0, 0.0),
        }
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ArrowLeft" => Self::ArrowLeft,
            "ArrowUp" => Self::ArrowUp,
            "ArrowRight" => Self::ArrowRight,
            "ArrowDown" => Self::ArrowDown,
            "Shift" => Self::Shift,
            "Alt" => Self::Alt,
            _ => Self::Other,
        }
    }
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

pub enum ResizeLevel {
    Full,
    None,
    KeepAspectRatio,
}
impl ResizeLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            ResizeLevel::Full => "full",
            ResizeLevel::None => "none",
            ResizeLevel::KeepAspectRatio => "keep-aspect-ratio",
        }
    }
}
