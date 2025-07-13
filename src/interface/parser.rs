use std::error::Error;
use std::fs;
use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use crate::math::mesh::Mesh;
use crate::math::triangle::Triangle;
use crate::math::vertex::Vertex;
use crate::rendering::stroke::Stroke;

pub fn parse_mesh(path: &Path) -> Result<Mesh, String> {
    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(e) => return Err(e.to_string()),
    };
    
    let mut vertexes: Vec<Vertex> = Vec::new();
    let mut faces: Vec<Triangle> = Vec::new();
    
    for line in contents.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some(token) => 
                match token {
                    "v" => {
                        let mut components: Vec<f32> = Vec::new();
                        for i in 0..3 {
                            match tokens.next() {
                                Some(token) => components.push(match token.parse::<f32>() {
                                    Ok(v) => v,
                                    Err(e) => return Err(e.to_string()),
                                }),
                                None => return Err(format!("Invalid vertex line: {}", line)),
                            }
                        }
                        vertexes.push(Vertex::new(components[0], components[1], components[2]));
                    },
                    "f" => {
                        let mut components: Vec<Vertex> = Vec::new();
                        for i in 0..3 {
                            match tokens.next() {
                                Some(token) => components.push(vertexes[match token.split('/').next().unwrap().parse::<usize>() {
                                    Ok(n) => n,
                                    Err(e) => return Err(e.to_string()),
                                } - 1]),
                                None => return Err(format!("Invalid face line: {}", line)),
                            }
                        }
                        faces.push(Triangle::from_vertexes(components[0], components[1], components[2], Stroke::new([255, 255, 255], '█')))
                    }
                    _ => {},
                }
            None => {}
        }
    }
    
    Ok(Mesh::new(faces))
}