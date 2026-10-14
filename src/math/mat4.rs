use std::ops::Mul;

use super::vec4::Vec4;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Mat4 {
    m: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        let mut result = Self::default();

        for i in 0..4 {
            result.m[i][i] = 1.0;
        }

        result
    }

    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.m[row][column]
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut result = Self::identity();

        result.m[0][3] = x;
        result.m[1][3] = y;
        result.m[2][3] = z;

        result
    }

    pub fn rotation_x(radians: f32) -> Self {
        let mut result = Self::identity();

        let c = radians.cos();
        let s = radians.sin();

        result.m[1][1] = c;
        result.m[1][2] = -s;
        result.m[2][1] = s;
        result.m[2][2] = c;

        result
    }

    pub fn rotation_y(radians: f32) -> Self {
        let mut result = Self::identity();

        let c = radians.cos();
        let s = radians.sin();

        result.m[0][0] = c;
        result.m[0][2] = s;
        result.m[2][0] = -s;
        result.m[2][2] = c;

        result
    }

    pub fn rotation_z(radians: f32) -> Self {
        let mut result = Self::identity();

        let c = radians.cos();
        let s = radians.sin();

        result.m[0][0] = c;
        result.m[0][1] = -s;
        result.m[1][0] = s;
        result.m[1][1] = c;

        result
    }

    /// Creates a right-handed perspective projection matrix.
    ///
    /// `fov_y` is the vertical field of view in radians.
    /// `aspect` is the viewport width divided by its height.
    /// `near` and `far` specify the near and far clipping planes.
    ///
    /// The resulting normalized device coordinate depth range is `0.0..=1.0`.
    pub fn perspective(
        fov_y: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Self {
        assert!(fov_y > 0.0 && fov_y < std::f32::consts::PI);
        assert!(aspect > 0.0);
        assert!(near > 0.0);
        assert!(far > near);

        let f = 1.0 / (fov_y / 2.0).tan();

        Self {
            m: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, far / (near - far), (far * near) / (near - far)],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, vector: Vec4) -> Vec4 {
        Vec4::new(
            self.m[0][0] * vector.x
                + self.m[0][1] * vector.y
                + self.m[0][2] * vector.z
                + self.m[0][3] * vector.w,

            self.m[1][0] * vector.x
                + self.m[1][1] * vector.y
                + self.m[1][2] * vector.z
                + self.m[1][3] * vector.w,

            self.m[2][0] * vector.x
                + self.m[2][1] * vector.y
                + self.m[2][2] * vector.z
                + self.m[2][3] * vector.w,

            self.m[3][0] * vector.x
                + self.m[3][1] * vector.y
                + self.m[3][2] * vector.z
                + self.m[3][3] * vector.w,
        )
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::default();

        for row in 0..4 {
            for column in 0..4 {
                for i in 0..4 {
                    result.m[row][column] +=
                        self.m[row][i] * other.m[i][column];
                }
            }
        }

        result
    }
}