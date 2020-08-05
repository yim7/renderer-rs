use crate::vector::Vector;
use crate::vertex::Vertex;
use sdl2::pixels::Color;

pub trait Interpolate {
    fn interpolate(&self, other: &Self, factor: f32) -> Self;
}

impl Interpolate for Vector {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let x = self.x.interpolate(&other.x, factor);
        let y = self.y.interpolate(&other.y, factor);
        let z = self.z.interpolate(&other.z, factor);
        Self { x, y, z }
    }
}

impl Interpolate for Vertex {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let position = self.position.interpolate(&other.position, factor);
        let normal = self.normal.interpolate(&other.normal, factor);
        let u = self.u.interpolate(&other.u, factor);
        let v = self.v.interpolate(&other.v, factor);
        let color = self.color.interpolate(&other.color, factor);
        Vertex {
            position,
            normal,
            u,
            v,
            color,
        }
    }
}

impl Interpolate for Color {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let r = self.r.interpolate(&other.r, factor);
        let g = self.g.interpolate(&other.g, factor);
        let b = self.b.interpolate(&other.b, factor);
        let a = self.a.interpolate(&other.a, factor);
        Color::RGBA(r, g, b, a)
    }
}

impl Interpolate for f32 {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        self + (other - self) * factor
    }
}

impl Interpolate for u8 {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let x = *self as f32 + (other - self) as f32 * factor;
        x as u8
    }
}
