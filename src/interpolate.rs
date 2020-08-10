use crate::vector::Vector;
use crate::{color::Color, vertex::Vertex};

pub trait Interpolate {
    fn interpolate(&self, other: &Self, factor: f32) -> Self;
}

impl Interpolate for Vector {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let x = self.x.interpolate(&other.x, factor);
        let y = self.y.interpolate(&other.y, factor);
        let z = self.z.interpolate(&other.z, factor);
        let w = self.w.interpolate(&other.w, factor);
        Self { x, y, z, w }
    }
}

impl Interpolate for Vertex {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let Vertex {
            position: p1,
            normal: n1,
            u: u1,
            v: v1,
            color: c1,
            intensity: i1,
        } = self;
        let Vertex {
            position: p2,
            normal: n2,
            u: u2,
            v: v2,
            color: c2,
            intensity: i2,
        } = other;

        let (u1, v1) = (u1 * p1.w, v1 * p1.w);
        let (u2, v2) = (u2 * p2.w, v2 * p2.w);
        let i1 = i1 * p1.w;
        let i2 = i2 * p2.w;
        let position = p1.interpolate(p2, factor);
        let normal = n1.interpolate(n2, factor);
        let u = u1.interpolate(&u2, factor) / position.w;
        let v = v1.interpolate(&v2, factor) / position.w;
        let intensity = i1.interpolate(&i2, factor) / position.w;
        // 也应当考虑透视校正
        let color = c1.interpolate(&c2, factor);

        Vertex {
            position,
            normal,
            u,
            v,
            color,
            intensity,
        }
    }
}

impl Interpolate for Color {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let r = self.r.interpolate(&other.r, factor);
        let g = self.g.interpolate(&other.g, factor);
        let b = self.b.interpolate(&other.b, factor);
        let a = self.a.interpolate(&other.a, factor);
        Color::new(r, g, b, a)
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
