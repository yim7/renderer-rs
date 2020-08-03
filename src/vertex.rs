use crate::vector::Vector;
use sdl2::pixels::Color;
#[derive(Debug)]
pub struct Vertex {
    pub position: Vector,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Vector, color: Color) -> Self {
        Vertex { position, color }
    }
}
