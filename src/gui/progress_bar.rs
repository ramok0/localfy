use std::hash::Hash;

use egui::{Rect, Rounding, Color32, Stroke, Vec2, Sense};



pub fn progress_bar(ui:&mut egui::Ui, percent:f32, size:Vec2) -> egui::Response {


    let (rect, mut response) = ui.allocate_exact_size(size, Sense::hover());

    if ui.is_rect_visible(rect) {
        ui.painter().rect(rect, Rounding::same(5.), Color32::WHITE, Stroke::NONE);
        let bar_rect = rect.clone().with_max_x(rect.min.x + rect.width() * percent / 100.0).expand(-1.0);

        let green = 255.0/100.*percent;
        let red = 255.0 - green;
        let color = Color32::from_rgb(red as u8, green as u8, 0);
        
        ui.painter().rect_filled(bar_rect, Rounding::same(5.), color);
    }

    ui.ctx().request_repaint(); //fixes fluidity problems i guess

    response
}