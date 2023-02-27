use std::fs::File;
use std::io::{BufRead, BufReader};

use kiss3d::nalgebra::Point3;

// Note: This is a very basic, incomplete PLY reader. Probably not very robust.
pub fn parse_ply_file(path: &str) -> Result<Vec<Point3<f32>>, String> {
    let file = File::open(path).map_err(|_| "Error opening PLY file")?;
    let reader = BufReader::new(file);
    let mut num_vertices = 0;
    let mut vertices: Vec<Point3<f32>> = Vec::new();
    let mut reading_vertices = false;

    for line in reader.lines() {
        let line = line.map_err(|_| "Error reading PLY file")?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if let Some(&"comment") = parts.first() {
            continue;
        }

        if let Some(&"element") = parts.first() {
            if let Some(&"vertex") = parts.get(1) {
                num_vertices = parts
                    .get(2)
                    .and_then(|v| v.parse().ok())
                    .ok_or("Bad PLY: missing vertices")?;
            }
        }

        if !reading_vertices {
            if line.trim() == "end_header" {
                reading_vertices = true;
            }
            continue;
        }

        if reading_vertices && vertices.len() < num_vertices {
            if parts.len() < 3 {
                return Err(
                    "Bad PLY: missing coordinates".to_string()
                );
            }

            let x = parts[0]
                .parse()
                .map_err(|_| "Bad PLY: invalid x")?;
            let y = parts[1]
                .parse()
                .map_err(|_| "Bad PLY: invalid y")?;
            let z = parts[2]
                .parse()
                .map_err(|_| "Bad PLY: invalid z")?;
            vertices.push(Point3::new(x, y, z));
        }
    }

    if vertices.len() != num_vertices {
        return Err(
            "Bad PLY: vertex count mismatch".to_string()
        );
    }

    Ok(vertices)
}
