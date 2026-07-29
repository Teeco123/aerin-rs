use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const NEG_ONE: Self = Self {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    };

    pub const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub const NEG_X: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const NEG_Y: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };

    pub const NEG_Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    pub const MIN: Self = Self {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };

    pub const MAX: Self = Self {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };

    pub const NAN: Self = Self {
        x: f32::NAN,
        y: f32::NAN,
        z: f32::NAN,
    };

    pub const INFINITY: Self = Self {
        x: f32::INFINITY,
        y: f32::INFINITY,
        z: f32::INFINITY,
    };

    pub const NEG_INFINITY: Self = Self {
        x: f32::NEG_INFINITY,
        y: f32::NEG_INFINITY,
        z: f32::NEG_INFINITY,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x: x, y: y, z: z }
    }

    #[inline]
    pub const fn add(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    #[inline]
    pub const fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }

    #[inline]
    pub const fn sub(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    #[inline]
    pub const fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }

    #[inline]
    pub const fn mul(self, rhs: f32) -> Vec3 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    #[inline]
    pub const fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }

    #[inline]
    pub const fn div(self, rhs: f32) -> Vec3 {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

    #[inline]
    pub const fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }

    #[inline]
    pub const fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.y == rhs.y
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        self.add(rhs)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(rhs);
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        self.sub(rhs)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(rhs);
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        self.mul(rhs)
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(rhs);
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        self.div(rhs)
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(rhs);
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.eq(rhs)
    }
}
