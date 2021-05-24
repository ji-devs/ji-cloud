use futures_signals::{
    map_ref,
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform, TraceShape as RawTraceShape};
use crate::transform::state::TransformState;
use dominator::clone;
use utils::{math::BoundsF64, prelude::*};

#[derive(Clone)]
pub struct Trace {
    pub transform: Rc<TransformState>,
    pub shape: Mutable<TraceShape>,
}

#[derive(Clone)]
pub enum TraceShape {
    /// width and height
    Rect(f64, f64),
    /// radius 
    Ellipse(f64, f64),
    /// points - all rendered at once so no benefit to MutableVec
    Path(Mutable<Vec<(f64, f64)>>)
}


impl Trace {
    pub fn new(raw: Option<RawTrace>, on_change_cb: Rc<Box<dyn Fn()>>) -> Self {
        let raw = match raw {
            Some(raw) => raw,
            None => {
                RawTrace {
                    transform: Transform::identity(),
                    shape: RawTraceShape::Path(Vec::new()) 
                }
            }
        };

        Self {
            transform: Rc::new(TransformState::new(
                raw.transform,
                None, 
                false,
                Some(move |_| {
                    on_change_cb();
                })
            )),
            shape: Mutable::new(raw.shape.into()) 
        }
    }

    pub fn to_raw(&self) -> RawTrace {
        RawTrace {
            transform: self.transform.get_inner_clone(),
            shape: self.shape.get_cloned().into()
        }
    }

    pub fn calc_bounds(&self, add_transform: bool) -> Option<BoundsF64> {
        let mut bounds = match &*self.shape.lock_ref() {
            TraceShape::Path(path) => {
                //Set to inverse of max values
                let mut left:f64 = 1.0;
                let mut right:f64 = 0.0;
                let mut top:f64 = 1.0;
                let mut bottom:f64 = 0.0;
                for (x, y) in path.lock_ref().iter() {
                    let x = *x;
                    let y = *y;
                    if x < left {
                        left = x;
                    }

                    if x > right {
                        right = x;
                    }

                    if y < top {
                        top = y;
                    }

                    if y > bottom {
                        bottom = y;
                    }
                }

                let width = right - left;
                let height = bottom - top;



                if width > 0.0 && height > 0.0 {
                    Some(BoundsF64 {
                        x: left,
                        y: top,
                        width,
                        height,
                        invert_y: true 
                    })
                } else {
                    None
                }
            },

            TraceShape::Ellipse(radius_x, radius_y) => {
                Some(BoundsF64 {
                    x: 0.0,
                    y: 0.0,
                    width: radius_x * 2.0,
                    height: radius_y * 2.0,
                    invert_y: true
                })
            },
            TraceShape::Rect(width, height) => {
                Some(BoundsF64 {
                    x: 0.0,
                    y: 0.0,
                    width: *width,
                    height: *height,
                    invert_y: true
                })
            }
        };

        if add_transform {
            if let Some(bounds) = bounds.as_mut() {
                let transform = self.transform.get_inner_clone();
                let (tx, ty) = transform.get_translation_2d();
                bounds.x += tx;
                bounds.y += ty;
            }
        }

        bounds

    }
}

impl From<RawTraceShape> for TraceShape {
    fn from(raw: RawTraceShape) -> Self {
        match raw {
            RawTraceShape::Rect(width, height) => Self::Rect(width, height),
            RawTraceShape::Ellipse(radius_x, radius_y) => Self::Ellipse(radius_x, radius_y),
            RawTraceShape::Path(path) => Self::Path(Mutable::new(path))
        }
    }
}

impl From<TraceShape> for RawTraceShape {
    fn from(trace: TraceShape) -> Self {
        match trace {
            TraceShape::Rect(width, height) => RawTraceShape::Rect(width, height),
            TraceShape::Ellipse(radius_x, radius_y) => RawTraceShape::Ellipse(radius_x, radius_y),
            TraceShape::Path(path) => RawTraceShape::Path(path.lock_ref().to_vec()),
        }
    }
}
impl TraceShape {

    pub fn new_path(path:Vec<(f64, f64)>) -> Self {
        Self::Path(Mutable::new(path))
    }
    /*
    pub fn has_shape_signal(&self) -> impl Signal<Item = bool> {
        self.shape.signal_ref(|shape| {
            match shape {
                TraceShape::Path(path) => {
                    path.len() > 2
                },
                _ => false
            }
        })
    }
    */

}
