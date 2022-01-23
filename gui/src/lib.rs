use eframe::egui::{Separator, Ui};
use eframe::epi::Image;
use eframe::{egui, epi};

const PADDING: f32 = 5.0;

pub fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("RDR2Converter");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

pub fn decode_image(buffer: &[u8]) -> Option<epi::Image> {
    use image::GenericImageView;
    let image = image::load_from_memory(buffer).ok()?;
    let image_buffer = image.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image_buffer.into_vec();
    Some(epi::Image::from_rgba_unmultiplied(size, &pixels))
}

pub fn render_image(
    ui: &mut egui::Ui,
    frame: &epi::Frame,
    tex_mngr: &mut TextureManager,
    image: Option<epi::Image>,
) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            if let Some(image) = image {
                if let Some(texture_id) = tex_mngr.texture(frame, &image) {
                    let mut size = egui::Vec2::new(image.size[0] as f32, image.size[1] as f32);
                    size *= (ui.available_width() / size.x).min(1.0);
                    ui.image(texture_id, size);
                } else {
                    ui.monospace("ERROR");
                }
            }
        })
}

#[derive(Default)]
pub struct TextureManager {
    texture_id: Option<egui::TextureId>,
}

impl TextureManager {
    pub fn texture(&mut self, frame: &epi::Frame, image: &Image) -> Option<egui::TextureId> {
        if let Some(texture_id) = self.texture_id.take() {
            frame.free_texture(texture_id);
        }
        self.texture_id = Some(frame.alloc_texture(image.clone()));
        self.texture_id
    }
}
