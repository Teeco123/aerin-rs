use crate::math::vec4::Vec4;

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub x_col: Vec4,
    pub y_col: Vec4,
    pub z_col: Vec4,
    pub w_col: Vec4,
}

impl Mat4 {
    #[inline]
    pub fn new(x_col: Vec4, y_col: Vec4, z_col: Vec4, w_col: Vec4) -> Mat4 {
        Self {
            x_col,
            y_col,
            z_col,
            w_col,
        }
    }

    #[inline]
    pub fn rotate_x(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_col: Vec4::new(1.0, 0.0, 0.0, 0.0),
            y_col: Vec4::new(0.0, cos, sin, 0.0),
            z_col: Vec4::new(0.0, -sin, cos, 0.0),
            w_col: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn rotate_y(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_col: Vec4::new(cos, 0.0, -sin, 0.0),
            y_col: Vec4::new(0.0, 1.0, 0.0, 0.0),
            z_col: Vec4::new(sin, 0.0, cos, 0.0),
            w_col: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn rotate_z(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_col: Vec4::new(cos, sin, 0.0, 0.0),
            y_col: Vec4::new(-sin, cos, 0.0, 0.0),
            z_col: Vec4::new(0.0, 0.0, 1.0, 0.0),
            w_col: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    #[inline]
    pub fn to_array(&self) -> [f32; 16] {
        [
            self.x_col.x,
            self.x_col.y,
            self.x_col.z,
            self.x_col.w,
            self.y_col.x,
            self.y_col.y,
            self.y_col.z,
            self.y_col.w,
            self.z_col.x,
            self.z_col.y,
            self.z_col.z,
            self.z_col.w,
            self.w_col.x,
            self.w_col.y,
            self.w_col.z,
            self.w_col.w,
        ]
    }
}
