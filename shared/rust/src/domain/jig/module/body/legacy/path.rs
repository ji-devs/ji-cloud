use serde::{Deserialize, Serialize};
use serde_repr::*;

pub use super::*;

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathPoint {
    #[allow(missing_docs)]
    #[serde(rename = "type")]
    pub kind: PathElementKind,

    #[allow(missing_docs)]
    pub x: f64,
    #[allow(missing_docs)]
    pub y: f64,

    #[allow(missing_docs)]
    #[serde(default)]
    pub cp1x: f64,

    #[allow(missing_docs)]
    #[serde(default)]
    pub cp1y: f64,

    #[allow(missing_docs)]
    #[serde(default)]
    pub cp2x: f64,

    #[allow(missing_docs)]
    #[serde(default)]
    pub cp2y: f64,
}

#[allow(missing_docs)]
#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum PathElementKind {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubPath,
}
