use std::ops::{Mul, Index, IndexMut};

// 4x4 column-major matrix
#[derive(Debug, Clone, PartialEq)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub fn identity() -> Mat4 {
        return Mat4 ([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn zero() -> Mat4 {
        return Mat4 ([[0.0; 4]; 4]);
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        let mut m = Mat4::identity();
        m.0[3][0] = x;
        m.0[3][1] = y;
        m.0[3][2] = z;
        return m;
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        let mut m = Mat4::identity();
        m.0[0][0] = x;
        m.0[1][1] = y;
        m.0[2][2] = z;
        return m;
    }

    pub fn rotation_x(angle: f32) -> Mat4 {
        let (s, c) = angle.sin_cos();
        return Mat4 ([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c,   s,   0.0],
            [0.0, -s,  c,   0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn rotation_y(angle: f32) -> Mat4 {
        let (s, c) = angle.sin_cos();
        return Mat4 ([
            [ c,  0.0, -s,  0.0],
            [0.0, 1.0, 0.0, 0.0],
            [ s,  0.0,  c,  0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn rotation_z(angle: f32) -> Mat4 {
        let (s, c) = angle.sin_cos();
        return Mat4 ([
            [ c,  s,  0.0, 0.0],
            [-s,  c,  0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn rotation_axis(axis: (f32, f32, f32), angle: f32) -> Mat4 {
        let (x, y, z) = axis;
        let len = (x*x + y*y + z*z).sqrt();
        if len == 0.0 {
            return Mat4::identity();
        }
        let (x, y, z) = (x / len, y / len, z / len);
        let (s, c) = angle.sin_cos();
        let ic = 1.0 - c;

        return Mat4 ([
            [x*x*ic + c,     y*x*ic + z*s, z*x*ic - y*s, 0.0],
            [x*y*ic - z*s,   y*y*ic + c,   z*y*ic + x*s, 0.0],
            [x*z*ic + y*s,   y*z*ic - x*s, z*z*ic + c,   0.0],
            [0.0,            0.0,          0.0,          1.0],
        ]);
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        return Mat4 ([
            [fov / aspect, 0.0, 0.0, 0.0],
            [0.0, fov, 0.0, 0.0],
            [0.0, 0.0, (far + near)/(far - near), 1.0],
            [0.0, 0.0, -(2.0 * far * near)/(far - near), 0.0],
        ]);
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        let rl = right - left;
        let tb = top - bottom;
        let fn_ = far - near;

        return Mat4 ([
            [2.0 / rl, 0.0, 0.0, 0.0],
            [0.0, 2.0 / tb, 0.0, 0.0],
            [0.0, 0.0, -2.0 / fn_, 0.0],
            [-(right + left) / rl, -(top + bottom) / tb, -(far + near) / fn_, 1.0],
        ]);
    }

    pub fn mul_mat4(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.0[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        return result;
    }

    pub fn mul_vec4(&self, v: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
        let mut out: [f32; 4] = [0.0; 4];
        for i in 0..4 {
            out[i] = self.0[0][i] * v.0
                   + self.0[1][i] * v.1
                   + self.0[2][i] * v.2
                   + self.0[3][i] * v.3;
        }
        return (out[0], out[1], out[2], out[3]);
    }

    pub fn mul_vec4_as_slice(&self, v: [f32; 4]) -> [f32; 4] {
        let mut out: [f32; 4] = [0.0; 4];
        for i in 0..4 {
            out[i] = self.0[0][i] * v[0]
                   + self.0[1][i] * v[1]
                   + self.0[2][i] * v[2]
                   + self.0[3][i] * v[3];
        }
        return out;
    }

    pub fn mul_vec3_as_slice(&self, v: [f32; 3]) -> [f32; 3] {
        let out = self.mul_vec4_as_slice([v[0], v[1], v[2], 1.0]);
        return [out[0], out[1], out[2]];
    }

    pub fn transpose(&self) -> Mat4 {
        let mut m = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                m.0[i][j] = self.0[j][i];
            }
        }
        return m;
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        self.mul_mat4(&rhs)
    }
}

impl Index<usize> for Mat4 {
    type Output = [f32; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
