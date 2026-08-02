use crate::math::{vec3::Vec3, vec4::Vec4};

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub x_col: Vec4,
    pub y_col: Vec4,
    pub z_col: Vec4,
    pub w_col: Vec4,
}

impl Mat4 {
    pub const ZERO: Self = Self::new(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);

    pub const IDENTITY: Self = Self::new(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    #[inline]
    pub const fn new(x_col: Vec4, y_col: Vec4, z_col: Vec4, w_col: Vec4) -> Mat4 {
        Self {
            x_col,
            y_col,
            z_col,
            w_col,
        }
    }

    #[inline]
    pub const fn to_array(&self) -> [f32; 16] {
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

    #[inline]
    pub fn rotate_x(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_col: Vec4::X,
            y_col: Vec4::new(0.0, cos, sin, 0.0),
            z_col: Vec4::new(0.0, -sin, cos, 0.0),
            w_col: Vec4::W,
        }
    }

    #[inline]
    pub fn rotate_y(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_col: Vec4::new(cos, 0.0, -sin, 0.0),
            y_col: Vec4::Y,
            z_col: Vec4::new(sin, 0.0, cos, 0.0),
            w_col: Vec4::W,
        }
    }

    #[inline]
    pub fn rotate_z(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x_col: Vec4::new(cos, sin, 0.0, 0.0),
            y_col: Vec4::new(-sin, cos, 0.0, 0.0),
            z_col: Vec4::Z,
            w_col: Vec4::W,
        }
    }

    #[inline]
    pub const fn translate(position: Vec3) -> Self {
        Mat4::new(
            Vec4::X,
            Vec4::Y,
            Vec4::Z,
            Vec4::new(position.x, position.y, position.z, 1.0),
        )
    }

    #[inline]
    pub const fn scale(scale: Vec3) -> Self {
        Mat4::new(
            Vec4::new(scale.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.z, 0.0),
            Vec4::W,
        )
    }

    #[inline]
    pub fn projection(fov: f32, width: f32, height: f32, near: f32, far: f32) -> Self {
        let aspect_ratio = width / height;
        let focal_length = 1.0 / (fov.to_radians() / 2.0).tan();

        let m00 = focal_length / aspect_ratio;
        let m22 = (far + near) / (near - far);
        let m32 = (2.0 * near * far) / (near - far);

        Mat4::new(
            Vec4::new(m00, 0.0, 0.0, 0.0),
            Vec4::new(0.0, focal_length, 0.0, 0.0),
            Vec4::new(0.0, 0.0, m22, -1.0),
            Vec4::new(0.0, 0.0, m32, 0.0),
        )
    }
}
