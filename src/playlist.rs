use std::{path::PathBuf, sync::Arc};

use egui::{pos2, Align2, Color32, FontFamily, FontId, ImageSource, Widget};

use crate::{app::{self, App, AppImpl}, constants::{BACKGROUND_COLOR, TEXT_COLOR}, gui::{model::Pages, page::RenderablePage, song::{SongWidget}}, song::Song};



#[derive(Clone, Debug, serde::Serialize, Hash, serde::Deserialize)]
pub struct PlaylistDescriptor {
    pub id:String,
    pub name:String,
    pub image:Option<PathBuf>,
}

impl PlaylistDescriptor {
    pub fn hash_songs(&self, app:Arc<AppImpl>) -> Vec<u64> {
        {
            app.database().raw().data.lock().unwrap().playlists.iter().find(|x| x.name == self.name).unwrap_or(&Playlist { id: self.id.clone(), name: self.name.clone(), image: self.image.clone(), songs: vec![] }).songs.clone()
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, Hash, serde::Deserialize)]
pub struct Playlist {
    pub id:String,
    pub name:String,
    pub image:Option<PathBuf>,
    pub(crate) songs:Vec<u64> // Vec to hashes of songs to gain performance and reduce database size
}

impl From<Playlist> for PlaylistDescriptor {
    fn from(playlist:Playlist) -> Self {
        Self {
            id:playlist.id,
            name:playlist.name,
            image:playlist.image
        }
    }
}

#[derive(Clone)]
pub struct DecodedPlaylist {
    pub descriptor:PlaylistDescriptor,
    pub songs:Vec<crate::song::Song>
}

impl RenderablePage for PlaylistDescriptor {
    fn get_page_title(&self) -> String {
        self.name.clone()
    }

    fn render(&self, application:&mut App, ui:&mut egui::Ui, max_rect:egui::Rect) {

        if ui.button("go back").clicked() {
            application.gui_settings.location = crate::gui::model::UserLocation::Home;
            application.gui_settings.page = Pages::Home;
        }

        let list_rect = max_rect.expand2(egui::vec2(0., -50.)).shrink(35.);
        let paint_rect = list_rect.expand(10.0);


        ui.painter().rect_filled(paint_rect, 10.0, BACKGROUND_COLOR);

        let font = FontId::new(17., FontFamily::Name("Montserrat".into()));
        ui.painter().text(pos2(paint_rect.min.x, paint_rect.min.y - 35.0), Align2::LEFT_CENTER, &self.name, font, TEXT_COLOR);
        ui.painter().text(pos2(paint_rect.min.x, paint_rect.min.y - 10.0), Align2::LEFT_CENTER, "Playlist - WorkInProgress", FontId::proportional(14.), Color32::RED);

        //create padding
        let mut container = ui.child_ui(list_rect, egui::Layout::default());

        let songs = {
            application.app.database().playlists().unhash_playlist_songs(self)
        };

        if let Some(resolved_playlist) = songs {
            egui::ScrollArea::new([false, true]).show(&mut container, |download_ui: &mut egui::Ui| {
            
                resolved_playlist.songs.iter().for_each(|song| {
                    let response = download_ui.add(SongWidget::new(song.clone()));

                    if response.clicked() {
                        song.on_clicked(application.app.clone(), crate::gui::model::UserLocation::Playlist(resolved_playlist.descriptor.clone()));
                    }

                    song.context_menu(response, ui, application);
                });
            });
        } else {
            container.label("Failed to resolve playlist");
        }


    }

    fn get_page_icon(&self) -> ImageSource {
        todo!()
    }
}