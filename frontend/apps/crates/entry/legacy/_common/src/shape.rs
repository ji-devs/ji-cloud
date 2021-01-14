use serde::{Serialize, Deserialize};
use serde_repr::*;

pub use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PathPoint {
    #[serde(rename="type")]
    pub kind: PathElementKind,

    pub x: f64,
    pub y: f64,

    #[serde(default)]
    pub cp1x: f64,
    #[serde(default)]
    pub cp1y: f64,
    #[serde(default)]
    pub cp2x: f64,
    #[serde(default)]
    pub cp2y: f64,
}
#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum PathElementKind {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubPath
}
