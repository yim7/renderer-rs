use std::ops::{Add, Mul, Sub};
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    fn interpolate(&self, other: Vector, factor: f32) -> Self {
        let x = self.x + (other.x - self.x);
        let y = self.x + (other.x - self.x);
        let y = self.x + (other.x - self.x);
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
