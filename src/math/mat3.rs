use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::math::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Mat3 {
    pub n: [Vec3; 3],
}

impl Mat3 {
    pub fn new(
        n00: f32,
        n01: f32,
        n02: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n20: f32,
        n21: f32,
        n22: f32,
    ) -> Mat3 {
        Self {
            n: [
                Vec3::new(n00, n10, n20),
                Vec3::new(n01, n11, n21),
                Vec3::new(n02, n12, n22),
            ],
        }
    }

    pub fn zero() -> Mat3 {
        Self {
            n: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ],
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = Vec3;
    fn index(&self, i: usize) -> &Vec3 {
        &self.n[i]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, i: usize) -> &mut Vec3 {
        &mut self.n[i]
    }
}

impl Index<(usize, usize)> for Mat3 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        let col = &self.n[j];
        match i {
            0 => &col.x,
            1 => &col.y,
            2 => &col.z,
            _ => panic!("Row index out of bounds! Must be 0, 1, or 2."),
        }
    }
}

impl IndexMut<(usize, usize)> for Mat3 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        let col = &mut self.n[j];
        match i {
            0 => &mut col.x,
            1 => &mut col.y,
            2 => &mut col.z,
            _ => panic!("Row index out of bounds! Must be 0, 1, or 2."),
        }
    }
}
