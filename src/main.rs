use eframe::egui;
use std::time::Instant;

mod paint;
mod pendulum;
mod pid;

/// This struct holds the data (state) for our application.
pub struct MyApp {
    pid: pid::PIDCfg,
    pendulum: pendulum::PendulumState,
    pendulum_draw: paint::PendulumDraw,

    last_update: Instant,
    adapative_dt: bool,

    pause: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pid: pid::PIDCfg::default(),
            pendulum: pendulum::PendulumState::default(),
            pendulum_draw: paint::PendulumDraw::default(),
            last_update: Instant::now(),
            adapative_dt: true,
            pause: true,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let frame_dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.adapative_dt && (frame_dt > 0.0 && frame_dt < 0.1) {
            self.pendulum.dt = frame_dt;
        }

        if !self.pause {
            self.pendulum.next();
        }

        egui::Panel::left("simulation_cfg").show(ui, |ui| {
            ui.heading("Simulation parameters");

            ui.checkbox(&mut self.adapative_dt, "Adapative dt")
                .on_hover_text("Forces dt ro correpond to FPS window.");

            ui.add(egui::Slider::new(&mut self.pendulum.dt, 0.001..=1.0).text("dt"));
            ui.horizontal(|ui| {
                ui.label("Angle: ");
                ui.add(egui::DragValue::new(&mut self.pendulum.alpha).speed(0.1));
            });

            ui.add(egui::Slider::new(&mut self.pendulum.masse, 0.001..=10.0).text("kg"));
            ui.add(egui::Slider::new(&mut self.pendulum.length, 0.001..=2.0).text("length"));
            ui.add(egui::Slider::new(&mut self.pendulum.friction, 0.00001..=5.0).text("friction"));
            if ui.button("Reset").clicked() {
                self.pendulum = pendulum::PendulumState::default();
            }

            ui.separator();

            ui.heading("PID parameters");

            ui.add(egui::Slider::new(&mut self.pid.kp, 0.0..=1.0).text("Kp"));
            ui.add(egui::Slider::new(&mut self.pid.ki, 0.0..=1.0).text("Ki"));
            ui.add(egui::Slider::new(&mut self.pid.kd, 0.0..=1.0).text("Kd"));

            if ui.button("Reset").clicked() {
                self.pid = pid::PIDCfg::default();
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Pause").clicked() {
                    self.pause = !self.pause;
                }

                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            })
        });

        egui::CentralPanel::default().show(ui, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());

                self.pendulum_draw.ui_pendulum(
                    ui,
                    1.0,
                    self.pause,
                    200.0 * self.pendulum.length,
                    self.pendulum.alpha,
                );
            });
        });

        // update pendulum
        ui.ctx().request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "PID demonstrator",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
