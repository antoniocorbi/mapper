// Copyright (C) 2026  Antonio-M. Corbi Bellot
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

use egui::Rect;

/// A type alias for a vector of `Point2D`s, representing a collection of map points.
pub type Map = Vec<Point2D>;

/// Represents a 2D point with `x` and `y` coordinates.
///
/// This struct is `Debug`, `Copy`, and `Clone` for easy manipulation and printing.
#[derive(Debug, Copy, Clone)]
pub struct Point2D {
    /// The x-coordinate of the point.
    pub x: f32,
    /// The y-coordinate of the point.
    pub y: f32,
}

impl Point2D {
    /// Transforms the `Point2D` from a world coordinate system to a screen coordinate system.
    ///
    /// This method uses linear remapping to translate the point's coordinates
    /// from a source `world` rectangle to a target `screen` rectangle.
    ///
    /// # Arguments
    ///
    /// * `world` - The `egui::Rect` defining the boundaries of the world coordinate system.
    /// * `screen` - The `egui::Rect` defining the boundaries of the screen coordinate system.
    ///
    /// # Returns
    ///
    /// A new `Point2D` with coordinates remapped to the screen coordinate system.
    pub fn world2screen(&self, world: Rect, screen: Rect) -> Point2D {
        let x = egui::remap(
            self.x,
            world.min.x..=world.max.x,
            screen.min.x..=screen.max.x,
        );
        let y = egui::remap(
            self.y,
            world.min.y..=world.max.y,
            screen.min.y..=screen.max.y,
        );
        Point2D { x, y }
    }
}
