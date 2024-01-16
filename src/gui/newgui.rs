use std::{hash::Hash, collections::hash_map::DefaultHasher};
use std::hash::Hasher;
use egui::{Color32, Margin, Pos2, Rect};

use crate::{constants, gui::model::Pages};

use super::model::{Event, DrawableSongArray};

impl eframe::App for crate::app::App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(event) = self.gui_settings.event_manager.1.try_recv() {
            match event {
                Event::SearchResult(tracks) => {
                    self.gui_settings.search_results = tracks;
                },
                Event::SongArray(song_array) => {
                    self.gui_settings.requested_song_array = false;
                    self.gui_settings.song_array = song_array;
                }
            }
        }

        let songs = self.app.database.songs().get_songs();
        let mut hasher = DefaultHasher::new();
        songs.hash(&mut hasher);
        let song_array_hash = hasher.finish();

        if !self.gui_settings.requested_song_array && self.gui_settings.song_array.hash != song_array_hash && self.gui_settings.song_array.created_at.elapsed().as_secs() > 2 {
            self.gui_settings.requested_song_array = true;
            
            let ctx = ctx.clone();
            let tx = self.gui_settings.event_manager.0.clone();
            self.gui_settings.requested_song_array = true;
            let cache_manager = self.app.cache_manager.clone();
            tokio::spawn(async move {
                let song_array = DrawableSongArray::from_song_array(&ctx,songs, cache_manager).await;
                let _  = tx.send(Event::SongArray(song_array)).await;
            });
        }

        // if ctx.input(|i| i.key_released(egui::Key::Space)) {
        //     if self.player.is_playing() {
        //         self.player.pause();
        //     } else {
        //         self.player.play();
        //     }
        // }
        
        let my_frame = egui::containers::Frame {
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::from_rgb(0, 0, 0),
            stroke: egui::Stroke::NONE,
            inner_margin: Margin::same(10.0),
            outer_margin: Margin::same(0.0),
        };

        
        egui::CentralPanel
            ::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                if let Some(window_pos) = ui.input(|i| { i.viewport().inner_rect }) {
                    let window_size = window_pos.max - window_pos.min;

                    let controls_size_y = window_size.y * constants::CONTROLS_SIZE_PERCENT;

                    const NAVBAR_SIZE_X:u32 = 90;
                    let navbar_size_y = window_size.y - controls_size_y;

                    let navbar_position_min = Pos2 { x: 0.0, y: 0.0 };
                    let navbar_position_max = Pos2 { x: NAVBAR_SIZE_X as f32, y: navbar_size_y };
                    let mut navbar_position = Rect {
                        min: navbar_position_min,
                        max: navbar_position_max,
                    };

                    let mut panel_rect = Rect::from_min_max(Pos2 {x:0.0, y:0.0},Pos2 {x:0.0, y:0.0});

                    self.draw_navbar(ui, &mut navbar_position, &mut panel_rect);

                    let controls_position_min = Pos2 { x: 0.0, y: navbar_position.height() };
                    let controls_position_max = Pos2 { x: window_size.x, y: window_size.y };
                    let mut controls_position = Rect {
                        min: controls_position_min,
                        max: controls_position_max,
                    };

                    self.draw_controls(ui, &mut controls_position);

                    let container_min_x = navbar_position.max.x + ui.style().spacing.item_spacing.x;
                    let container_max_y = window_size.y - controls_position.height();
                    
                    //calculer le rectangle avec les diagonales
                    let container_position_min = Pos2 { x: container_min_x, y: panel_rect.min.y };
                    let container_position_max = Pos2 { x: window_size.x, y: container_max_y };

                    let container_rect = Rect {
                        min: container_position_min,
                        max: container_position_max,
                    };

                    match self.gui_settings.page {
                        Pages::Home => {
                            self.draw_home_page(ui, container_rect);
                        },
                        Pages::Search => {
                            self.draw_search_page(ui, container_rect);
                        },
                        Pages::Downloads => {
                            self.draw_downloads_page(ui, container_rect);
                        },
                        Pages::Settings => {
                            self.draw_settings_page(ui, container_rect);
                        }
                    }
                };
            });
    }
}