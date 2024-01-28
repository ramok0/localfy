
use std::sync::Arc;

use egui::{vec2, Color32, Id, Image, InnerResponse, Label, Rect, RichText, Rounding, Sense, Vec2, Widget};

use crate::app::{App, AppImpl};
use crate::constants::{TEXT_COLOR, self};
use crate::renderer::Drawable;
use crate::song::Song;

use super::helper;

pub struct SongWidget {
    song:Song,
    image_size:f32,
    background_color:Option<Color32>
}

impl SongWidget {
    pub fn new(song:Song) -> Self {
        Self {
            song,
            image_size:50.,
            background_color: None
        }
    }
    
    pub fn background_color(mut self, background_color:Color32) -> Self {
        self.background_color = Some(background_color);
        self
    }

    pub fn image_size(mut self, image_size:f32) -> Self {
        self.image_size = image_size;
        self
    }

    pub fn song(&self) -> &Song {
        &self.song
    }
}

impl Widget for SongWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {

        let response = ui.allocate_ui(vec2(ui.available_width(), self.image_size), |ui| {


            if let Some(color) = self.background_color {
                if let Some(rect) = ui.memory(|mem| mem.data.get_temp::<Rect>(Id::new(self.song()))) {
                    ui.painter().rect_filled(rect, Rounding::same(5.), color);
               }
            }

            let response = helper::scope_click(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Image::new(self.song.get_texture()).rounding(Rounding::same(15.)).fit_to_exact_size(Vec2::splat(self.image_size)));
    
                    let text = self.song.get_title();
    
                    let feats:Option<Vec<String>> = self.song.tidal_track.as_ref().and_then(|track| {
                        let other_artists:Vec<String> = track.artists.iter().filter(|artist| artist.name != self.song.get_artist()).map(|x| x.name.clone()).collect();
                        if other_artists.len() > 0 {
                            Some(other_artists)
                        } else {
                            None
                        }
                    });
    
                    let text = if let Some(feats) = feats {
                        let mut text = text.clone();
                        if !text.contains("feat") {
                            text.push_str(" (feat. ");
                            text.push_str(&feats.join(", "));
                            text.push_str(")");
                            text
                        } else {
                            text
                        }
                    } else {
                        text
                    };
        
                    ui.add(Label::new(RichText::new(text).color(TEXT_COLOR)));
                });
            }).response;

            ui.memory_mut(|mem| mem.data.insert_temp(Id::new(self.song()), response.rect));

            response
        }).inner;

        response
    }
}

pub fn song_context_menu(app:Arc<AppImpl>, ui:&mut egui::Ui, song:&Song) {
    if ui.button("Add to waiting-list").clicked() {
        app.player.queue().add_to_queue(&song);
        ui.close_menu();
    }

    if ui.button("Add to playlist").clicked() {
        
    }

    if ui.button("Delete").clicked() {
        let _ = app.database().songs().remove_song(song.clone());
        ui.close_menu();
    }
    

}