extern crate sdl2;

use render::TextureCreator;
use sdl2::event::Event;
use sdl2::{
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{self, Texture},
    surface::Surface,
    video::{Window, WindowContext},
    Sdl,
};

use std::time::Duration;

mod vector;

use vector::Vector;
struct Canvas<'a> {
    texture_creator: TextureCreator<WindowContext>,
    buffer: Surface<'a>,
    renderer: render::Canvas<Window>,
    width: u32,
    height: u32,
}

impl<'a> Canvas<'a> {
    fn new(sdl: &Sdl, title: &str, width: u32, height: u32) -> Self {
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
        Canvas {
            buffer: surface,
            renderer: canvas,
            texture_creator,
            width,
            height,
        }
    }

    fn render(&mut self) {
        let Self {
            buffer,
            renderer,
            texture_creator,
            ..
        } = self;
        let texture = Texture::from_surface(buffer, texture_creator).unwrap();
        renderer.copy(&texture, None, None).unwrap();
        self.renderer.present()
    }

    fn clear(&mut self) {
        self.buffer
            .fill_rect(None, Color::RGBA(0, 0, 0, 255))
            .unwrap();
        self.renderer.clear()
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let w = self.width;
        self.buffer.with_lock_mut(|pixels| {
            let i = ((w * y + x) * 4) as usize;
            let Color { r, g, b, a } = color;
            pixels[i] = r;
            pixels[i + 1] = g;
            pixels[i + 2] = b;
            pixels[i + 3] = a;
        })
    }

    fn draw_point(&mut self, x: u32, y: u32, color: Color) {
        let Self { width, height, .. } = *self;
        if x < width && y < height {
            self.set_pixel(x, y, color);
        }
    }

    fn draw_scanline(&mut self, v1: Vector, v2: Vector, color: Color) {
        let Self { width, height, .. } = *self;
        if x < width && y < height {
            self.set_pixel(x, y, color);
        }
    }
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut canvas = Canvas::new(&sdl_context, "game", 800, 600);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut c = 0;
    'running: loop {
        canvas.clear();
        for i in 0..=800 {
            for j in 0..=600 {
                canvas.draw_point(i, j, Color::RGBA(c, 0, 255 - c, 255));
            }
        }
        c = (c + 1) % 255;
        canvas.render();
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
