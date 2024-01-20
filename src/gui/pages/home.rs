use egui::{Rect, Layout, ScrollArea};
use crate::{app::App, gui::song::draw_song};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        ui.label("Home Page");

        ScrollArea::new([false, true]).show(&mut ui, |ui| {
            let height = ui.available_height();
            
            let playlist = {
                self.app.player.queue().get_playlist().clone()
            };

            playlist.iter().for_each(|song| {
                draw_song(&self, ui, song,height );
            });
        });
    }
}