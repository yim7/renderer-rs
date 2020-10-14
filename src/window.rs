use crate::{canvas::Canvas, mesh::Mesh};
use anyhow::{anyhow, Result};
use sdl2::{event::Event, keyboard::Keycode, Sdl};
pub struct Window<'a> {
    sdl: Sdl,
    canvas: Canvas<'a>,
    mesh: Option<Mesh>,
    running: bool,
}

impl<'a> Window<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl = sdl2::init().map_err(|_| anyhow!("sdl init error"))?;
        let canvas = Canvas::new(&sdl, title, width, height);
        Ok(Window {
            sdl,
            canvas,
            mesh: None,
            running: true,
        })
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = Some(mesh);
    }

    pub fn run(&mut self) {
        while self.running {
            self.update_input();
            self.update();
            self.clear();
            self.draw();
            self.show();
        }
    }

    fn update(&mut self) {
        self.canvas.clear();
    }

    fn draw(&mut self) {
        let mesh = match &mut self.mesh {
            Some(mesh) => mesh,
            None => return,
        };
        self.canvas.draw_mesh(mesh);
    }

    fn show(&mut self) {
        self.canvas.render();
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn on_keydown_event(&mut self, key: Keycode) {
        let mut mesh = match &mut self.mesh {
            Some(mesh) => mesh,
            None => return,
        };
        match key {
            Keycode::Up => mesh.rotation.x += 0.5,
            Keycode::Down => mesh.rotation.x -= 0.5,
            Keycode::Left => mesh.rotation.y += 0.5,
            Keycode::Right => mesh.rotation.y -= 0.5,
            Keycode::W => mesh.position.y += 0.5,
            Keycode::S => mesh.position.y -= 0.5,
            Keycode::A => mesh.position.x -= 0.5,
            Keycode::D => mesh.position.x += 0.5,
            _ => {}
        }
    }

    fn on_mouse_event(&mut self, y: i32) {
        let mut mesh = match &mut self.mesh {
            Some(mesh) => mesh,
            None => return,
        };
        if y < 0 {
            mesh.position.z += 0.5;
        } else {
            mesh.position.z -= 0.5;
        }
        // println!("update z {}", mesh.position.z);
    }

    fn update_input(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit(),
                Event::KeyDown {
                    keycode: Some(key), ..
                } => self.on_keydown_event(key),
                Event::MouseWheel { y, .. } => self.on_mouse_event(y),
                _ => {}
            }
        }
    }
}
