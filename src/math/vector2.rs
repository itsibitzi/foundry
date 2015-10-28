use std::convert::{From, Into};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{Point, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {x: x, y: y}
    }

    pub fn one() -> Vector2 {
        Vector2 {x: 1.0, y: 1.0}
    }

    pub fn unit_x() -> Vector2 {
        Vector2 {x: 1.0, y: 0.0}
    }

    pub fn unit_y() -> Vector2 {
        Vector2 {x: 0.0, y: 1.0}
    }

    pub fn zero() -> Vector2 {
        Vector2 {x: 0.0, y: 0.0}
    }

    pub fn len_sqrd(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self.x * rhs.x, y: self.y * rhs.y}
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Div for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self.x / rhs.x, y: self.y / rhs.y}
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {x: self.x / rhs, y: self.y / rhs}
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {x: -self.x, y: -self.y}
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl From<Point> for Vector2 {
    fn from(p: Point) -> Vector2 {
        Vector2 {x: p.x as f32, y: p.y as f32}
    }
}

impl Into<Vector3> for Vector2 {
    fn into(self) -> Vector3 {
        Vector3 {x: self.x, y: self.y, z: 0.0}
    }
}
