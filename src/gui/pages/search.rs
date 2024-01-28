
use egui::{vec2, Color32, Image, Layout, Rect, Rounding, ScrollArea, Spinner};
use tidal_rs::model::{SearchResult, SearchType};
use crate::{app::App, gui::model::Event, renderer::Drawable, song::Song};
use tokio::task;

impl App {
    pub fn draw_search_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        let inner = ui.horizontal(|ui| {
            ui.label("Search for a song (downloader) : ");
            ui.text_edit_singleline(&mut self.gui_settings.search_query);

            if ui.button("Clear").clicked() {
                self.gui_settings.search_query.clear();
                self.gui_settings.search_results = SearchResult::new();
            }

            ui.button("Search")
        });

        if inner.inner.clicked() {
            let tidal_client = self.app.tidal_client.clone();
            let search_query = self.gui_settings.search_query.clone();
            let tx = self.gui_settings.event_manager.0.clone();
            self.gui_settings.is_searching = true;
            self.gui_settings.search_results.clear();
            tokio::spawn(async move {

                if let Ok(id) = search_query.parse::<usize>() {
                    if let Ok(track) = tidal_client.media().get_track(id).await {
                        let _ = tx.send(Event::SearchResult(SearchResult {
                            tracks: vec![track],
                            ..Default::default()
                        })).await;
                        return;
                    }

                    if let Ok(album) = tidal_client.media().get_album(id).await {
                        let _ = tx.send(Event::SearchResult(SearchResult {
                            albums: vec![album],
                            ..Default::default()
                        })).await;
                        return;
                    }
                }

                let result: Result<SearchResult, tidal_rs::error::Error> = tidal_client
                    .search()
                    .all(&search_query, Some(20)).await;
                if let Ok(search_result) = result {
                    let _ = tx.send(Event::SearchResult(search_result)).await;
                }
            });
        }

        if self.gui_settings.is_searching {
            ui.centered_and_justified(|ui| {
                ui.add(Spinner::new().size(ui.available_width()/10.0));
            });
        }

        if !self.gui_settings.search_results.is_empty() {
            ui.horizontal(|ui| {
                SearchType::search_types()
                    .iter()
                    .for_each(|x| {
                        ui.radio_value(
                            &mut self.gui_settings.search_type,
                            *x,
                            x.to_string()
                        );
                    });
            });

            ScrollArea::new([false, true]).show(&mut ui, |ui| {


                let items:Vec<Box<dyn Drawable + Send + Sync>> = match self.gui_settings.search_type {
                    SearchType::Artist => {
                        self.gui_settings.search_results.artists.clone().into_iter().map(|x| Box::new(x) as Box<dyn Drawable + Send + Sync>).collect()
                    },
                    SearchType::Track => {
                        self.gui_settings.search_results.tracks.clone().into_iter().map(|x| Box::new(x) as Box<dyn Drawable+ Send + Sync>).collect()
                    },
                    SearchType::Album => {
                        self.gui_settings.search_results.albums.clone().into_iter().map(|x| Box::new(x) as Box<dyn Drawable+ Send + Sync>).collect()
                    },
                    SearchType::Playlist => {
                        vec![]
                    }
                };

                items.into_iter().for_each(|item| {
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            Image::new(item.get_texture()).fit_to_exact_size(
                                vec2(35., 35.)
                            )
                        );

                        if self.gui_settings.search_type == SearchType::Track {
                            let track = item.get_track().unwrap();

                            if let Some(song) = Song::resolve(self.app.clone(), &track) {
                                song.context_menu(response, ui, &self);
                            }   
                        }

                        ui.label(item.get_title());
                        if ui.button("Download").clicked() {
                            let app = self.app.clone();
          

                            match self.gui_settings.search_type {
                                SearchType::Artist => {
                                    tokio::spawn(async move {
                                        let quality = app.get_quality_or_highest_avaliable();
        
                                        let mut albums = app.tidal_client.media().get_artist_albums(item.id(), None).await.unwrap_or(vec![]);
                                        let singles = app.tidal_client.media().get_artist_singles(item.id(), None).await.unwrap_or(vec![]);
        
                                        albums.extend(singles.into_iter());
                                        
                                        albums.iter().for_each(|album| {
                                            let app = app.clone();
                                            let album = album.clone();
                                            task::spawn(async move {
                                                let _ = app.download_manager.enqueue_album(app.clone(), album, quality).await;
                                            });
                                        });
                    
                                    });
                                },
                                SearchType::Track => {
                                    tokio::spawn(async move {
                                        let quality = app.get_quality_or_highest_avaliable();
                                        let _ = app.download_manager.enqueue_single(app.clone(), quality, item.get_track().unwrap(), None).await;
                                    });
                                },
                                SearchType::Album => {
                                    tokio::spawn(async move {
                                        let quality = app.get_quality_or_highest_avaliable();
                                        let _ = app.download_manager.enqueue_album(app.clone(), item.get_album().unwrap(), quality).await;
                                    });
                                },
                                SearchType::Playlist => todo!(),
                            }

              
                        }
                    });
                });

                // match self.gui_settings.search_type {
                //     SearchType::Artist => {
                //         self.gui_settings.search_results.artists.iter().for_each(|artist| {
                //             ui.horizontal(|ui| {
                //                 ui.add(
                //                     Image::new(artist.get_texture()).fit_to_exact_size(
                //                         vec2(35., 35.)
                //                     )
                //                 );
                //                 ui.label(format!("{}", artist.get_title()));
                //                 if ui.button("Download").clicked() {
                //                     let app = self.app.clone();
                //                     let drawable_artist = artist.clone();

                //                     tokio::spawn(async move {
                //                         let quality = app.get_quality_or_highest_avaliable();

                //                         let mut albums = app.tidal_client.media().get_artist_albums(drawable_artist.id, None).await.unwrap_or(vec![]);
                //                         let singles = app.tidal_client.media().get_artist_singles(drawable_artist.id, None).await.unwrap_or(vec![]);

                //                         albums.extend(singles.into_iter());
                                        
                //                         albums.iter().for_each(|album| {
                //                             let app = app.clone();
                //                             let album = album.clone();
                //                             task::spawn(async move {
                //                                 let _ = app.download_manager.enqueue_album(app.clone(), album, quality).await;
                //                             });
                //                         });
                    
                //                     });
                //                 }
                //             });
                //         });
                //     }
                //     SearchType::Track => {
                //         self.gui_settings.search_results.tracks.iter().for_each(|track| {
                //             ui.horizontal(|ui| {
                //                 ui.add(
                //                     Image::new(track.get_texture()).fit_to_exact_size(vec2(35., 35.))
                //                 );
                //                 ui.label(track.get_title());
                //                 if ui.button("Download").clicked() {
                //                     let app = self.app.clone();
                //                     let track = track.clone();
                //                     tokio::spawn(async move {
                //                         let quality = app.get_quality_or_highest_avaliable();

                //                         let _ = app.download_manager.enqueue_single(app.clone(), quality, track).await;
                //                     });
                //                 }
                //             });
                //         });
                //     }
                //     SearchType::Album => {
                //         self.gui_settings.search_results.albums.iter().for_each(|album| {
                //             ui.horizontal(|ui| {
                //                 ui.add(
                //                     Image::new(album.get_texture()).fit_to_exact_size(vec2(35., 35.))
                //                 );
                //                 ui.label(format!("{}", album.get_title()));
                //                 if ui.button("Download").clicked() {
                //                     let app = self.app.clone();
                //                     let drawable_album = album.clone();

                //                     tokio::spawn(async move {
                //                         let quality = app.get_quality_or_highest_avaliable();
                //                         let _ = app.download_manager.enqueue_album(app.clone(), drawable_album, quality).await;
                //                     });
                   
                //                 }
                //             });
                //         });
                //     }
                //     SearchType::Playlist => (),
                // }
            
            });
        }
    }
}