use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn normalize(&self) -> Vec3 {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
        }
    }

    pub fn distance_to(&self, other: &Vec3) -> f32 {
        (((self.x - other.x) * (self.x - other.x))
            + ((self.y - other.y) * (self.y - other.y))
            + ((self.z - other.z) * (self.z - other.z)))
            .sqrt()
    }

    pub fn direction_to(&self, other: &Vec3) -> Vec3 {
        (*other - *self).normalize()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.y == other.y
    }
}
