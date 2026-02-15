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

use crate::types::{Lines, Point3D, Points};

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

pub fn read_obj(fname: &str) -> io::Result<(Vec<Point3D>, Vec<Vec<usize>>, egui::Rect)> {
    // 1. Abrir el archivo
    let path = Path::new(fname);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let minxy: egui::Pos2 = egui::pos2(f32::MAX, f32::MAX);
    let maxxy: egui::Pos2 = egui::pos2(f32::MIN, f32::MIN);
    let mut worldr: egui::Rect = egui::Rect::from_min_max(minxy, maxxy);

    // 2. Iterar sobre las líneas de forma eficiente
    let mut vs = vec![];
    let mut fs = vec![];
    for line in reader.lines() {
        let line = line?; // Manejar posibles errores de lectura

        // 3. Filtrar por el prefijo deseado
        // if line.starts_with("v ") || line.starts_with("f ") {
        //     // Aquí puedes procesar la cadena
        //     println!("Procesando: {}", line);
        //
        //     // Si quisieras extraer los números, podrías usar line.split_whitespace()
        // }

        if line.starts_with("v ") {
            let coords: Vec<f32> = line
                .split_whitespace()
                .skip(1) // Saltarse la "v"
                .map(|s| s.parse().unwrap())
                .collect();
            // Ahora coords es algo como [1.0, 0.5, -2.0]
            let p = Point3D {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            };
            vs.push(p);

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
        // println!("{:?}", vs);

        if line.starts_with("f ") {
            let vertices = parse_face(&line);
            fs.push(vertices);
        }
        //println!("{:?}", fs);
    }

    Ok((vs, fs, worldr))
}
