#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 900.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    image: RetainedImage,
    tint: egui::Color32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "rust-logo-256x256.png",
                include_bytes!("rust-logo-256x256.png"),
            )
            .unwrap(),
            tint: egui::Color32::from_rgb(255, 0, 255),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            self.image.show(ui);

            ui.heading("This is a rotated image with a tint:");
            ui.add(
                egui::Image::new(self.image.texture_id(ctx), self.image.size_vec2())
                    .rotate(45.0_f32.to_radians(), egui::Vec2::splat(0.5))
                    .tint(self.tint),
            );

            ui.horizontal(|ui| {
                ui.label("Tint:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut self.tint,
                    egui::color_picker::Alpha::BlendOrAdditive,
                );
            });

            ui.heading("This is an image you can click:");
            ui.add(egui::ImageButton::new(
                self.image.texture_id(ctx),
                self.image.size_vec2(),
            ));
        });
    }
}
