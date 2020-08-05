use crate::vector::Vector;
use sdl2::pixels::Color;
#[derive(Debug)]
pub struct Vertex {
    pub position: Vector,
    pub normal: Vector,
    pub u: f32,
    pub v: f32,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Vector, normal: Vector, u: f32, v: f32, color: Color) -> Self {
        Vertex {
            position,
            normal,
            u,
            v,
            color,
        }
    }
}
