use eframe::egui; // Import necessary parts of eframe and egui

mod paint;
mod pendulum;
mod pid;

/// This struct holds the data (state) for our application.
#[derive(Default)]
pub struct MyApp {
    pid: pid::PIDCfg,
    pendulum: pendulum::PendulumState,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("simulation_cfg").show(ui, |ui| {
            ui.heading("Simulation parameters");

            ui.add(egui::Slider::new(&mut self.pendulum.dt, 0.0..=1.0).text("dt"));
            ui.add(egui::Slider::new(&mut self.pendulum.masse, 0.0..=10.0).text("kg"));

            ui.horizontal(|ui| {
                ui.label("Angle: ");
                ui.add(egui::DragValue::new(&mut self.pendulum.alpha).speed(1));
            });

            ui.add(egui::Slider::new(&mut self.pendulum.length, 0.0..=2.0).text("length"));
            ui.add(egui::Slider::new(&mut self.pendulum.friction, 0.0..=1.0).text("friction"));
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
                    todo!()
                }
                if ui.button("Resume").clicked() {
                    todo!()
                }
                if ui.button("Reset").clicked() {
                    todo!()
                }
            })
        });

        egui::CentralPanel::default().show(ui, |ui| ui.add(egui::Label::new("TODO")));
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
