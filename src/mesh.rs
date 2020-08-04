use crate::{vector::Vector, vertex::Vertex};
use anyhow::{anyhow, bail, Result};
use sdl2::pixels::Color;
use std::fs::File;
use std::io::prelude::*;
pub struct Mesh {
    pub position: Vector,
    pub rotation: Vector,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn load(path: &str) -> Result<Mesh> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // println!("mesh\n{}", content);
        Self::parse(content)
    }

    pub fn parse(content: String) -> Result<Mesh> {
        let mut lines = content.split('\n');

        // 吃掉格式描述信息
        lines.next();
        lines.next();

        // 读取顶点和索引行数
        let number_of_vertices: usize = lines
            .next()
            .ok_or(anyhow!("顶点格式错误"))?
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse()?;
        println!("number of vertices {}", number_of_vertices);
        let number_of_indices: usize = lines
            .next()
            .ok_or(anyhow!("顶点索引格式错误"))?
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse()?;
        println!("number of indices {}", number_of_indices);
        let mut vertices = vec![];
        for _ in 0..number_of_vertices {
            let line = lines.next().ok_or(anyhow!("模型格式错误"))?;
            // println!("vertex line {}", line);
            let v: Vec<f32> = line
                .split(' ')
                .map(|a| a.trim())
                .map(|a| a.parse())
                .collect::<Result<Vec<f32>, _>>()?;
            match v[..] {
                [x, y, z, ..] => {
                    let p = Vector::new(x, y, z);
                    let v = Vertex::new(p, Color::RGBA(255, 255, 255, 255));
                    vertices.push(v);
                }
                _ => bail!("解析顶点数据错误"),
            }
        }
        println!("read vertices {}", vertices.len());
        let mut indices = vec![];
        for _ in 0..number_of_indices {
            let line = lines.next().ok_or(anyhow!("模型格式错误"))?;
            let v: Vec<usize> = line
                .split(' ')
                .map(|a| a.trim())
                .map(|a| a.parse())
                .collect::<Result<Vec<usize>, _>>()?;
            match v[..] {
                [i, j, k] => indices.push((i, j, k)),
                _ => bail!("解析顶点索引错误"),
            }
        }

        Ok(Mesh {
            position: Vector::default(),
            rotation: Vector::default(),
            vertices,
            indices,
        })
    }
}
