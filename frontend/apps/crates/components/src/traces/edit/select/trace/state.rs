use futures_signals::signal::{Signal, SignalExt, Mutable};

use shared::domain::jig::module::body::{
    Transform,
    _groups::design::{Trace as RawTrace, TraceShape},
};
use std::cell::RefCell;
use std::rc::Rc;

use super::super::select_box::*;
use crate::traces::utils::TraceExt;
use utils::{math::BoundsF64, prelude::*, resize::ResizeInfo};
use web_sys::SvgElement;

pub struct EditSelectTrace {
    pub transform: Transform,
    pub shape: TraceShape,
    pub size: (f64, f64),
    pub select_box: Rc<SelectBox>,
    pub elem: RefCell<Option<SvgElement>>,
}

impl EditSelectTrace {
    pub fn new(raw: RawTrace, resize_info: &ResizeInfo) -> Self {
        let mut _self = Self {
            transform: raw.transform.clone(),
            shape: raw.shape,
            size: (0.0, 0.0),
            select_box: Rc::new(SelectBox::new(raw.transform)),
            elem: RefCell::new(None),
        };

        if let Some(bounds) = _self.calc_bounds(true) {
            _self.size = resize_info.get_size_full(bounds.width, bounds.height);
        }

        _self
    }


}

impl TraceExt for EditSelectTrace {
    fn to_raw(&self) -> RawTrace {
        RawTrace {
            transform: self.transform.clone(),
            shape: self.shape.clone(),
        }
    }

    fn calc_bounds(&self, add_offset: bool) -> Option<BoundsF64> {
        use crate::traces::utils::{calc_bounds, ShapeRef};

        let offset = if add_offset {
            Some(self.transform.get_translation_2d())
        } else {
            None
        };

        match &self.shape {
            TraceShape::Path(path) => calc_bounds(ShapeRef::Path(&path), offset),

            TraceShape::Ellipse(radius_x, radius_y) => {
                calc_bounds(ShapeRef::Ellipse(*radius_x, *radius_y), offset)
            }
            TraceShape::Rect(width, height) => calc_bounds(ShapeRef::Rect(*width, *height), offset),
        }
    }
}
