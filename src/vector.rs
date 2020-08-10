use std::ops::Sub;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z, w: 1.0 }
    }

    pub fn length(&self) -> f32 {
        let Self { x, y, z, .. } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let Self { x, y, z, .. } = self;
        let length = self.length();
        let factor = if length > 0.0 { 1.0 / length } else { 0.0 };
        Self {
            x: x * factor,
            y: y * factor,
            z: z * factor,
            w: 1.0,
        }
    }

    pub fn dot(&self, v: &Vector) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vector) -> Self {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.x * v.z;
        let z = self.x * v.y - self.y * v.x;
        let w = 1.0;
        Self { x, y, z, w }
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
