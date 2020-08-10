use crate::color::Color;
use crate::{texture::Texture, vector::Vector, vertex::Vertex};
use anyhow::{anyhow, bail, Result};
use std::fs::File;
use std::io::prelude::*;
pub struct Mesh {
    pub position: Vector,
    pub rotation: Vector,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<(usize, usize, usize)>,
    pub texture: Option<Texture>,
}

impl Mesh {
    pub fn set_position(&mut self, position: Vector) {
        self.position = position;
    }

    pub fn load_gua3d(path: &str, texture_path: Option<&str>) -> Result<Mesh> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // println!("mesh\n{}", content);

        let texture = if let Some(path) = texture_path {
            Some(Texture::load(path)?)
        } else {
            None
        };
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
                [x, y, z, nx, ny, nz, u, v] => {
                    let p = Vector::new(x, y, z);
                    let n = Vector::new(nx, ny, nz);
                    let v = Vertex::new(p, n, u, v, Color::new(255, 255, 255, 255));
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
            texture,
        })
    }

    pub fn load_obj(path: &str, texture_path: Option<&str>) -> Result<Mesh> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // println!("mesh\n{}", content);

        let texture = if let Some(path) = texture_path {
            Some(Texture::load(path)?)
        } else {
            None
        };
        let lines = content.split('\n');
        let mut positions = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];
        let mut vertices = vec![];

        let mut index = 0;
        for mut line in lines {
            line = line.trim();
            if line.is_empty() {
                continue;
            }
            let vs: Vec<&str> = line.split(' ').collect();
            match vs[0] {
                "v" => {
                    let x = vs[1].parse()?;
                    let y = vs[2].parse()?;
                    let z = vs[3].parse()?;
                    let v = Vector::new(x, y, z);
                    positions.push(v);
                }
                "vt" => {
                    let u = vs[1].parse()?;
                    let v = vs[2].parse()?;
                    uvs.push((u, v));
                }
                "vn" => {
                    let x = vs[1].parse()?;
                    let y = vs[2].parse()?;
                    let z = vs[3].parse()?;
                    let v = Vector::new(x, y, z);
                    normals.push(v);
                }
                "f" => {
                    let parse = |s: &str| -> Result<Vertex, std::num::ParseIntError> {
                        let vs: Vec<usize> =
                            s.split('/')
                                .map(|s| s.parse())
                                .collect::<Result<Vec<usize>, _>>()?;
                        let p = positions[vs[0] - 1];
                        let n = normals[vs[2] - 1];
                        let (u, v) = uvs[vs[1] - 1];
                        let vertex = Vertex::new(p, n, u, v, Color::new(255, 255, 255, 255));
                        Ok(vertex)
                    };
                    let v1 = parse(vs[1])?;
                    let v2 = parse(vs[2])?;
                    let v3 = parse(vs[3])?;
                    vertices.extend(vec![v1, v2, v3]);
                    indices.push((index, index + 1, index + 2));
                    index += 3;
                    // println!("{} {} {}", positions.len(), normals.len(), uvs.len());
                }
                _ => {}
            }
        }

        // println!("{} {}", vertices.len(), indices.len());
        Ok(Mesh {
            position: Vector::default(),
            rotation: Vector::default(),
            vertices,
            indices,
            texture,
        })
    }
}
