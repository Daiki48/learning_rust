#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

#[derive(Default)]
struct MyApp {
    count: i32
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is header");
            ui.horizontal(|ui| {
                ui.label("This is label");
            });
            if ui.button("+").clicked() {
                self.count += 1;
            }
            if ui.button("-").clicked() {
                self.count -= 1;
            }
            ui.label(format!("count: {}", &self.count));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Minimal",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
