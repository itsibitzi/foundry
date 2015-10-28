use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x: x, y: y}
    }

    pub fn to_vector2(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {x: self.x * rhs.x, y: self.y * rhs.y}
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Point {
        Point {x: self.x / rhs.x, y: self.y / rhs.y}
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        Point {x: self.x / rhs, y: self.y / rhs}
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {x: -self.x, y: -self.y}
    }
}
