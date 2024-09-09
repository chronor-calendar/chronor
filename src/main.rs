#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Pixliz", native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))))
}

#[derive(Default)]
struct MyApp{}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self{
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            
        });
    }
}
