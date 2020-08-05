use sdl2::event::Event;
use sdl2::{keyboard::Keycode, pixels::Color};

use std::time::Duration;

mod canvas;
mod interpolate;
mod matrix;
mod mesh;
mod texture;
mod vector;
mod vertex;

use canvas::Canvas;
use mesh::Mesh;
use vector::Vector;
use vertex::Vertex;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut canvas = Canvas::new(&sdl_context, "game", 800, 600);

    let mut mesh = Mesh::load("assets/illidan.3d", Some("assets/illidan.image")).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Up => mesh.rotation.x += 1.0,
                    Keycode::Down => mesh.rotation.y -= 1.0,
                    Keycode::Left => mesh.rotation.y += 1.0,
                    Keycode::Right => mesh.rotation.y -= 1.0,
                    Keycode::W => mesh.position.x += 1.0,
                    Keycode::S => mesh.position.x -= 1.0,
                    Keycode::A => mesh.position.y -= 1.0,
                    Keycode::D => mesh.position.y += 1.0,
                    _ => {}
                },
                _ => {}
            }
        }
        canvas.draw_mesh(&mesh);
        canvas.render();
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
            Vector::default(),
            0.0,
            0.0,
            Color::RGBA(255, 255, 0, 255),
        );
        let v2 = Vertex::new(
            Vector::new(200.0, 200.0, 0.0),
            Vector::default(),
            0.0,
            0.0,
            Color::RGBA(0, 255, 255, 255),
        );
        let v3 = Vertex::new(
            Vector::new(500.0, 110.0, 0.0),
            Vector::default(),
            0.0,
            0.0,
            Color::RGBA(255, 0, 255, 255),
        );

        canvas.draw_triangle(&v1, &v2, &v3, None);
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
