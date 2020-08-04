use crate::{interpolate::Interpolate, matrix::Matrix, mesh::Mesh, vector::Vector, vertex::Vertex};
use sdl2::render::TextureCreator;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{self, Texture},
    surface::Surface,
    video::{Window, WindowContext},
    Sdl,
};
use std::{mem::swap, time::Duration};

pub struct Canvas<'a> {
    texture_creator: TextureCreator<WindowContext>,
    buffer: Surface<'a>,
    renderer: render::Canvas<Window>,
    width: u32,
    height: u32,
}

impl<'a> Canvas<'a> {
    pub fn new(sdl: &Sdl, title: &str, width: u32, height: u32) -> Self {
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

    pub fn render(&mut self) {
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

    pub fn clear(&mut self) {
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

    pub fn draw_point(&mut self, point: &Vector, color: Color) {
        let x = point.x.round() as u32;
        let y = point.y.round() as u32;
        let Self { width, height, .. } = *self;
        if x < width && y < height {
            self.set_pixel(x, y, color);
        }
    }

    pub fn draw_scanline<'b>(&mut self, mut v1: &'b Vertex, mut v2: &'b Vertex) {
        if v1.position.x > v2.position.x {
            swap(&mut v1, &mut v2);
        }
        let x1 = v1.position.x;
        let x2 = v2.position.x;
        let (start, end) = (x1 as u32, x2 as u32);
        for x in start..=end {
            let factor = if x1 == x2 {
                0.0
            } else {
                (x as f32 - x1) / (x2 - x1)
            };

            let v = v1.interpolate(&v2, factor);
            // println!("factor {} {} {} {} {:?}", factor, x, x1, x2, v);
            self.draw_point(&v.position, v.color);
        }
    }

    pub fn draw_triangle<'b>(
        &mut self,
        mut v1: &'b Vertex,
        mut v2: &'b Vertex,
        mut v3: &'b Vertex,
    ) {
        if v1.position.y > v2.position.y {
            swap(&mut v1, &mut v2);
        }
        if v2.position.y > v3.position.y {
            swap(&mut v2, &mut v3);
        }
        if v1.position.y > v2.position.y {
            swap(&mut v1, &mut v2);
        }
        let middle_factor = (v2.position.y - v1.position.y) / (v3.position.y - v1.position.y);
        let middle = v1.interpolate(v3, middle_factor);
        // println!("middle {:?}", middle);
        let start_y = v1.position.y as i32;
        let end_y = v2.position.y as i32;
        for y in start_y..=end_y {
            let factor = (y - start_y) as f32 / (end_y - start_y) as f32;
            let va = v1.interpolate(v2, factor);
            let vb = v1.interpolate(&middle, factor);
            self.draw_scanline(&va, &vb);
        }
        let start_y = v2.position.y as i32;
        let end_y = v3.position.y as i32;
        for y in start_y..=end_y {
            let factor = (y - start_y) as f32 / (end_y - start_y) as f32;
            let va = v2.interpolate(v3, factor);
            let vb = middle.interpolate(v3, factor);
            self.draw_scanline(&va, &vb);
        }
    }

    pub fn project(&self, v: &Vertex, transform: &Matrix) -> Vertex {
        let mut p = transform.transform(&v.position);
        let w = self.width as f32;
        let h = self.height as f32;
        p.x = p.x * w + w / 2.0;
        p.y = -p.y * h + h / 2.0;

        Vertex::new(p, v.color)
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        let camera_position = Vector::new(0.0, 0.0, -20.0);
        let camera_target = Vector::new(0.0, 0.0, 0.0);
        let camera_up = Vector::new(0.0, 1.0, 0.0);

        let view = Matrix::look_at_lh(&camera_position, &camera_target, &camera_up);
        let projection =
            Matrix::perspective_fov_lh(0.8, self.width as f32 / self.height as f32, 0.1, 1.0);
        let rotation = Matrix::rotation(&mesh.rotation);
        let translation = Matrix::translation(&mesh.position);

        let world = rotation * translation;
        let transform = world * view * projection;

        for (i, j, k) in &mesh.indices {
            let a = &mesh.vertices[*i];
            let b = &mesh.vertices[*j];
            let c = &mesh.vertices[*k];

            let v1 = self.project(a, &transform);
            let v2 = self.project(b, &transform);
            let v3 = self.project(c, &transform);
            self.draw_triangle(&v1, &v2, &v3);
        }
    }
}
