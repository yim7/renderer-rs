use sdl2::event::Event;
use sdl2::{keyboard::Keycode, pixels::Color};

use std::time::Duration;

mod canvas;
mod interpolate;
mod mesh;
mod vector;
mod vertex;

use canvas::Canvas;
use mesh::Mesh;
use vector::Vector;
use vertex::Vertex;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut canvas = Canvas::new(&sdl_context, "game", 800, 600);

    let mesh = Mesh::load("assets/illidan.gua3d").unwrap();
    canvas.draw_mesh(&mesh);
    canvas.render();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_draw_triangle() {
        let sdl_context = sdl2::init().unwrap();

        let mut canvas = Canvas::new(&sdl_context, "game", 800, 600);

        canvas.clear();

        let v1 = Vertex::new(
            Vector::new(300.0, 400.0, 0.0),
            Color::RGBA(255, 255, 0, 255),
        );
        let v2 = Vertex::new(
            Vector::new(200.0, 200.0, 0.0),
            Color::RGBA(0, 255, 255, 255),
        );
        let v3 = Vertex::new(
            Vector::new(500.0, 110.0, 0.0),
            Color::RGBA(255, 0, 255, 255),
        );

        canvas.draw_triangle(&v1, &v2, &v3);
        canvas.render();
        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
