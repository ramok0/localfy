use std::{path::PathBuf, sync::Arc};

use egui::{pos2, Align2, Color32, FontFamily, FontId, ImageSource, Widget};

use crate::{app::AppImpl, constants::{BACKGROUND_COLOR, TEXT_COLOR}, gui::{page::RenderablePage, song::{draw_song, SongWidget}}, song::Song};



#[derive(Clone, Debug, serde::Serialize, Hash, serde::Deserialize)]
pub struct Playlist {
    pub name:String,
    pub image:Option<PathBuf>,
    pub songs:Vec<u64> // Vec to hashes of songs to gain performance and reduce database size
}

#[derive(Clone)]
pub struct DecodedPlaylist {
    pub playlist:Playlist,
    pub songs:Vec<crate::song::Song>
}

impl RenderablePage for DecodedPlaylist {
    fn get_title(&self) -> String {
        self.playlist.name.clone()
    }

    fn render(&self, app:Arc<AppImpl>, ui:&mut egui::Ui, max_rect:egui::Rect) {
        let list_rect = max_rect.expand2(egui::vec2(0., -50.)).shrink(35.);
        let paint_rect = list_rect.expand(10.0);


        ui.painter().rect_filled(paint_rect, 10.0, BACKGROUND_COLOR);

        let font = FontId::new(17., FontFamily::Name("Montserrat".into()));
        ui.painter().text(pos2(paint_rect.min.x, paint_rect.min.y - 35.0), Align2::LEFT_CENTER, &self.playlist.name, font, TEXT_COLOR);
        ui.painter().text(pos2(paint_rect.min.x, paint_rect.min.y - 10.0), Align2::LEFT_CENTER, "Playlist - WorkInProgress", FontId::proportional(14.), Color32::RED);

        //create padding
        let mut download_ui = ui.child_ui(list_rect, egui::Layout::default());

        egui::ScrollArea::new([false, true]).show(&mut download_ui, |download_ui: &mut egui::Ui| {
            self.songs.iter().for_each(|song| {
                if download_ui.add(SongWidget::new(song.clone())).clicked() {
                    song.on_clicked(app.clone(), crate::gui::model::UserLocation::Playlist(self.clone()));
                }
            });
        });
    }

    fn get_icon(&self) -> ImageSource {
        todo!()
    }
}