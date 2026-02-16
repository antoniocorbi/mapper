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
use egui::{pos2, remap, Color32, Pos2, Rect, Stroke};

// -- Constants: ----------------------------------------------------------
const MIN_ZOOM: f32 = 0.001;
const MAX_ZOOM: f32 = 15.00;

const MIN_WIDTH: f32 = 0.01;
const MAX_WIDTH: f32 = 2.00;

// -- Structs: ------------------------------------------------------------
pub struct AppMap {
    zoom: f32,
    line_width: f32,
    file_path: String,
    error_message: String,
    points: Map,
    worldr: Rect,
    invert_y: bool,
}

// -- Implementation AppMap: -----------------------------------------------
impl AppMap {
    pub fn new() -> Self {
        let worldr: Rect = Rect::from_min_max(pos2(-1.0, -1.0), pos2(1.0, 1.0));
        let points = vec![];

        Self {
            zoom: 1.0,
            line_width: 0.05,
            file_path: String::from("assets/coastline.dat"),
            error_message: String::new(),
            points,
            worldr,
            invert_y: true,
        }
    }

    fn draw_point(p: Point2D, zoom: f32, line_width: f32, painter: &egui::Painter) {
        // También puedes obtener los límites
        // let min = painter.clip_rect().min; // Esquina superior izquierda (Pos2)
        // let max = painter.clip_rect().max; // Esquina inferior derecha (Pos2)

        let centro = pos2(p.x, p.y);
        let mut radio = zoom;
        // let radio = zoom.min(3.5);
        // let radio = ((zoom + 0.125) / 2.5).max(3.5);
        // let color = Color32::from_rgb(255, 255, 255);
        let color = Color32::CYAN;

        radio = line_width;

        painter.circle_filled(centro, radio, color);
    }

    fn draw_lines(lines: &Vec<Pos2>, painter: &egui::Painter) {
        let stroke = Stroke::new(0.5, egui::Color32::LIGHT_YELLOW);
        painter.line(lines.to_vec(), stroke);
    }

    fn draw_map(&self, painter: &egui::Painter) {
        //let dz = MAX_ZOOM - self.zoom;
        let worldr: Rect = self.worldr / self.zoom;
        let screenr: Rect = painter.clip_rect();

        for wp in &self.points {
            let mut iwp = *wp;
            if self.invert_y {
                iwp.y *= -1.0;
            }
            let sp = iwp.world2screen(worldr, screenr);
            AppMap::draw_point(sp, self.zoom, self.line_width, painter);
        }
    }

    pub fn draw_contents(&self, painter: &egui::Painter) {
        //self.draw_circle(painter);
        self.draw_map(painter);
    }
}

// -- Implementation eframe@AppMap: ----------------------------------------
impl eframe::App for AppMap {
    /// Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
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
                        //ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        //});
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

                                println!("File read: wr: {:?}", wr);

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
                            // dbg!(ERROR_TIMEOUT);
                            if ERROR_TIMEOUT == 0 {
                                ERROR_TIMEOUT = TIMEOUT;
                                self.error_message = "".to_string();
                            }
                        }
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    // ui.checkbox(&mut self.rotx, "Rotate X");
                    // ui.checkbox(&mut self.roty, "Rotate Y");
                    // ui.checkbox(&mut self.rotz, "Rotate Z");
                    // ui.separator();

                    ui.separator();
                    ui.colored_label(egui::Color32::LIGHT_YELLOW, "Zoom: ");
                    ui.add(
                        egui::DragValue::new(&mut self.zoom)
                            .speed(0.1)
                            .range(MIN_ZOOM..=MAX_ZOOM),
                    );
                    ui.separator();

                    ui.separator();
                    ui.colored_label(egui::Color32::YELLOW, "Line Width: ");
                    ui.add(
                        egui::DragValue::new(&mut self.line_width)
                            .speed(0.01)
                            .range(MIN_WIDTH..=MAX_WIDTH),
                    );
                    ui.separator();

                    ui.checkbox(&mut self.invert_y, "Invert Y axis");
                    ui.separator();

                    if ui.button("Restart View").clicked() {
                        //self.calculate_bounds_and_fit(ui.available_rect_before_wrap());
                        *self = Self::new();
                    }
                });

                ui.separator();
            });

            // El área de dibujo para el objeto 3D
            let mut available_rect_before_wrap = ui.available_rect_before_wrap();
            available_rect_before_wrap.max.y -= 70.0; // Important for clipping
            let painter = ui.painter_at(available_rect_before_wrap);

            // Dibujar un fondo para el área del mapa
            painter.rect_filled(
                available_rect_before_wrap,
                0.0,
                egui::Color32::from_rgb(50, 50, 50),
            );
            // let screenr: Rect = painter.clip_rect();
            // painter.set_clip_rect(screenr);

            self.draw_contents(&painter);

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
    use super::*; // Importa todo lo del archivo padre
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
