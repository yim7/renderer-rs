use crate::vector::Vector;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Matrix {
    m: [f32; 16],
}

impl Matrix {
    pub fn new(values: [f32; 16]) -> Self {
        Matrix { m: values }
    }

    pub fn zero() -> Self {
        Self { m: [0.0; 16] }
    }

    pub fn look_at_lh(eye: &Vector, target: &Vector, up: &Vector) -> Self {
        let axis_z = (target - eye).normalize();
        let axis_x = up.cross(&axis_z).normalize();
        let axis_y = axis_z.cross(&axis_x).normalize();

        let eye_x = -axis_x.dot(eye);
        let eye_y = -axis_y.dot(eye);
        let eye_z = -axis_z.dot(eye);

        #[rustfmt::skip]
        let values = [
            axis_x.x, axis_y.x, axis_z.x, 0.0,
            axis_x.y, axis_y.y, axis_z.y, 0.0,
            axis_x.z, axis_y.z, axis_z.z, 0.0,
            eye_x,    eye_y,    eye_z,    1.0,
        ];

        Matrix::new(values)
    }

    pub fn perspective_fov_lh(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let height = 1.0 / (fov / 2.0).tan();
        let width = height / aspect;
        #[rustfmt::skip]
        let values = [
            width,    0.0,      0.0,                                0.0,
            0.0,      height,   0.0,                                0.0,
            0.0,      0.0,      zfar / (zfar - znear),              1.0,
            0.0,      0.0,      (znear * zfar) / (znear - zfar),    0.0,
        ];
        Matrix::new(values)
    }

    pub fn rotation_x(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        #[rustfmt::skip]
        let values = [
            1.0, 0.0, 0.0, 0.0,
            0.0, c,   s,   0.0,
            0.0, -s,  c,   0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        Matrix::new(values)
    }

    pub fn rotation_y(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        #[rustfmt::skip]
        let values = [
            c, 0.0, -s, 0.0,
            0.0, 1.0, 0.0,  0.0,
            s, 0.0, c,  0.0,
            0.0, 0.0, 0.0,  1.0,
        ];
        Matrix::new(values)
    }

    pub fn rotation_z(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        #[rustfmt::skip]
        let values = [
            c,  s, 0.0, 0.0,
            -s, c, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0,
        ];
        Matrix::new(values)
    }

    pub fn rotation(angle: &Vector) -> Self {
        let x = Matrix::rotation_x(angle.x);
        let y = Matrix::rotation_y(angle.y);
        let z = Matrix::rotation_y(angle.z);
        x * y * z
    }

    pub fn translation(v: &Vector) -> Self {
        let Vector { x, y, z, .. } = *v;
        #[rustfmt::skip]
        let values = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, z, 1.0,
        ];
        Matrix::new(values)
    }

    pub fn transform(&self, v: &Vector) -> Vector {
        let m = self.m;
        let x = v.x * m[0 * 4 + 0] + v.y * m[1 * 4 + 0] + v.z * m[2 * 4 + 0] + m[3 * 4 + 0];
        let y = v.x * m[0 * 4 + 1] + v.y * m[1 * 4 + 1] + v.z * m[2 * 4 + 1] + m[3 * 4 + 1];
        let z = v.x * m[0 * 4 + 2] + v.y * m[1 * 4 + 2] + v.z * m[2 * 4 + 2] + m[3 * 4 + 2];
        let w = v.x * m[0 * 4 + 3] + v.y * m[1 * 4 + 3] + v.z * m[2 * 4 + 3] + m[3 * 4 + 3];
        Vector {
            x: x / w,
            y: y / w,
            z: z / w,
            w: 1.0 / w,
        }
    }

    pub fn transform_vector(&self, v: &Vector) -> Vector {
        let m = self.m;
        let x = v.x * m[0 * 4 + 0] + v.y * m[1 * 4 + 0] + v.z * m[2 * 4 + 0] + m[3 * 4 + 0];
        let y = v.x * m[0 * 4 + 1] + v.y * m[1 * 4 + 1] + v.z * m[2 * 4 + 1] + m[3 * 4 + 1];
        let z = v.x * m[0 * 4 + 2] + v.y * m[1 * 4 + 2] + v.z * m[2 * 4 + 2] + m[3 * 4 + 2];
        Vector { x, y, z, w: 0.0 }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; 16];
        let m1 = self.m;
        let m2 = rhs.m;
        for index in 0..16 {
            let i = index / 4;
            let j = index % 4;
            values[index] = m1[i * 4] * m2[j]
                + m1[i * 4 + 1] * m2[4 + j]
                + m1[i * 4 + 2] * m2[2 * 4 + j]
                + m1[i * 4 + 3] * m2[3 * 4 + j];
        }
        Matrix::new(values)
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
