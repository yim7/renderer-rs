use anyhow::{anyhow, bail, Result};
use sdl2::pixels::{Color, PixelFormatEnum};
use std::fs::File;
use std::io::prelude::*;

pub struct Texture {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // println!("mesh\n{}", content);
        Self::parse(content)
    }

    pub fn parse(content: String) -> Result<Self> {
        let mut lines = content.split('\n');
        lines.next();
        lines.next();
        let width = lines
            .next()
            .ok_or(anyhow!("贴图宽度缺失"))?
            .trim()
            .parse()?;
        let height = lines
            .next()
            .ok_or(anyhow!("贴图长度缺失"))?
            .trim()
            .parse()?;
        println!("texture w {} h {}", width, height);
        let mut pixels = vec![];
        for _ in 0..height {
            let line = lines.next().ok_or(anyhow!("贴图数据行数不完整"))?;
            let mut ps = line.split(' ');
            for _ in 0..width {
                let pixel: u32 = ps.next().ok_or(anyhow!("贴图数据列数不完整"))?.parse()?;
                let r = (pixel >> 24) & 0xFF;
                let g = (pixel >> 16) & 0xFF;
                let b = (pixel >> 0) & 0xFF;
                let a = (pixel) & 0xFF;
                let color = Color::RGBA(r as u8, g as u8, b as u8, a as u8);
                pixels.push(color);
            }
        }

        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    pub fn sample(&self, mut u: f32, mut v: f32) -> Color {
        if u > 1.0 {
            u = 1.0
        } else if u < 0.0 {
            u = 0.0
        }
        if v > 1.0 {
            v = 1.0
        } else if v < 0.0 {
            u = 0.0
        }

        let w = self.width;
        let h = self.height;
        let tu = (w - 1) as f32 * u;
        let tv = (h - 1) as f32 * v;

        let index = tu as usize + w * tv as usize;
        // println!("u {} v {} index {} color {:?}", u, v, index, color);
        self.pixels[index]
    }
}
