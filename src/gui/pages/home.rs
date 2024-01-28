use egui::{vec2, Color32, Layout, Rect, Rounding, ScrollArea};
use crate::{app::App, constants::BACKGROUND_COLOR, gui::{helper, page::RenderablePage, song::draw_song}, playlist::{DecodedPlaylist, Playlist}};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());
        
        let add_pos = ui.max_rect().max - vec2(40., 0.0);
        let add_rect = Rect::from_center_size(add_pos, vec2(40., 40.));

        ui.painter().rect_filled(add_rect, Rounding::same(30.), BACKGROUND_COLOR);

        //dessiner un + avec des lignes
        ui.painter().line_segment([add_pos + vec2(-10., 0.), add_pos + vec2(10., 0.)], (2.0, Color32::WHITE));
        ui.painter().line_segment([add_pos + vec2(0., -10.), add_pos + vec2(0., 10.)], (2.0, Color32::WHITE));

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