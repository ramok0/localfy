use egui::{Layout, Rect, ScrollArea};
use crate::{app::App, gui::song::draw_song};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        match &self.gui_settings.location {
            crate::gui::model::UserLocation::Home => {
                
            },
            crate::gui::model::UserLocation::Playlist(page) => {
                page.render(&mut ui, max_rect);
            },
            crate::gui::model::UserLocation::Artist(_) => todo!(),
            crate::gui::model::UserLocation::Album(_) => todo!(),
        }
    }
}