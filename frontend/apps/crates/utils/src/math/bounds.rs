use super::*;
use web_sys::{Element, DomRect};

use shared::domain::jig::module::body::{Vec3, Vec4, Transform};
use crate::resize::ResizeInfo;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use dominator::clone;
use crate::{prelude::*, resize::resize_info_signal};

pub fn size_width_center_rem_signal(size_signal: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    center_rem_signal(size_signal)
        .map(|center| {
            match center {
                None => "0".to_string(),
                Some(center) => format!("{}rem", center.0)
            }
        })
}
pub fn size_height_center_rem_signal(size_signal: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    center_rem_signal(size_signal)
        .map(|center| {
            match center {
                None => "0".to_string(),
                Some(center) => format!("{}rem", center.1)
            }
        })
}

pub fn center_rem_signal(size_signal: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = Option<(f64, f64)>> {
    map_ref! {
        let resize_info = resize_info_signal(),
        let size = size_signal 
            => {
                center_rem(*size, resize_info)
            }
    }
}

// will 0,0 mean centering in the middle of the screen
pub fn center_rem(size: Option<(f64, f64)>, resize_info: &ResizeInfo) -> Option<(f64, f64)> {
    size.map(|(width, height)| {
        let (full_width, full_height) = resize_info.full_size();
        
        (
            (full_width - width) / 2.0,
            (full_height - height) / 2.0,
        )

    })
}


// does not include rotation!
// so if creating a proper bounding box, gotta rotate separately
pub fn transform_px(coords_in_center: bool, transform: &Transform, size: Option<(f64, f64)>, resize_info: &ResizeInfo) -> BoundsF64 {

    if let Some(size) = size {
        let (mut x, mut y) = transform
            .map(|t| {
                let mut t = t.clone();
                t.set_rotation_identity();
                t.denormalize_translation(resize_info);
                t.get_translation_2d()
            });

        let (scale_x, scale_y) =  transform.get_scale_2d(); 
        let (native_width, native_height) = size;

        //Uhhh.... I don't know... it works though
        //change at your own risk!
        let rel_width = native_width * resize_info.scale;
        let width = rel_width * scale_x;

        let rel_height = native_height * resize_info.scale;
        let height = rel_height * scale_y;

        x -= ((width - rel_width)/2.0);
        y -= ((height - rel_height)/2.0);

        //only if we want to put it at center
        if coords_in_center {
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






#[derive(Debug, Clone, Copy)]
pub struct BoundsF64 {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64, 
    pub invert_y: bool,
}

impl BoundsF64 {
    pub fn new_from_dom_normalized(rect:&DomRect, resize_info:&ResizeInfo) -> Self {
        let (x, y) = resize_info.get_pos_normalized(rect.x(), rect.y());
        let (width, height) = (
            rect.width() / resize_info.width, 
            rect.height() / resize_info.height
        ); 

        Self {
            x,
            y,
            width,
            height,
            invert_y: true
        }
    }
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            invert_y: false,
        }
    }

    pub fn top(&self) -> f64 {
        self.y
    }
    pub fn bottom(&self) -> f64 {
        if self.invert_y {
            self.y + self.height
        } else {
            self.y - self.height
        }
    }
    pub fn left(&self) -> f64 {
        self.x
    }
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn middle_horizontal(&self) -> f64 {
        self.x + (self.width/2.0)
    }
    pub fn middle_vertical(&self) -> f64 {
        if self.invert_y {
            self.y + (self.height/2.0)
        } else {
            self.y - (self.height/2.0)
        }
    }

    pub fn middle(&self) -> (f64, f64) {
        (self.middle_horizontal(), self.middle_vertical())
    }

    pub fn contains(&self, other:Self) -> bool {
        if self.invert_y != other.invert_y {
            log::warn!("TODO - handle a case of different coordinate spaces!");
            return false;
        }

        let contains_horiz = (self.left() <= other.left() && self.right() >= other.right());
        let contains_vert = {
            if self.invert_y {
                self.top() <= other.top() && self.bottom() >= other.bottom()
            } else {
                self.top() >= other.top() && self.bottom() <= other.bottom()
            }
        };

        contains_horiz && contains_vert
    }

    pub fn intersects(&self, other:Self) -> bool {
        self.contains_corner(other) || other.contains_corner(*self)
    }

    pub fn contains_corner(&self, other:Self) -> bool {
        if self.invert_y != other.invert_y {
            log::warn!("TODO - handle a case of different coordinate spaces!");
            return false;
        }

        let contains_left = other.left() >= self.left() && other.left() <= self.right();
        let contains_right = other.right() >= self.left() && other.right() <= self.right();
        let contains_top = {
            if self.invert_y {
                other.top() >= self.top() && other.top() <= self.bottom()
            } else {
                other.top() >= self.bottom() && other.top() <= self.top()
            }
        };
        let contains_bottom = {
            if self.invert_y {
                other.bottom() >= self.top() && other.bottom() <= self.bottom()
            } else {
                other.bottom() >= self.bottom() && other.bottom() <= self.top()
            }
        };


        (contains_left && contains_top)
            || (contains_right && contains_top)
            || (contains_left && contains_bottom)
            || (contains_right && contains_bottom)


    }

    pub fn denormalize(&self, resize_info: &ResizeInfo) -> Self {

        let (x, y) = resize_info.get_pos_denormalized(self.x, self.y);

        let (width, height) = resize_info.get_size_denormalized(self.width, self.height);


        Self {
            x,
            y,
            width,
            height,
            invert_y: self.invert_y 
        }
    }
    pub fn denormalize_signal(&self) -> impl Signal<Item = Self> {

        resize_info_signal().map({
            let _self = self.clone();

            move |resize_info| {
                _self.denormalize(&resize_info)
            }
        })
    }
    pub fn denormalize_fixed_signal(&self) -> impl Signal<Item = Self> {

        resize_info_signal().map({
            let _self = self.clone();

            move |resize_info| {
                let mut bounds = _self.denormalize(&resize_info);
                let (pos_x, pos_y) = resize_info.get_fixed_pos_px(bounds.x, bounds.y);
                bounds.x = pos_x;
                bounds.y = pos_y;


                bounds

            }
        })
    }
    pub fn denormalize_x_signal(&self) -> impl Signal<Item = f64> {
        self.denormalize_signal().map(|bounds| bounds.x)
    }
    pub fn denormalize_y_signal(&self) -> impl Signal<Item = f64> {
        self.denormalize_signal().map(|bounds| bounds.y)
    }
    pub fn denormalize_width_signal(&self) -> impl Signal<Item = f64> {
        self.denormalize_signal().map(|bounds| bounds.width)
    }
    pub fn denormalize_height_signal(&self) -> impl Signal<Item = f64> {
        self.denormalize_signal().map(|bounds| bounds.height)
    }
    pub fn denormalize_x_string_signal(&self) -> impl Signal<Item = String> {
        self.denormalize_signal().map(|bounds| format!("{}px", bounds.x))
    }
    pub fn denormalize_y_string_signal(&self) -> impl Signal<Item = String> {
        self.denormalize_signal().map(|bounds| format!("{}px", bounds.y))
    }
    pub fn denormalize_width_string_signal(&self) -> impl Signal<Item = String> {
        self.denormalize_signal().map(|bounds| format!("{}px", bounds.width))
    }
    pub fn denormalize_height_string_signal(&self) -> impl Signal<Item = String> {
        self.denormalize_signal().map(|bounds| format!("{}px", bounds.height))
    }
}

impl From<DomRect> for BoundsF64 {
    fn from(rect:DomRect) -> Self {
        Self {
            x: rect.x(),
            y: rect.y(),
            width: rect.width(),
            height: rect.height(),
            invert_y: true
        }
    }
}
impl From<BoundsF64> for DomRect {
    fn from(bounds:BoundsF64) -> Self {
        DomRect::new_with_x_and_y_and_width_and_height(
            bounds.x,
            bounds.y,
            bounds.width,
            bounds.height
        ).unwrap_ji()
    }
}
impl From<&Element> for BoundsF64 {
    fn from(el:&Element) -> Self {
        Self::from(el.get_bounding_client_rect())
    }
}
impl From<Element> for BoundsF64 {
    fn from(el:Element) -> Self {
        Self::from(el.get_bounding_client_rect())
    }
}

impl From<(PointI32, RectF64)> for BoundsF64 {
    fn from((point, rect):(PointI32, RectF64)) -> Self {
        Self {
            x: point.x as f64,
            y: point.y as f64,
            width: rect.width,
            height: rect.height,
            invert_y: false
        }
    }
}
