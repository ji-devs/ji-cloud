#[derive(Debug, Clone, Copy)]
pub struct RectF64 {
    pub width: f64,
    pub height: f64, 
}

impl RectF64 {
    pub fn new(width: f64, height: f64) -> Self {
        Self {width, height }
    }
}

impl std::ops::Div for RectF64 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}
