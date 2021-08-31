use futures_signals::signal::Mutable;

use std::rc::Rc;

use crate::transform::state::{TransformCallbacks, TransformState};
use shared::domain::jig::module::body::{
    Transform,
    Audio,
    _groups::design::{Trace as RawTrace, TraceShape as RawTraceShape, TraceKind},
};

use utils::{math::BoundsF64, prelude::*};

#[derive(Clone)]
pub struct DrawTrace {
    pub transform: Rc<TransformState>,
    pub shape: Mutable<TraceShape>,
    pub kind: TraceKind,
    pub audio: Option<Audio>,
}

impl DrawTrace {
    pub fn new(raw: Option<RawTrace>, default_kind: TraceKind, on_change_cb: Rc<Box<dyn Fn()>>) -> Self {
        let raw = match raw {
            Some(raw) => raw,
            None => RawTrace {
                transform: Transform::identity(),
                shape: RawTraceShape::Path(Vec::new()),
                kind: default_kind,
                audio: None,
            },
        };

        Self {
            transform: Rc::new(TransformState::new(
                raw.transform,
                None,
                false,
                TransformCallbacks::new(
                    Some(move |_| {
                        on_change_cb();
                    }),
                    None::<fn()>,
                ),
            )),
            shape: Mutable::new(raw.shape.into()),
            kind: raw.kind,
            audio: raw.audio
        }
    }
}

impl crate::traces::utils::TraceExt for DrawTrace {
    fn to_raw(&self) -> RawTrace {
        RawTrace {
            transform: self.transform.get_inner_clone(),
            shape: self.shape.get_cloned().into(),
            kind: self.kind,
            audio: self.audio.clone()
        }
    }

    fn calc_bounds(&self, add_offset: bool) -> Option<BoundsF64> {
        use crate::traces::utils::{calc_bounds, ShapeRef};

        let offset = if add_offset {
            Some(self.transform.get_inner_clone().get_translation_2d())
        } else {
            None
        };

        match &*self.shape.lock_ref() {
            TraceShape::Path(path) => calc_bounds(ShapeRef::Path(&path.lock_ref()), offset),

            TraceShape::Ellipse(radius_x, radius_y) => {
                calc_bounds(ShapeRef::Ellipse(*radius_x, *radius_y), offset)
            }
            TraceShape::Rect(width, height) => calc_bounds(ShapeRef::Rect(*width, *height), offset),
        }
    }
}
#[derive(Clone)]
pub enum TraceShape {
    /// width and height
    Rect(f64, f64),
    /// radius
    Ellipse(f64, f64),
    /// points - all rendered at once so no benefit to MutableVec
    Path(Mutable<Vec<(f64, f64)>>),
}

impl From<RawTraceShape> for TraceShape {
    fn from(raw: RawTraceShape) -> Self {
        match raw {
            RawTraceShape::Rect(width, height) => Self::Rect(width, height),
            RawTraceShape::Ellipse(radius_x, radius_y) => Self::Ellipse(radius_x, radius_y),
            RawTraceShape::Path(path) => Self::Path(Mutable::new(path)),
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
    pub fn new_path(path: Vec<(f64, f64)>) -> Self {
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
