use std::{ ops::Sub, vec };

use egui::{
    Rect,
    Pos2,
    Align2,
    FontId,
    Vec2,
    Rounding,
    Color32,
    Stroke,
    Image,
    include_image,
    Sense,
    vec2,
    pos2,
    ImageSource,
};
use crate::{
    app::App,
    constants::{
        TEXT_COLOR_SECONDARY,
        TEXT_COLOR,
        SECONDARY_ACTION_COLOR,
        SECONDARY_ACTION_COLOR_HOVER,
        SECONDARY_HOVER_COLOR,
    },
    gui::{ helper, model::DrawableSong }, player::PlaybackMode,
};
use crate::gui::model::Drawable;

const CONTROLS_ICONS_DISTANCE: f32 = 30.0;

fn add_icon_to_controls(
    ui: &mut egui::Ui,
    icon: ImageSource,
    position: Rect,
    size: Vec2,
    active:bool
) -> egui::Response {
    ui.put(position, Image::new(icon).tint(match active {
        true => SECONDARY_HOVER_COLOR,
        false => Color32::WHITE,
    }).sense(Sense::click()).fit_to_exact_size(size))
}

impl App {
    pub fn draw_controls(&mut self, ui: &mut egui::Ui, position_mut: &mut Rect) {
        if position_mut.height() < 70.0 {
            position_mut.min.y -= 70.0 - position_mut.height();
        }

        let position = *position_mut;

        const XMARGINTITLE: f32 = 0.02;
        ui.painter().rect(position, Rounding::same(0.0), Color32::BLACK, Stroke::NONE);
        let width = position.max.x - position.min.x;

        let center_y = position.center().y;

        let font = FontId::proportional(16.0);

        let play_button_rect = Rect::from_center_size(
            position.center().sub(vec2(0.0, 10.0)),
            vec2(30.0, 30.0)
        );

        ui.scope(|ui| {
            let current_title_opt: Option<DrawableSong> = {
                self.app.player.queue().get_current_title()
            }; //self.player.queue().current_title();
            if current_title_opt.is_some() {
                let current_title = current_title_opt.unwrap();

                let with_margin = position.expand(-15.0); // Deflate the position to create some padding
                let image_size = vec2(with_margin.height(), with_margin.height());
                let image_rect = Rect::from_min_size(with_margin.min, image_size);

                Image::new(current_title.get_texture())
                    .rounding(Rounding::same(10.0))
                    .fit_to_exact_size(image_size)
                    .paint_at(ui, image_rect);

                // helper::croix(ui, image_rect.max);
                // helper::croix(ui, image_rect.min);

                let title = current_title.get_title();
                let mut font_size = self.app.player.per_song_gui_settings
                    .lock()
                    .unwrap().title_font_size;
                let title_size = ui
                    .painter()
                    .layout_no_wrap(title.clone(), FontId::proportional(font_size), TEXT_COLOR)
                    .size();
                let play_button_rect_inflated = play_button_rect.expand(5.0); // Inflate the play button rect to create some padding
                let title_min = Pos2 {
                    x: image_rect.max.x + position.width() * XMARGINTITLE,
                    y: center_y,
                };
                let mut title_rect = Rect::from_min_size(title_min, title_size);

                while title_rect.intersects(play_button_rect_inflated) && font_size > 8.0 {
                    font_size -= 1.0;
                    let new_font = FontId::proportional(font_size);
                    title_rect = Rect::from_min_size(
                        title_min,
                        ui.painter().layout_no_wrap(title.clone(), new_font, TEXT_COLOR).size()
                    );
                }

                ui.painter().text(
                    title_min,
                    Align2::LEFT_CENTER,
                    title.clone(),
                    FontId::proportional(font_size),
                    TEXT_COLOR
                );

                let artist_font = FontId::proportional(font.size - 5.0);
                let artist = current_title.get_item().artist;

                let artist_name_pos = pos2(title_min.x, title_min.y + 15.0);

                ui.painter().text(
                    artist_name_pos,
                    Align2::LEFT_CENTER,
                    artist.clone(),
                    artist_font,
                    TEXT_COLOR_SECONDARY
                );

                //   ui.image(current_title.get_texture());
            }

            let progress = self.app.player.get_progress();
            let duration = self.app.player.get_duration();

            if let Some(duration) = duration {
                if let Some(mut progress) = progress {
                    let progress_bar_width = width * 0.3;
                    ui.style_mut().spacing.slider_width = progress_bar_width;

                    //placer la barre de progression juste en dessous du bouton play mais avec la width qu'on vient de calculer
                    let progress_min = Pos2 {
                        x: play_button_rect.max.x - 10.0 - progress_bar_width / 2.0,
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

                    if
                        helper
                            ::slider(ui, progress_rect, &mut progress, 0..=duration, "_progressbar")
                            .changed()
                    {
                        self.app.player.set_progress(progress);
                    }
                }
            }

            let playback_mode = self.app.player.playback_mode();
            let icon_size = vec2(20.0, 20.0);

            if add_icon_to_controls(ui, include_image!("../../../assets/repeat.svg"), play_button_rect.translate(vec2(CONTROLS_ICONS_DISTANCE * 2.0, 0.0)), icon_size, playback_mode == PlaybackMode::Repeat).clicked() {
                match playback_mode {
                    PlaybackMode::Repeat => self.app.player.set_playback_mode(crate::player::PlaybackMode::Normal),
                    _ => self.app.player.set_playback_mode(crate::player::PlaybackMode::Repeat),
                }
            }



            if
                add_icon_to_controls(
                    ui,
                    include_image!("../../../assets/forward-step-solid.svg"),
                    play_button_rect.translate(vec2(CONTROLS_ICONS_DISTANCE, 0.0)),
                    icon_size,
                    false
                ).clicked()
            {
                self.app.player.play_next();
            }

            if add_icon_to_controls(ui, include_image!("../../../assets/backward-step-solid.svg"), play_button_rect.translate(vec2(-CONTROLS_ICONS_DISTANCE, 0.0)), icon_size, false).clicked() {
                self.app.player.play_previous();
            }

            if add_icon_to_controls(ui, include_image!("../../../assets/shuffle-solid.svg"), play_button_rect.translate(vec2(-CONTROLS_ICONS_DISTANCE * 2.0, 0.0)), icon_size, playback_mode == PlaybackMode::Shuffle).clicked() {
                match playback_mode {
                    PlaybackMode::Shuffle => self.app.player.set_playback_mode(crate::player::PlaybackMode::Normal),
                    _ => self.app.player.set_playback_mode(crate::player::PlaybackMode::Shuffle),
                }
            }

            if add_icon_to_controls(ui, match self.app.player.is_playing() {
                true => include_image!("../../../assets/pause-solid.svg"),
                false => include_image!("../../../assets/play-solid.svg"),
            }, play_button_rect, vec2(30., 30.), false).clicked() {
                if self.app.player.is_playing() {
                    self.app.player.pause();
                } else {
                    self.app.player.play();
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

            let image: egui::ImageSource<'_> = match self.app.player.get_volume() {
                0 => include_image!("../../../assets/volume-off-solid.svg"),
                1..=60 => include_image!("../../../assets/volume-low-solid.svg"),
                _ => include_image!("../../../assets/volume-high-solid.svg"),
            };

            ui.put(volume_icon_pos, Image::new(image).fit_to_exact_size(Vec2::new(15.0, 15.0)));

            ui.style_mut().spacing.slider_width = 90.0;

            if
                helper
                    ::slider(
                        ui,
                        volume_bar_rect,
                        &mut self.user_settings.volume,
                        0..=100,
                        "_volume"
                    )
                    .changed()
            {
                self.app.player.set_volume(self.user_settings.volume);
            }
        });
    }
}
