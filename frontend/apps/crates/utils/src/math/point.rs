#[derive(Debug, Clone, Copy)]
pub struct PointI32 {
    pub x: i32,
    pub y: i32,
}

impl PointI32 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

//TODO - simplify these with macros
impl std::ops::Sub for PointI32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign for PointI32 {
    fn sub_assign(&mut self, other: Self) {
        *self = PointI32 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Add for PointI32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for PointI32 {
    fn add_assign(&mut self, other: Self) {
        *self = PointI32 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// f64
#[derive(Debug, Clone, Copy)]
pub struct PointF64 {
    pub x: f64,
    pub y: f64,
}

impl PointF64 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for PointF64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign for PointF64 {
    fn sub_assign(&mut self, other: Self) {
        *self = PointF64 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Add for PointF64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for PointF64 {
    fn add_assign(&mut self, other: Self) {
        *self = PointF64 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
