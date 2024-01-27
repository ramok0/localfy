use egui::{vec2, Image, Layout, Rect, ScrollArea, Widget};
use crate::{app::App, gui::song::draw_song};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        ui.label("Home Page");
        ScrollArea::new([false, true]).show(&mut ui, |ui| {
            let height = ui.available_height();
            
            let playlist = {
                self.app.database().songs().get_songs().clone()
            };

            playlist.iter().for_each(|song| {
                draw_song(&self, ui, song,height );
            });
        });
    }
}