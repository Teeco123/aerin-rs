use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Self { x: x, y: y }
    }

    pub fn zero() -> Vec2 {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        (self.x * other.x) - (self.y * other.y)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn normalize(&self) -> Vec2 {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
        }
    }

    pub fn distance_to(&self, other: &Vec2) -> f32 {
        (((self.x - other.x) * (self.x - other.x)) + ((self.y - other.y) * (self.y - other.y)))
            .sqrt()
    }

    pub fn direction_to(&self, other: &Vec2) -> Vec2 {
        (*other - *self).normalize()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f32) -> Vec2 {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
