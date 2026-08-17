use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Pos2, Sense, Stroke, Ui, vec2};

/// Struct that stores pendulum elements.
pub struct PendulumDraw {
    _masse: Pos2,
}

impl Default for PendulumDraw {
    fn default() -> Self {
        Self {
            _masse: Pos2 { x: 0.5, y: 1.5 },
        }
    }
}

impl PendulumDraw {
    pub fn ui_pendulum(
        &mut self,
        ui: &mut Ui,
        _scale: f32,
        pause: bool,
        stick_length: f32,
        alpha: f32,
    ) -> egui::Response {
        // dimensions
        let canvas_width = ui.available_width();
        let canvas_height = ui.available_height();

        let canvas_size = vec2(canvas_width, canvas_height);
        let (response, painter) = ui.allocate_painter(canvas_size, Sense::hover());
        let rect = response.rect;
        let c = rect.center();

        // style
        let red = Color32::from_rgb(255, 0, 0);
        let green = Color32::from_rgb(0, 255, 0);
        let white = Color32::from_rgb(255, 255, 255);

        let stroke = Stroke::new(2.0, white);

        let masse_coord = c + stick_length * vec2(alpha.sin(), alpha.cos());

        // cross
        painter.line_segment([c - vec2(0.0, 25.0), c + vec2(0.0, 25.0)], stroke);
        painter.line_segment([c - vec2(25.0, 0.0), c + vec2(25.0, 0.0)], stroke);

        // stick
        painter.line_segment([c, masse_coord], stroke);

        painter.circle(masse_coord, 10.0, red, Stroke::new(2.0, red));
        painter.circle(c, 2.0, green, Stroke::new(2.0, green));

        let text_pos = rect.min + vec2(10.0, 10.0);
        let text: &str;
        if pause {
            text = "Pendulum - Simulation (paused)";
        } else {
            text = "Pendulum - Simulation";
        }

        painter.text(
            text_pos,
            Align2::LEFT_TOP,
            text,
            FontId::proportional(14.0),
            Color32::WHITE,
        );

        response
    }
}
