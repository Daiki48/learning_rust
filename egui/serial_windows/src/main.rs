#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Close").clicked() {
                eprintln!("Pressed Close button");
                frame.close();
            }
        });
    }
}

fn main() {
    if cfg!(target_os = "macos") {
        eprintln!("WARNING: This example does not work on Mac!");
    }

    let options = eframe::NativeOptions {
        run_and_return: true,
        ..Default::default()
    };

    eprintln!("Starting first window");
    eframe::run_native(
        "First window",
        options.clone(),
        Box::new(|_cc| Box::new(MyApp::default())),
    );

    std::thread::sleep(std::time::Duration::from_secs(2));

    eprintln!("Starting second window");
    eframe::run_native(
        "Second window",
        options.clone(),
        Box::new(|_cc| Box::new(MyApp::default())),
    );

    std::thread::sleep(std::time::Duration::from_secs(2));

    eprintln!("Starting third window");
    eframe::run_native(
        "Third window",
        options.clone(),
        Box::new(|_cc| Box::new(MyApp::default())),
    );



}
