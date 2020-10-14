use crate::color::Color;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;

pub struct Texture {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
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
            let line = lines.next().ok_or(anyhow!("贴图数据行数不完整"))?.trim();
            let mut ps = line.split(' ');
            for _ in 0..width {
                let p = ps.next().ok_or(anyhow!("贴图数据列数不完整"))?;
                // println!("p {:?}", p);
                let pixel: u32 = p.parse()?;
                let r = (pixel >> 24) & 0xFF;
                let g = (pixel >> 16) & 0xFF;
                let b = (pixel >> 8) & 0xFF;
                let a = (pixel) & 0xFF;
                let color = Color::new(r as u8, g as u8, b as u8, a as u8);
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
            v = 0.0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let t = Texture::load("assets/illidan.image").unwrap();
        let mut result = vec![];
        for i in 0..5 {
            let c = t.sample(i as f32 * 0.1, i as f32 * 0.1);
            result.push(c);
        }
        let expected = vec![
            Color {
                r: 24,
                g: 24,
                b: 16,
                a: 255,
            },
            Color {
                r: 71,
                g: 54,
                b: 31,
                a: 0,
            },
            Color {
                r: 38,
                g: 18,
                b: 16,
                a: 255,
            },
            Color {
                r: 66,
                g: 53,
                b: 72,
                a: 255,
            },
            Color {
                r: 43,
                g: 32,
                b: 51,
                a: 255,
            },
        ];
        assert_eq!(result, expected);
    }
}
