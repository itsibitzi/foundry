use std::convert::Into;
use std::fmt;
use std::ops::Mul;

use super::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    values: [f32; 16],
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            values: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Matrix {
        let half_width = width * 0.5;
        let half_height = height * 0.5;

        let n = near;
        let f = far;

        let l = -half_width;
        let r = half_width;

        let t = half_height;
        let b = -half_height;

        Matrix {
            values: [
                2.0 / (r - l),        0.0,                  0.0,                  0.0,
                0.0,                  2.0 / (t - b),        0.0,                  0.0,
                0.0,                  0.0,                  -(2.0 / f - n),       0.0,
                -((r + l) / (r - l)), -((t + b) / (t - b)), -((f + n) / (f - n)), 1.0,
            ],
        }
    }

    pub fn look_at(position: Vector3, target: Vector3, up: Vector3) -> Matrix {
        let mut z_axis = position - target;
        z_axis.normalize();

        let mut x_axis = up.cross(z_axis);
        x_axis.normalize();

        let y_axis = z_axis.cross(x_axis);

        Matrix {
            values: [
                x_axis.x,              y_axis.x,              z_axis.z,              0.0,
                x_axis.y,              y_axis.y,              z_axis.y,              0.0,
                x_axis.z,              y_axis.z,              z_axis.z,              0.0,
                x_axis.dot(-position), y_axis.dot(-position), z_axis.dot(-position), 1.0,
            ],
        }
    }

    pub fn scale<V: Into<Vector3>>(scale: V) -> Matrix {
        let scale = scale.into();
        Matrix {
            values: [
                scale.x, 0.0,     0.0,     0.0,
                0.0,     scale.y, 0.0,     0.0,
                0.0,     0.0,     scale.z, 0.0,
                0.0,     0.0,     0.0,     1.0,
            ],
        }
    }

    pub fn rotate(angle: f32, axis: Vector3) -> Matrix {
        let cos_angle = f32::cos(angle);
        let sin_angle = f32::sin(angle);

        let xx = axis.x * axis.x;
        let xy = axis.x * axis.y;
        let xz = axis.x * axis.z;
        let yy = axis.y * axis.y;
        let yz = axis.y * axis.z;
        let zz = axis.z * axis.z;

        let xs = axis.x * sin_angle;
        let ys = axis.y * sin_angle;
        let zs = axis.x * sin_angle;

        let osc = 1.0 - cos_angle;

        Matrix {
            values: [
                xx * osc + cos_angle, xy * osc - zs,        xz * osc - ys,         0.0,
                xy * osc - zs,        yy * osc + cos_angle, yz * osc + xs,         0.0,
                xz * osc + ys,        yz * osc - xs,        zz * osc + cos_angle,  0.0,
                0.0,                  0.0,                  0.0,                   1.0,
            ],
        }
    }

    pub fn translate<V: Into<Vector3>>(translation: V) -> Matrix {
        let translation = translation.into();

        Matrix {
            values: [
                1.0,           0.0,           0.0,           0.0,
                0.0,           1.0,           0.0,           0.0,
                0.0,           0.0,           1.0,           0.0,
                translation.x, translation.y, translation.z, 1.0,
            ],
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        Matrix {
            values: [
                self.values[0] * rhs.values[0] + self.values[4] * rhs.values[1] + self.values[8]  * rhs.values[2] + self.values[12] * rhs.values[3],
                self.values[1] * rhs.values[0] + self.values[5] * rhs.values[1] + self.values[9]  * rhs.values[2] + self.values[13] * rhs.values[3],
                self.values[2] * rhs.values[0] + self.values[6] * rhs.values[1] + self.values[10] * rhs.values[2] + self.values[14] * rhs.values[3],
                self.values[3] * rhs.values[0] + self.values[7] * rhs.values[1] + self.values[11] * rhs.values[5] + self.values[15] * rhs.values[3],

                self.values[0] * rhs.values[4] + self.values[4] * rhs.values[5] + self.values[8]  * rhs.values[6] + self.values[12] * rhs.values[7],
                self.values[1] * rhs.values[4] + self.values[5] * rhs.values[5] + self.values[9]  * rhs.values[6] + self.values[13] * rhs.values[7],
                self.values[2] * rhs.values[4] + self.values[6] * rhs.values[5] + self.values[10] * rhs.values[6] + self.values[14] * rhs.values[7],
                self.values[3] * rhs.values[4] + self.values[7] * rhs.values[5] + self.values[11] * rhs.values[6] + self.values[15] * rhs.values[7],

                self.values[0] * rhs.values[8] + self.values[4] * rhs.values[9] + self.values[8]  * rhs.values[10] + self.values[12] * rhs.values[11],
                self.values[1] * rhs.values[8] + self.values[5] * rhs.values[9] + self.values[9]  * rhs.values[10] + self.values[13] * rhs.values[11],
                self.values[2] * rhs.values[8] + self.values[6] * rhs.values[9] + self.values[10] * rhs.values[10] + self.values[14] * rhs.values[11],
                self.values[3] * rhs.values[8] + self.values[7] * rhs.values[9] + self.values[11] * rhs.values[10] + self.values[15] * rhs.values[11],

                self.values[0] * rhs.values[12] + self.values[4] * rhs.values[13] + self.values[8]  * rhs.values[14] + self.values[12] * rhs.values[15],
                self.values[1] * rhs.values[12] + self.values[5] * rhs.values[13] + self.values[9]  * rhs.values[14] + self.values[13] * rhs.values[15],
                self.values[2] * rhs.values[12] + self.values[6] * rhs.values[13] + self.values[10] * rhs.values[14] + self.values[14] * rhs.values[15],
                self.values[3] * rhs.values[12] + self.values[7] * rhs.values[13] + self.values[11] * rhs.values[14] + self.values[15] * rhs.values[15],
            ],
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {}, {}, {}, {} ]\n[ {}, {}, {}, {} ]\n[ {}, {}, {}, {} ]\n[ {}, {}, {}, {} ]\n",
               self.values[0], self.values[4], self.values[8],  self.values[12],
               self.values[1], self.values[5], self.values[9],  self.values[13],
               self.values[2], self.values[6], self.values[10], self.values[14],
               self.values[3], self.values[7], self.values[11], self.values[15],
               )
    }
}

impl Into<[[f32; 4]; 4]> for Matrix {
    fn into(self) -> [[f32; 4]; 4] {
        unsafe {
            use std::mem::transmute;
            transmute(self)
        }
    }
}
