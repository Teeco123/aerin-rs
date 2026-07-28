use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn add(self, rhs: Vec4) -> Vec4 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }

    #[inline]
    pub fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }

    #[inline]
    pub fn sub(self, rhs: Vec4) -> Vec4 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }

    #[inline]
    pub fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }

    #[inline]
    pub fn mul(self, rhs: f32) -> Vec4 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }

    #[inline]
    pub fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }

    #[inline]
    pub fn div(self, rhs: f32) -> Vec4 {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }

    #[inline]
    pub fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }

    #[inline]
    pub fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z && self.w == rhs.w
    }
}

impl Add for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Vec4 {
        self.add(rhs)
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(rhs);
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Vec4 {
        self.sub(rhs)
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(rhs);
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: f32) -> Vec4 {
        self.mul(rhs)
    }
}

impl MulAssign for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(rhs);
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: f32) -> Vec4 {
        self.div(rhs)
    }
}

impl DivAssign for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(rhs);
    }
}

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.eq(rhs)
    }
}
