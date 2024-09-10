#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Chronor")
            .with_min_inner_size([330.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Chronor",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn to_percent(percent: f32, from: f32) -> f32{
    from * percent / 100.0
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let current_height = ctx.input(|i: &egui::InputState| i.screen_rect().max[1]);
        let current_width = ctx.input(|i: &egui::InputState| i.screen_rect().max[0]);

        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {});
                ui.menu_button("Edit", |ui| {});
                ui.menu_button("Window", |ui| {});
            });
        });
        egui::SidePanel::left("left_panel")
            .min_width(180.0)
            .max_width(to_percent(20.0, current_width))
            .show(ctx, |ui| {
            });
        egui::CentralPanel::default().show(ctx, |ui|{

        });
    }
}
