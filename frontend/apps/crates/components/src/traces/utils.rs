use utils::{math::BoundsF64, prelude::*, resize::ResizeInfo};
use shared::domain::jig::module::body::{Transform, _groups::design::{Trace as RawTrace, TraceShape as RawTraceShape}};

pub trait TraceExt {
    fn to_raw(&self) -> RawTrace;

    fn calc_bounds(&self, add_offset: bool) -> Option<BoundsF64>;

    fn calc_size(&self, resize_info: &ResizeInfo) -> Option<(f64, f64)> {
        self.calc_bounds(false)
            .map(|bounds| {
                resize_info.get_size_full(bounds.width, bounds.height)
            })
    }
}

impl TraceExt for RawTrace {
    fn to_raw(&self) -> RawTrace {
        self.clone()
    }

    fn calc_bounds(&self, add_offset: bool) -> Option<BoundsF64> {
        use crate::traces::utils::{calc_bounds, ShapeRef};

        let offset = if add_offset {
            Some(self.transform.get_translation_2d())
        } else {
            None
        };

        match &self.shape {
            RawTraceShape::Path(path) => {
                calc_bounds(ShapeRef::Path(&path), offset)
            },

            RawTraceShape::Ellipse(radius_x, radius_y) => {
                calc_bounds(ShapeRef::Ellipse(*radius_x, *radius_y), offset)
            },
            RawTraceShape::Rect(width, height) => {
                calc_bounds(ShapeRef::Rect(*width, *height), offset)
            }
        }

    }
}
pub enum ShapeRef<'a>
{
    Path(&'a[(f64, f64)]),
    Ellipse(f64, f64),
    Rect(f64,f64)
}

//Gets the bounds of the shape itself, prior to any scaling or rotation
//if offset is supplied, then it is added
//TODO - document the use-cases for where offset is used
pub fn calc_bounds<'a>(shape: ShapeRef<'a>, offset: Option<(f64, f64)>) -> Option<BoundsF64> {
    let mut bounds = match shape {
            ShapeRef::Path(path) => {
                //Set to inverse of max values
                let mut left:f64 = 1.0;
                let mut right:f64 = 0.0;
                let mut top:f64 = 1.0;
                let mut bottom:f64 = 0.0;
                for (x, y) in path.iter() {
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

            ShapeRef::Ellipse(radius_x, radius_y) => {
                Some(BoundsF64 {
                    x: 0.0,
                    y: 0.0,
                    width: radius_x * 2.0,
                    height: radius_y * 2.0,
                    invert_y: true
                })
            },
            ShapeRef::Rect(width, height) => {
                Some(BoundsF64 {
                    x: 0.0,
                    y: 0.0,
                    width,
                    height,
                    invert_y: true
                })
            }
        };

    match (offset, bounds.as_mut()) {
        (Some((tx, ty)), Some(bounds)) => {
            bounds.x += tx;
            bounds.y += ty;
        },
        _ => {}
    };


    bounds
}
