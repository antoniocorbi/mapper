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

/// Parses a "face" line from an OBJ file format.
///
/// This function extracts vertex indices from a line starting with "f",
/// adjusting them from 1-based (OBJ) to 0-based (Rust vector indexing).
/// It expects the format `f v1[/vt1][/vn1] v2[/vt2][/vn2] ...`.
///
/// # Arguments
///
/// * `line` - A string slice representing a line from an OBJ file, expected
///            to start with 'f'.
///
/// # Returns
///
/// A `Vec<usize>` containing the 0-based vertex indices.
///
/// # Panics
///
/// Panics if a vertex index cannot be parsed as a `usize`.
fn parse_face(line: &str) -> Vec<usize> {
    line.split_whitespace() // Separates "f", "23/1/23", "3/2/3", etc.
        .skip(1) // Ignores the "f"
        .map(|blk| {
            // Takes only what is before the first '/'
            let idx_str = blk.split('/').next().unwrap();
            // Converts to number (adjusting the 1-based OBJ index to 0-based Rust)
            idx_str.parse::<usize>().expect("Invalid vertex index") - 1 // Rust vector indexes start @0 not 1
        })
        .collect()
}

/// Reads map data from a specified file and computes its `egui::Rect` bounds.
///
/// This function opens a file, reads it line by line, and parses each line
/// into `Point2D` objects. It also determines the minimum and maximum
/// X and Y coordinates to establish the `worldr` (world rectangle) of the map.
/// Lines starting with '#' or empty lines are ignored.
///
/// # Arguments
///
/// * `fname` - The path to the file containing the map data.
///
/// # Returns
///
/// A `io::Result` containing a tuple:
/// * `Map`: A `Vec<Point2D>` representing the parsed map data.
/// * `egui::Rect`: The bounding box (`worldr`) of all points in the map.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be opened or read.
pub fn read_map(fname: &str) -> io::Result<(Map, egui::Rect)> {
    // 1. Open the file
    let path = Path::new(fname);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let minxy: egui::Pos2 = egui::pos2(f32::MAX, f32::MAX);
    let maxxy: egui::Pos2 = egui::pos2(f32::MIN, f32::MIN);
    let mut worldr: egui::Rect = egui::Rect::from_min_max(minxy, maxxy);

    // 2. Iterate over the lines efficiently
    let mut m: Map = vec![];
    for line in reader.lines() {
        let line = line?; // Handle potential read errors

        if !line.starts_with("#") && line.len() > 0 {
            let coords: Vec<f32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            // Now coords is something like [1.0, 0.5]
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
    }

    Ok((m, worldr))
}
