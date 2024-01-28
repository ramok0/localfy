use egui::{Layout, Rect, ScrollArea};
use crate::{app::App, gui::{page::RenderablePage, song::draw_song}, playlist::{DecodedPlaylist, Playlist}};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());



        match &self.gui_settings.location {
            crate::gui::model::UserLocation::Home => {
                if ui.button("playlist test").clicked() {
                    self.gui_settings.location = crate::gui::model::UserLocation::Playlist(DecodedPlaylist {
                        playlist: Playlist {
                            name: "placeholder playlist name".to_string(),
                            image: None,
                            songs: vec![],
                        },
                        songs: self.app.database().songs().get_songs().clone().into_iter().take(5).collect(),
                    });
                }
            },
            crate::gui::model::UserLocation::Playlist(page) => {
                page.render(self.app.clone(), &mut ui, max_rect);
            },
            crate::gui::model::UserLocation::Artist(_, _) => todo!(),
            crate::gui::model::UserLocation::Album(_, _) => todo!(),
        }
    }
}