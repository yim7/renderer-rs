use std::ops::Sub;

#[derive(Debug, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    pub fn length(&self) -> f32 {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let Self { x, y, z } = self;
        let length = self.length();
        let factor = if length > 0.0 { 1.0 / length } else { 0.0 };
        Self {
            x: x * factor,
            y: y * factor,
            z: z * factor,
        }
    }

    pub fn dot(&self, v: &Vector) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vector) -> Self {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.x * v.z;
        let z = self.x * v.y - self.y * v.x;
        Self { x, y, z }
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
