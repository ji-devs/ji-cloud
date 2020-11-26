use super::*;
use web_sys::{Element, DomRect};

#[derive(Debug, Clone, Copy)]
pub struct BoundsF64 {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64, 
    pub invert_y: bool,
}

impl BoundsF64 {
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
