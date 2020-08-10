use crate::{color::Color, vector::Vector};
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vertex {
    pub position: Vector,
    pub normal: Vector,
    pub u: f32,
    pub v: f32,
    pub color: Color,
    pub intensity: f32,
}

impl Vertex {
    pub fn new(position: Vector, normal: Vector, u: f32, v: f32, color: Color) -> Self {
        Vertex {
            position,
            normal,
            u,
            v,
            color,
            intensity: 1.0,
        }
    }
}
