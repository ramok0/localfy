use egui::{Rect, Pos2, Align2, FontId, Vec2, Rounding, Color32, Stroke, Image, include_image, Sense, vec2, pos2};
use crate::{app::App, constants::{TEXT_COLOR_SECONDARY, TEXT_COLOR}, gui::{helper, model::DrawableSong}};
use crate::gui::model::Drawable;

impl App {
    pub fn draw_controls(&mut self, ui: &mut egui::Ui, position_mut: &mut Rect) {
        if position_mut.height() < 70.0 {
            position_mut.min.y -= 70.0 - position_mut.height();
        }

        let position = *position_mut;

        const XMARGINTITLE: f32 = 0.02;
        ui.painter().rect(position, Rounding::same(0.0), Color32::BLACK, Stroke::NONE);
        let width = position.max.x - position.min.x;

        let center_x = (position.min.x + position.max.x) / 2.0;
        let center_y = (position.min.y + position.max.y) / 2.0;

        let center = Pos2 { x: center_x, y: center_y };

        let font = FontId::proportional(16.0);

        //on calcule les coordonneés du bouton play
        let play_min = center - Vec2 { x: 10.0, y: 15.0 };
        let play_max = center + Vec2 { x: 10.0, y: 5.0 };
        let play_button_rect = Rect { min: play_min, max: play_max };

        ui.scope(|ui| {
            let current_title_opt:Option<DrawableSong> = self.player.get_queue().current_title;//self.player.queue().current_title();
            if current_title_opt.is_some() {
                let current_title = current_title_opt.unwrap();

                let with_margin = position.expand(-15.0); // Deflate the position to create some padding
                let image_size = vec2(with_margin.height(), with_margin.height());
                let image_rect = Rect::from_min_size(with_margin.min, image_size);

                Image::new(current_title.get_texture()).rounding(Rounding::same(10.)).fit_to_exact_size(image_size).paint_at(ui, image_rect);

                // helper::croix(ui, image_rect.max);
                // helper::croix(ui, image_rect.min);

                let title = current_title.get_title();
                    let mut font_size = self.player.per_song_gui_settings.lock().unwrap().title_font_size;
                    let title_size = ui.painter().layout_no_wrap(title.clone(), FontId::proportional(font_size), TEXT_COLOR).size();
                    let play_button_rect_inflated = play_button_rect.expand(5.0); // Inflate the play button rect to create some padding
                    let title_min = Pos2 { x: image_rect.max.x + (position.width() * XMARGINTITLE), y: center_y };
                    let mut title_rect = Rect::from_min_size(title_min, title_size);

                    while title_rect.intersects(play_button_rect_inflated) && font_size > 8.0 {
                        font_size -= 1.0;
                        let new_font = FontId::proportional(font_size);
                        title_rect = Rect::from_min_size(title_min, ui.painter().layout_no_wrap(title.clone(), new_font, TEXT_COLOR).size());
                    }

                ui.painter().text(title_min, Align2::LEFT_CENTER, title.clone(), FontId::proportional(font_size), TEXT_COLOR);

                let artist_font = FontId::proportional(font.size - 5.0);
                let artist = current_title.get_item().artist;

                let artist_name_pos = pos2(title_min.x, title_min.y +15.);

                ui.painter().text(
                    artist_name_pos,
                    Align2::LEFT_CENTER,
                    artist.clone(),
                    artist_font,
                    TEXT_COLOR_SECONDARY
                );

             //   ui.image(current_title.get_texture());
            }


            let progress = self.player.get_progress();
            let duration = self.player.get_duration();

            if let Some(duration) = duration {
                if let Some(mut progress) = progress {
                    let progress_bar_width = width * 0.3;
                    ui.style_mut().spacing.slider_width = progress_bar_width;

                    //placer la barre de progression juste en dessous du bouton play mais avec la width qu'on vient de calculer
                    let progress_min = Pos2 {
                        x: play_max.x - 10.0 - progress_bar_width / 2.0,
                        y: center_y + 10.0,
                    };

                    let progress_max = Pos2 {
                        x: progress_min.x + progress_bar_width,
                        y: progress_min.y + 5.0,
                    };

                    let progress_rect = Rect { min: progress_min, max: progress_max };
                    // ui.painter().rect(progress_rect, Rounding::ZERO, Color32::RED, Stroke::NONE);
                    let song_duration_string = crate::time::ms_to_min_sec(duration as u64);
                    let current_cursor_string = crate::time::ms_to_min_sec(progress as u64);

                    // Place song_duration_string 5% to the right of progress_rect
                    let mut song_duration_pos = Pos2 {
                        x: progress_max.x + progress_bar_width * 0.1,
                        y: progress_min.y,
                    };

                    // Place current_cursor_string 5% to the left of progress_rect
                    let mut current_cursor_pos = Pos2 {
                        x: progress_min.x - progress_bar_width * 0.1,
                        y: progress_min.y,
                    };

                    // Ensure that the distance between progress bar and current_cursor_pos or song_duration_pos is at least 60
                    if progress_min.x - current_cursor_pos.x < 60.0 {
                        current_cursor_pos.x = progress_min.x - 60.0;
                    }

                    if song_duration_pos.x - progress_max.x < 60.0 {
                        song_duration_pos.x = progress_max.x + 60.0;
                    }

                    helper::place_text_at(
                        ui,
                        font.clone(),
                        song_duration_pos,
                        song_duration_string,
                        true
                    );
                    helper::place_text_at(
                        ui,
                        font.clone(),
                        current_cursor_pos,
                        current_cursor_string,
                        false
                    );

                    // if
                    //     ui
                    //         .put(
                    //             progress_rect,
                    //             Slider::new(&mut progress, 0..=duration).trailing_fill(true).show_value(false)
                    //         )
                    //         .changed()
                    // {
                    //     self.jukebox().set_progress(progress);
                    // }

                    if helper::slider(ui, progress_rect, &mut progress, 0..=duration, "_progressbar").changed() {
                        self.player.set_progress(progress);
                    }
                }
            }

            if self.player.is_playing() {
                if
                    ui
                        .put(
                            play_button_rect,
                            Image::new(include_image!("../../../assets/pause-solid.svg"))
                                  .fit_to_exact_size(Vec2::new(20., 20.))
                                .sense(Sense::click())
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                {
                    self.player.pause();
                }
            } else {
                if
                    ui
                        .put(
                            play_button_rect,
                            Image::new(include_image!("../../../assets/play-solid.svg"))
                                .fit_to_exact_size(Vec2::new(20., 20.))
                                .sense(Sense::click())
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                {
                    self.player.play();
                }
            }
            let volume_max = Pos2 {
                x: position.max.x - position.max.x * 0.015,
                y: center_y,
            };

            let volume_min = Pos2 {
                x: volume_max.x - 90.0,
                y: center_y,
            };

            let volume_bar_rect = Rect { min: volume_min, max: volume_max };

            let volume_icon_center = Pos2 {
                x: volume_min.x - 15.0,
                y: center_y,
            };

            let volume_icon_pos = Rect {
                min: volume_icon_center - Vec2 { x: 10.0, y: 0.0 },
                max: volume_icon_center + Vec2 { x: 10.0, y: 20.0 },
            };

            let image: egui::ImageSource<'_> = match self.player.get_volume() {
                0 => include_image!("../../../assets/volume-off-solid.svg"),
                1..=60 => include_image!("../../../assets/volume-low-solid.svg"),
                _ => include_image!("../../../assets/volume-high-solid.svg"),
            };

            ui.put(
                volume_icon_pos,
                Image::new(image).fit_to_exact_size(Vec2::new(15., 15.))
            );

            ui.style_mut().spacing.slider_width = 90.0;

            if helper::slider(ui, volume_bar_rect, &mut self.user_settings.volume, 0..=100, "_volume").changed() {
                self.player.set_volume(self.user_settings.volume);
            }
        });
    }
}