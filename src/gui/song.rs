
use std::sync::Arc;

use egui::{Image, Rounding, vec2, Label, RichText, Id};

use crate::app::{App, AppImpl};
use crate::constants::{TEXT_COLOR, self};
use crate::renderer::Drawable;
use crate::song::Song;



pub fn draw_song(app:&App, ui:&mut egui::Ui, song:&Song, height:f32) {
    if let Some(last_rect) = ui.memory_mut(|mem| mem.data.get_temp(Id::new(song))) {
        ui.painter().rect_filled(last_rect, Rounding::same(5.), constants::BACKGROUND_COLOR);
    }
    let last_item_spacing = ui.style_mut().spacing.item_spacing;
    ui.style_mut().spacing.item_spacing = vec2(25., 25.);

    let response = crate::gui::helper::scope_click(ui, |ui| {
        ui.horizontal(|ui| {

            ui.add(Image::new(song.get_texture()).rounding(Rounding::same(15.)).fit_to_exact_size(vec2(height /10., height /10.)));
           
            let text = song.get_title();
            let feats:Option<Vec<String>> = song.tidal_track.as_ref().and_then(|track| {
                let other_artists:Vec<String> = track.artists.iter().filter(|artist| artist.name != song.get_artist()).map(|x| x.name.clone()).collect();
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
    });

    ui.style_mut().spacing.item_spacing = last_item_spacing;
    ui.memory_mut(|mem: &mut egui::Memory| mem.data.insert_temp(Id::new(song), response.response.rect.expand(10.)));

    if response.response.clicked() {
        let _ = app.app.player.set_media(&song, true);
    }

    response.response.context_menu(|ui| {
        song_context_menu(app.app.clone(), ui, &song);
    });
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