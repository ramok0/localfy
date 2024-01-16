use egui::{Rect, Pos2, Rounding, Vec2, include_image, Image, Sense};
use crate::{app::{App}, constants::BACKGROUND_COLOR, gui::model::Pages};

impl App {
    pub fn draw_navbar(&mut self, ui: &mut egui::Ui, max_rect_mut: &mut Rect, panel_rect:&mut Rect) {
        let max_rect = *max_rect_mut;

        let height = max_rect.max.y - max_rect.min.y;
        let width = max_rect.max.x - max_rect.min.x;

        let min_panel_center_x = max_rect.min.x + width/2.0;
        let min_panel_y = max_rect.min.y + height*0.05;

        const PANEL_WIDTH:f32 = 65.0;
        const PANEL_HEIGHT:f32 = 160.0;

        let min_panel:Pos2 = Pos2 { x: min_panel_center_x - PANEL_WIDTH/2.0, y: min_panel_y };
        let max_panel:Pos2 = min_panel + Vec2 { x: PANEL_WIDTH, y: min_panel.y + PANEL_HEIGHT };

        panel_rect.min = min_panel;
        panel_rect.max = max_panel;

        ui.painter().rect_filled(*panel_rect, Rounding::same(5.), BACKGROUND_COLOR);
        let home_min_y = (max_panel.y - min_panel.y)*0.18;
        let home_min = Pos2 { x: min_panel_center_x, y: min_panel.y + home_min_y };
        let home_rect = Rect { min: home_min - Vec2 {x:12.5, y:12.5}, max: home_min + Vec2 {x:12.5, y:12.5} };


        let search_min_y = (max_panel.y + min_panel.y) / 2.0;
        let search_min = Pos2 { x: min_panel_center_x, y: search_min_y };
        let search_rect = Rect { min: search_min -  Vec2 {x:12.5, y:12.5} , max: search_min +  Vec2 {x:12.5, y:12.5}  };

        let settings_min_y: f32 = (max_panel.y - min_panel.y)*0.82;
        let settings_min = Pos2 { x: min_panel_center_x, y: min_panel.y + settings_min_y };
        let settings_rect = Rect { min: settings_min - Vec2 {x:12.5, y:12.5}, max: settings_min + Vec2 {x:12.5, y:12.5} };

        if ui.put(home_rect, Image::new(include_image!("../../../assets/home.svg")).sense(Sense::click()).fit_to_exact_size(Vec2 {x:home_rect.width(), y:home_rect.height()})).clicked() {
            self.gui_settings.page = Pages::Home;
        }

        if ui.put(search_rect, Image::new(include_image!("../../../assets/search.svg")).sense(Sense::click()).fit_to_exact_size(Vec2 {x:search_rect.width(), y:search_rect.height()})).clicked() {
            self.gui_settings.page = Pages::Search;
        }

        if ui.put(settings_rect, Image::new(include_image!("../../../assets/settings.svg")).sense(Sense::click()).fit_to_exact_size(Vec2 {x:settings_rect.width(), y:settings_rect.height()})).clicked() {
            self.gui_settings.page = Pages::Settings;
        }

    //    ui.painter().rect_filled(settings_rect, Rounding::ZERO, Color32::RED);
    }
}