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
            y_col: Vec4::new(0.0, cos, -sin, 0.0),
            z_col: Vec4::new(0.0, sin, cos, 0.0),
            w_col: Vec4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}
