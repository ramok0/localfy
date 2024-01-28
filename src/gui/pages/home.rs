use std::time::{Instant, SystemTime, UNIX_EPOCH};

use egui::{vec2, Align, Color32, Id, Label, Layout, Rect, RichText, Rounding, ScrollArea, Widget, Window};
use crate::{app::App, constants::BACKGROUND_COLOR, gui::{add_button_animated::AddButtonAnimated, helper, model::{Pages, UserLocation}, page::RenderablePage, song::{SongWidget}}, playlist::{DecodedPlaylist, Playlist}, renderer::Drawable, song::Song};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::top_down(Align::LEFT));
        let new_playlist = Id::new("_new_playlist");
        let add_song_to_playlist = Id::new("_add_song_to_playlist");

        if ui.memory(|mem| mem.data.get_temp(add_song_to_playlist).unwrap_or(false)) == true {
            Window::new("Add song to playlist").show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Search").strong());
                    ui.text_edit_singleline(&mut self.gui_settings.song_name_to_add);
                    if ui.button("Add Songs").clicked() {
                        if let UserLocation::Playlist(playlist) = &self.gui_settings.location {
                            self.app.database().playlists().push_to_playlist(&playlist, &self.gui_settings.add_songs);
                            self.gui_settings.add_songs.clear();
                        }
        
                        ui.memory_mut(|mem| mem.data.remove::<bool>(add_song_to_playlist));
                    }

                    if ui.button("Close").clicked() {
                        ui.memory_mut(|mem| mem.data.remove::<bool>(add_song_to_playlist));
                    }
                });


                let library = {
                    self.app.database().songs().get_songs()
                };

                let result = library.into_iter().filter(|item| {
                    if self.gui_settings.song_name_to_add.is_empty() {
                        return true;
                    }

                    if helper::simplify(&item.get_title()).contains(&self.gui_settings.song_name_to_add.to_lowercase()) {
                        return true;
                    }

                    if helper::simplify(&item.get_artist()).to_lowercase().contains(&self.gui_settings.song_name_to_add.to_lowercase()) {
                        return true;
                    }

                    if item.get_album().and_then(|album| Some(helper::simplify(&album.title).contains(&self.gui_settings.song_name_to_add.to_lowercase()))).unwrap_or(false) {
                        return true;
                    }

                    return false;
                }).collect::<Vec<Song>>();



                ScrollArea::new([false, true]).show(ui, |ui| {
                    result.iter().for_each(|song| {
                        let widget = if self.gui_settings.add_songs.contains(&song) {
                            SongWidget::new(song.clone()).background_color(Color32::RED)
                        } else {
                            SongWidget::new(song.clone())
                        };

                        let response = widget.ui(ui);

                        if response.clicked() {
                            if self.gui_settings.add_songs.contains(&song) {
                                if let Some(position) = self.gui_settings.add_songs.iter().position(|x| x == song) {
                                    self.gui_settings.add_songs.remove(position);
                                }
                            } else {
                                self.gui_settings.add_songs.push(song.clone());
             
                            }
                        }


                    });
                });
            });    
        }

        if ui.memory(|mem| mem.data.get_temp(new_playlist).unwrap_or(false)) == true {
            Window::new("New playlist").show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Name").strong());
                    ui.text_edit_singleline(&mut self.gui_settings.new_playlist_name);
                });
                ui.horizontal(|ui| {
                    if !self.gui_settings.new_playlist_name.is_empty() && ui.button("Create").clicked() {
                        self.app.database().playlists().add_playlist(&Playlist {
                            id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                            name: self.gui_settings.new_playlist_name.clone(),
                            image: None, //TODO
                            songs: vec![], //TODO
                        });
                        
                        ui.memory_mut(|mem| mem.data.remove::<bool>(new_playlist));
                    };

                    if ui.button("Cancel").clicked() {
                        ui.memory_mut(|mem| mem.data.remove::<bool>(new_playlist));
                    }
                });
            });
        }
        
        
        let add_pos = ui.max_rect().max - vec2(60., 3.0);
        let add_rect = Rect::from_center_size(add_pos, vec2(40., 40.));


        match self.gui_settings.location.clone() {
            crate::gui::model::UserLocation::Home => {

                let playlists = {
                    self.app.database().playlists().get_playlists()
                };

                ui.label(RichText::new("Playlists").strong());
                ScrollArea::new([false, true]).id_source("_playlists_").show(&mut ui, |ui| {
                    playlists.into_iter().for_each(|playlist| {
                        if ui.button(playlist.name.clone()).clicked() {
                            self.gui_settings.location = UserLocation::Playlist(
                                playlist
                            );
                            
                            self.gui_settings.page = Pages::Playlist;
                        }
                    });
                });

                let albums = {
                    self.app.database().albums().get_albums(self.app.clone())
                };

                ui.label(RichText::new("Albums").strong());
                ScrollArea::new([false, true]).id_source("_albums_").show(&mut ui, |ui| {
                    albums.into_iter().for_each(|album_with_tracks| {
                        let response = ui.button(album_with_tracks.album.get_title().clone());

                        if response.clicked() {
                            self.gui_settings.location = UserLocation::Album(album_with_tracks.album);
                            
                            self.gui_settings.page = Pages::Album;
                        }

                        response.context_menu(|ui| {
                            ui.label("Dummy context menu - album"); 
                        });
                    });
                });

                // if ui.button("playlist test").clicked() {
                //     self.gui_settings.location = crate::gui::model::UserLocation::Playlist(DecodedPlaylist {
                //         playlist: Playlist {
                //             name: "placeholder playlist name".to_string(),
                //             image: None,
                //             songs: vec![],
                //         },
                //         songs: self.app.database().songs().get_songs().clone().into_iter().take(5).collect(),
                //     });
                // }
            },
            crate::gui::model::UserLocation::Playlist(playlist_descriptor) => {
                playlist_descriptor.render(self, &mut ui, max_rect);
            },
            crate::gui::model::UserLocation::Artist( _) => todo!(),
            crate::gui::model::UserLocation::Album(album) => {
                album.render(self, &mut ui, max_rect);
            },
        }

        let button_id = Id::new("__add_button");
        let popup_id = ui.make_persistent_id("my_unique_id");
        let response = ui.put(add_rect, AddButtonAnimated::new(button_id).line_width(11.5));
        if response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }
        
            egui::containers::popup_above_or_below_widget(&ui, popup_id, &response, egui::AboveOrBelow::Above, |ui| {
                ui.set_min_height(50.0); 
                ui.set_min_width(100.0); 
                if ui.button("New playlist").clicked() {
                    ui.memory_mut(|mem| mem.data.insert_temp(new_playlist, true));
                }

                if self.gui_settings.page == Pages::Playlist {
                    if ui.button("Add song to playlist").clicked() {
                        ui.memory_mut(|mem| mem.data.insert_temp(add_song_to_playlist, true));
                    }
                }

                ui.button("New nique ta grand mere");
                
            });
        

    }
}