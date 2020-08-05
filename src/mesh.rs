use crate::{texture::Texture, vector::Vector, vertex::Vertex};
use anyhow::{anyhow, bail, Result};
use sdl2::pixels::Color;
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
                        let vertex = Vertex::new(p, n, u, v, Color::RGBA(255, 255, 255, 255));
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
