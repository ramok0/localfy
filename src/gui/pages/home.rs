use egui::{Rect, Layout, Image, Vec2, Sense, ScrollArea, vec2, Rounding};
use crate::{app::{App}, gui::{model::Drawable, helper::{centerer, scope_click}, song::draw_song}};

impl App {
    pub fn draw_home_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        ui.label("Home Page");

        println!("self.gui_settings.song_array.songs : {}",  self.gui_settings.song_array.songs.len());

        ScrollArea::new([false, true]).show(&mut ui, |ui| {
            let height = ui.available_height();
            self.gui_settings.song_array.songs.iter().for_each(|song| {
                // let response = scope_click(ui, |ui| {
                //     ui.horizontal(|ui| {
                //         ui.add(
                //             Image::new(song.get_texture()).rounding(Rounding::same(15.)).fit_to_exact_size(
                //                 vec2(60., 60.)
                //             )
                //         );
                //         ui.label(format!("{}", song.get_title()));
                //     });
                // });

                // if response.response.clicked() {
                //     println!("Clicked");
                //     let _ = self.player.set_media(&song);
                // }

                draw_song(&self, ui, song,height );
        });
        });
    }
}