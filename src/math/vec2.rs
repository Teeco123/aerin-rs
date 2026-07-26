use std::{
    f32,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const NEG_ONE: Self = Self { x: -1.0, y: -1.0 };

    pub const X: Self = Self { x: 1.0, y: 0.0 };

    pub const Y: Self = Self { x: 0.0, y: 1.0 };

    pub const NEG_X: Self = Self { x: -1.0, y: 0.0 };

    pub const NEG_Y: Self = Self { x: 0.0, y: -1.0 };

    pub const MIN: Self = Self {
        x: f32::MIN,
        y: f32::MIN,
    };

    pub const MAX: Self = Self {
        x: f32::MAX,
        y: f32::MAX,
    };

    pub const NAN: Self = Self {
        x: f32::NAN,
        y: f32::NAN,
    };

    pub const INFINITY: Self = Self {
        x: f32::INFINITY,
        y: f32::INFINITY,
    };

    pub const NEG_INFINITY: Self = Self {
        x: f32::NEG_INFINITY,
        y: f32::NEG_INFINITY,
    };

    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Self { x: x, y: y }
    }

    #[inline]
    pub fn add(self, rhs: Vec2) -> Vec2 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    #[inline]
    pub fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    #[inline]
    pub fn sub(self, rhs: Vec2) -> Vec2 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    #[inline]
    pub fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    #[inline]
    pub fn mul(self, rhs: f32) -> Vec2 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

    #[inline]
    pub fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }

    #[inline]
    pub fn div(self, rhs: f32) -> Vec2 {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }

    #[inline]
    pub fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }

    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        self.add(rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(rhs);
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Vec2) -> Vec2 {
        self.sub(rhs)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(rhs);
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: f32) -> Vec2 {
        self.mul(rhs)
    }
}

impl MulAssign for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(rhs);
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn div(self, rhs: f32) -> Vec2 {
        self.div(rhs)
    }
}

impl DivAssign for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(rhs);
    }
}

impl PartialEq for Vec2 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.eq(rhs)
    }
}
