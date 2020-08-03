use crate::vector::Vector;
use crate::vertex::Vertex;
use sdl2::pixels::Color;

pub trait Interpolate {
    fn interpolate(&self, other: &Self, factor: f32) -> Self;
}

impl Interpolate for Vector {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let x = self.x + (other.x - self.x) * factor;
        let y = self.y + (other.y - self.y) * factor;
        let z = self.z + (other.z - self.z) * factor;
        Self { x, y, z }
    }
}

impl Interpolate for Vertex {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let position = self.position.interpolate(&other.position, factor);
        let color = self.color.interpolate(&other.color, factor);
        Vertex { position, color }
    }
}

impl Interpolate for Color {
    fn interpolate(&self, other: &Self, factor: f32) -> Self {
        let r = self.r as f32 + (other.r as f32 - self.r as f32) * factor;
        let g = self.g as f32 + (other.g as f32 - self.g as f32) * factor;
        let b = self.b as f32 + (other.b as f32 - self.b as f32) * factor;
        let a = self.a as f32 + (other.a as f32 - self.a as f32) * factor;
        Color::RGBA(r as u8, g as u8, b as u8, a as u8)
    }
}
