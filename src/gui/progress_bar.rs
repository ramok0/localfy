use std::hash::Hash;

use egui::{Rect, Rounding, Color32, Stroke, Vec2, Sense};



pub fn progress_bar(ui:&mut egui::Ui, percent:f32, size:Vec2) -> egui::Response {


    let (rect, mut response) = ui.allocate_exact_size(size, Sense::hover());

    if ui.is_rect_visible(rect) {
        ui.painter().rect(rect, Rounding::same(5.), Color32::WHITE, Stroke::new(1., Color32::BLACK));
        let bar_rect = rect.clone().with_max_x(rect.min.x + rect.width() * percent);

        // let green = 255.0/100.*percent;
        // let red = 255.0 - green;
        // let color = Color32::from_rgb(red as u8, green as u8, 0);

        let color = blend_colors(Color32::from_rgb(0x8E, 0x08, 0x08), Color32::from_rgb(0x1b, 0x6f, 0x06), percent);
        
        ui.painter().rect_filled(bar_rect, Rounding::same(5.), color);
    }

    ui.ctx().request_repaint(); //fixes fluidity problems i guess

    response
}

fn blend_colors(color1: Color32, color2: Color32, percent: f32) -> Color32 {
    let red = (color1.r() as f32 * (1.0 - percent) + color2.r() as f32 * percent) as u8;
    let green = (color1.g() as f32 * (1.0 - percent) + color2.g() as f32 * percent) as u8;
    let blue = (color1.b() as f32 * (1.0 - percent) + color2.b() as f32 * percent) as u8;
    Color32::from_rgb(red, green, blue)
}


