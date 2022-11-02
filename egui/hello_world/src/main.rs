// このように書いてexeファイルを実行すると黒いコマンドウィンドウが立ち上がる
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// このように書くとコマンドウィンドウは立ち上がらない
#![windows_subsystem = "windows"]

use eframe::egui;

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 27,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Egui Sample App");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello {}, age {}", self.name, self.age));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui sample",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
