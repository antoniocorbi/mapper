// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::types::{Map, Point2D};

fn parse_face(line: &str) -> Vec<usize> {
    line.split_whitespace() // Separa "f", "23/1/23", "3/2/3", etc.
        .skip(1) // Ignora la "f"
        .map(|blk| {
            // Tomamos solo lo que está antes del primer '/'
            let idx_str = blk.split('/').next().unwrap();
            // Convertimos a número (ajustando el índice 1 del OBJ al 0 de Rust)
            idx_str
                .parse::<usize>()
                .expect("Índice de vértice no válido")
                - 1 // Rust vector indexes start @0 not 1
        })
        .collect()
}

pub fn read_map(fname: &str) -> io::Result<(Map, egui::Rect)> {
    // 1. Abrir el archivo
    let path = Path::new(fname);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let minxy: egui::Pos2 = egui::pos2(f32::MAX, f32::MAX);
    let maxxy: egui::Pos2 = egui::pos2(f32::MIN, f32::MIN);
    let mut worldr: egui::Rect = egui::Rect::from_min_max(minxy, maxxy);

    // 2. Iterar sobre las líneas de forma eficiente
    let mut m: Map = vec![];
    for line in reader.lines() {
        let line = line?; // Manejar posibles errores de lectura

        if !line.starts_with("#") && line.len() > 0 {
            let coords: Vec<f32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            // Ahora coords es algo como [1.0, 0.5]
            let p = Point2D {
                x: coords[0],
                y: coords[1],
            };
            m.push(p);

            // Compute new worldr
            if p.x < worldr.min.x {
                worldr.min.x = p.x;
            }
            if p.y < worldr.min.y {
                worldr.min.y = p.y;
            }
            if p.x > worldr.max.x {
                worldr.max.x = p.x;
            }
            if p.y > worldr.max.y {
                worldr.max.y = p.y;
            }
        }
        // println!("{:?}", m);
    }

    Ok((m, worldr))
}
