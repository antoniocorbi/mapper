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

// -- Uses: ---------------------------------------------------------------
use crate::types::{Map, Point2D};
use egui::color_picker::color_edit_button_srgba;
use egui::widgets::color_picker::{color_edit_button_srgb, Alpha};
use egui::{pos2, remap, Color32, Pos2, Rect, Stroke};

// -- Constants: ----------------------------------------------------------
const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 4.00;

const MIN_WIDTH: f32 = 0.01;
const MAX_WIDTH: f32 = 2.00;

const MAP_W: f32 = 2000.0;
const MAP_H: f32 = 1500.0;

// -- Structs: ------------------------------------------------------------
/// `AppMap` is the main application struct that holds the state for the map visualization.
///
/// It manages zoom levels, line widths, file paths, error messages, map data points,
/// and the rendering rectangles for both world and screen coordinates.
pub struct AppMap {
    /// The current zoom level of the map.
    zoom: f32,
    /// The width of the lines drawn on the map.
    line_width: f32,
    /// The file path to the map data.
    file_path: String,
    /// Stores the last error message encountered during file operations or other processes.
    error_message: String,
    /// A collection of `Point2D` representing the map data.
    points: Map,
    /// The rectangle representing the world coordinates of the map.
    worldr: Rect,
    /// The rectangle representing the screen coordinates where the map is drawn.
    screenr: Rect,
    /// A flag indicating whether the Y-axis should be inverted during rendering.
    invert_y: bool,
    /// The color used for drawing map elements, stored as an RGB array.
    color: [u8; 3],
}

// -- Implementation AppMap: -----------------------------------------------
impl AppMap {
    /// Creates a new instance of `AppMap` with default values.
    ///
    /// The default map file path is "assets/coastline.dat", zoom is 1.0,
    /// and line width is 0.5. The world rectangle is initialized to a default
    /// 2x2 square centered at the origin, and the screen rectangle is zero.
    pub fn new() -> Self {
        let worldr: Rect = Rect::from_min_max(pos2(-1.0, -1.0), pos2(1.0, 1.0));
        let screenr: Rect = Rect::ZERO;
        let points = vec![];

        Self {
            zoom: 1.0,
            line_width: 0.5,
            file_path: String::from("assets/coastline.dat"),
            error_message: String::new(),
            points,
            worldr,
            screenr,
            invert_y: true,
            color: [255, 215, 103],
        }
    }

    /// Draws a single `Point2D` on the `egui::Painter`.
    ///
    /// The point is drawn as a filled circle with a radius determined by `line_width`.
    ///
    /// * `p`: The `Point2D` to draw, in screen coordinates.
    /// * `zoom`: The current zoom level (currently unused directly for point size).
    /// * `line_width`: The radius of the drawn circle.
    /// * `color`: The `Color32` to fill the circle with.
    /// * `painter`: The `egui::Painter` used for drawing.
    fn draw_point(p: Point2D, line_width: f32, color: Color32, painter: &egui::Painter) {
        let centro = pos2(p.x, p.y);
        let radio = line_width;
        painter.circle_filled(centro, radio, color);
    }

    /// Draws a series of connected lines on the `egui::Painter`.
    ///
    /// This function takes a vector of `Pos2` points and draws lines between them.
    ///
    /// * `lines`: A reference to a vector of `Pos2` representing the vertices of the lines.
    /// * `line_width`: The thickness of the lines.
    /// * `color`: The `Color32` of the lines.
    /// * `painter`: The `egui::Painter` used for drawing.
    fn draw_lines(lines: &Vec<Pos2>, line_width: f32, color: Color32, painter: &egui::Painter) {
        let stroke = Stroke::new(line_width, color);
        painter.line(lines.to_vec(), stroke);
    }

    /// Draws the entire map data stored in `self.points` onto the `egui::Painter`.
    ///
    /// Each point in `self.points` is transformed from world to screen coordinates,
    /// optionally inverted along the Y-axis, and then drawn as a point.
    ///
    /// * `painter`: The `egui::Painter` used for drawing.
    fn draw_map(&self, painter: &egui::Painter) {
        let worldr: Rect = self.worldr;
        let screenr: Rect = self.screenr;

        for wp in &self.points {
            let mut iwp = *wp;
            if self.invert_y {
                iwp.y *= -1.0;
            }

            let sp = iwp.world2screen(worldr, screenr);
            let color = Color32::from_rgb(self.color[0], self.color[1], self.color[2]);
            AppMap::draw_point(sp, self.line_width, color, painter);
        }
    }

    /// Draws all visible contents of the `AppMap` onto the provided `egui::Painter`.
    ///
    /// Currently, this function primarily calls `draw_map`.
    ///
    /// * `painter`: The `egui::Painter` used for drawing.
    pub fn draw_contents(&self, painter: &egui::Painter) {
        self.draw_map(painter);
    }
}

// -- Implementation eframe@AppMap: ----------------------------------------
impl eframe::App for AppMap {
    /// Called each time the UI needs repainting, which may be many times per second.
    ///
    /// This method handles all UI interactions and drawing for the application.
    /// It sets up the central panel, control widgets (file loading, zoom, line width, color),
    /// and the main drawing area for the map. It also manages error messages and continuous repainting.
    ///
    /// * `ctx`: The `egui::Context` providing access to the egui environment.
    /// * `frame`: The `eframe::Frame` providing information about the application window.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        static TIMEOUT: u32 = 200;
        static mut ERROR_TIMEOUT: u32 = 0;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Panel de controles en la parte superior
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "·:Mapper:·");

                    ui.colored_label(egui::Color32::LIGHT_BLUE, "Theme: ");
                    egui::widgets::global_theme_preference_buttons(ui);

                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        ui.add_space(16.0);
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Map file:");
                    ui.text_edit_singleline(&mut self.file_path);
                    if ui.button("Load file").clicked() {
                        match crate::files::read_map(&self.file_path) {
                            Err(e) => {
                                // Error reading the objfile
                                self.error_message = format!("Last Error: '{}'.", e);
                                unsafe {
                                    ERROR_TIMEOUT = TIMEOUT;
                                }
                            }

                            Ok((points, wr)) => {
                                // We had success reading the objfile
                                // 1. Process obj file just read
                                self.points = points;
                                self.worldr = wr;

                                // println!("File read: wr: {:?}", wr);

                                // 2. Restart timeout values
                                unsafe {
                                    // File loaded, remove error text right now!
                                    ERROR_TIMEOUT = 1;
                                }
                                self.error_message = "".to_string();
                            }
                        }
                    }
                    unsafe {
                        if ERROR_TIMEOUT > 0 {
                            ERROR_TIMEOUT -= 1;
                            if ERROR_TIMEOUT == 0 {
                                ERROR_TIMEOUT = TIMEOUT;
                                self.error_message = "".to_string();
                            }
                        }
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "Color: ");
                    color_edit_button_srgb(ui, &mut self.color);

                    ui.separator();
                    ui.colored_label(egui::Color32::RED, "Line Width: ");
                    ui.add(
                        egui::DragValue::new(&mut self.line_width)
                            .speed(0.02)
                            .range(MIN_WIDTH..=MAX_WIDTH),
                    );

                    ui.separator();
                    ui.colored_label(egui::Color32::RED, "Zoom: ");
                    ui.add(
                        egui::DragValue::new(&mut self.zoom)
                            .speed(0.015)
                            .range(MIN_ZOOM..=MAX_ZOOM),
                    );

                    ui.separator();

                    ui.checkbox(&mut self.invert_y, "Invert Y axis");
                    ui.separator();

                    if ui.button("Restart View").clicked() {
                        *self = Self::new();
                    }
                });

                ui.separator();
            });

            // The drawing area for the 3D object
            let mut available_size_before_wrap = ui.available_size_before_wrap();
            available_size_before_wrap.y -= 70.0; // Important for clipping

            // 1. Define the size of the visible "window" of the painter
            let window_size = available_size_before_wrap;

            use egui::Vec2;
            // 2. Allocate that space in the UI
            let (outer_rect, _) = ui.allocate_exact_size(window_size, egui::Sense::hover());

            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(outer_rect));
            let gray_rect = child_ui.max_rect();

            egui::ScrollArea::both() // Allows scrolling in both directions
                .auto_shrink([false; 2]) // Optional: prevents the area from collapsing
                .max_height(window_size.y) // Enforce the clipping area limit
                .max_width(window_size.x)
                .show(&mut child_ui, |ui| {
                    // 1. Define the total size of your "map" or drawing
                    let canvas_size = egui::Vec2::new(MAP_W, MAP_H);

                    // 2. Allocate the space and get the Painter
                    // allocate_painter returns a response (for events) and the painter
                    let (response, painter) =
                        ui.allocate_painter(canvas_size, egui::Sense::hover());

                    //dbg!(response.rect);
                    painter.rect_filled(gray_rect, 0.0, egui::Color32::from_rgb(50, 50, 50));

                    // Draw a background for the map area
                    // response.rect is the actual rectangle, not the 'clipped' one: [[8.0 99.0] - [1508.0 1599.0]]
                    self.screenr = response.rect * self.zoom;

                    self.draw_contents(&painter);
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe_and_me(ui);
                egui::warn_if_debug_build(ui);
                ui.separator();
                // Show last error
                ui.colored_label(egui::Color32::YELLOW, &self.error_message);
                ui.separator();
            });

            // Continuous update
            ctx.request_repaint();
        });
    }
}

// -- Free functions: -----------------------------------------------------
/// Displays a "Powered by egui and eframe" message along with a copyright notice.
///
/// This function creates a horizontal layout with hyper-links to the egui and eframe
/// repositories, followed by a copyright notice.
///
/// * `ui`: The `egui::Ui` to add the widgets to.
fn powered_by_egui_and_eframe_and_me(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(". © Antonio-M. Corbi 2026");
    });
}

#[cfg(test)]
mod test {
    use super::*; // Import all from the parent file
    #[test]
    fn w2s() {
        let wr: egui::Rect = egui::Rect::from_min_max(pos2(-7.3, -0.1), pos2(6.7, 17.4));
        let sr: egui::Rect = egui::Rect::from_min_max(pos2(8.0, 99.0), pos2(812.0, 843.5));
        let pw1 = Point2D { x: -7.3, y: -0.1 };
        let pw1 = Point2D { x: 6.7, y: 17.4 };
        let pw1 = Point2D { x: -0.3, y: 8.65 };

        let ps = pw1.world2screen(wr, sr);

        dbg!(ps);
    }
}
