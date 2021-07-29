use super::*;
use web_sys::{Element, DomRect};

use shared::domain::jig::module::body::{Vec3, Vec4, Transform};
use crate::resize::ResizeInfo;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use dominator::clone;
use crate::{prelude::*, math, resize::resize_info_signal};


// the transform itself isn't expected to change here
pub fn map_offset(transform:Transform, offset:impl Signal<Item = (f64, f64)> + 'static) -> impl Signal<Item = Transform> {
    offset
        .map(move |(offset_x, offset_y)| {
            transform.map_offset(offset_x, offset_y)
        })
}
/*
 * These are signals mainly due to resize info
 * it's common to pass in an always(transform)
 */
pub fn oobb_bounds_px(
    coords_in_center: bool, 
    transform_signal: impl Signal<Item = Transform>, 
    size_signal: impl Signal<Item = Option<(f64, f64)>>
) -> impl Signal<Item = OobbF64> {
    map_ref! {
        let resize_info = resize_info_signal(),
        let transform = transform_signal,
        let size = size_signal 
        => move {
            super::bounds::oobb_transform_px(coords_in_center, transform, *size, resize_info)
        }
    }
}
pub fn aabb_bounds_px(
    coords_in_center: bool, 
    transform_signal: impl Signal<Item = Transform>, 
    size_signal: impl Signal<Item = Option<(f64, f64)>>
) -> impl Signal<Item = BoundsF64> {
    map_ref! {
        let resize_info = resize_info_signal(),
        let transform = transform_signal,
        let size = size_signal 
        => move {
            super::bounds::aabb_transform_px(coords_in_center, transform, *size, resize_info)
        }
    }
}

pub fn x_px(
    coords_in_center: bool, 
    transform_signal: impl Signal<Item = Transform>, 
    size_signal: impl Signal<Item = Option<(f64, f64)>>
) -> impl Signal<Item = f64> {
    aabb_bounds_px(coords_in_center, transform_signal, size_signal).map(|bounds| bounds.x)
}

pub fn y_px(
    coords_in_center: bool, 
    transform_signal: impl Signal<Item = Transform>, 
    size_signal: impl Signal<Item = Option<(f64, f64)>>
) -> impl Signal<Item = f64> {
    aabb_bounds_px(coords_in_center, transform_signal, size_signal).map(|bounds| bounds.y)
}

pub fn width_px(
    coords_in_center: bool, 
    transform_signal: impl Signal<Item = Transform>, 
    size_signal: impl Signal<Item = Option<(f64, f64)>>
) -> impl Signal<Item = f64> {
    aabb_bounds_px(coords_in_center, transform_signal, size_signal).map(|bounds| bounds.width)
}


pub fn height_px(
    coords_in_center: bool, 
    transform_signal: impl Signal<Item = Transform>, 
    size_signal: impl Signal<Item = Option<(f64, f64)>>
) -> impl Signal<Item = f64> {
    aabb_bounds_px(coords_in_center, transform_signal, size_signal).map(|bounds| bounds.height)
}



pub fn denormalize_matrix_string(transform_signal: impl Signal<Item = Transform>) -> impl Signal<Item = String> {
    map_ref! {
        let resize_info = resize_info_signal(),
        let transform = transform_signal 
        => {
            transform
                .map(|t| t.denormalize_matrix_string(resize_info))
        }
    }
}


//CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
//but set the rotation and translation to identity
pub fn scale_matrix_string(transform_signal: impl Signal<Item = Transform>) -> impl Signal<Item = String> {
    transform_signal
        .map(|t| t.scale_matrix_string())
}
//CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
//but set the scale and translation to identity
pub fn rotation_matrix_string(transform_signal: impl Signal<Item = Transform>) -> impl Signal<Item = String> {
    transform_signal
        .map(|t| t.rotation_matrix_string())
}
pub fn invert_rotation_matrix_string(transform_signal: impl Signal<Item = Transform>) -> impl Signal<Item = String> {
    transform_signal
        .map(|t| t.invert_rotation_matrix_string())
}
