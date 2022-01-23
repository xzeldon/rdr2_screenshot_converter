#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::buffer;
use eframe::{
    egui::{CentralPanel, Vec2},
    epi::App,
    run_native, NativeOptions,
};
use gui::TextureManager;
use std::path::PathBuf;

#[derive(Default)]
struct RDR2Converter {
    pub tex_mngr: TextureManager,
    picked_path: Option<PathBuf>,
}

impl App for RDR2Converter {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            gui::render_header(ui);

            if ui.button("Open file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path);
                }
            }

            // TODO: use threads and channels
            if let Some(picked_path) = &self.picked_path {
                let buf = buffer::read_file(&picked_path).unwrap();
                let pic = buffer::parse_buf(&buf, b"JPEG", b"JSON", 12);
                let image = gui::decode_image(&pic);
                gui::render_image(ui, frame, &mut self.tex_mngr, image);
            }
        });
    }

    fn name(&self) -> &str {
        "RDR2Converter"
    }
}

fn main() {
    let app = RDR2Converter::default();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_option);
}
