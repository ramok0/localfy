use std::sync::Arc;

use egui::{pos2, vec2, Align2, Color32, FontFamily, FontId, Image, Label, RichText, Rounding};
use tidal_rs::model::Album;

use crate::{app, constants::{BACKGROUND_COLOR, TEXT_COLOR}, gui::{helper, model::Pages, page::RenderablePage, song::SongWidget}, renderer::Drawable};



impl RenderablePage for Album {
    fn get_page_title(&self) -> String {
        self.title.clone()
    }

    fn render(&self, application:&mut crate::app::App, ui:&mut egui::Ui, max_rect:egui::Rect) {


        let tracks = application.app.database().albums().resolve_album(&self);

        let mut list_rect = max_rect.expand2(egui::vec2(0., -50.)).shrink(35.);


        let font = FontId::new(17., FontFamily::Name("Montserrat".into()));
        
    //    ui.painter().text(pos2(paint_rect.min.x, paint_rect.min.y - 35.0), Align2::LEFT_CENTER, &self.title, font, TEXT_COLOR);


        let res = ui.scope(|ui| {
            ui.style_mut().spacing.item_spacing.x += 15.;
            ui.horizontal(|ui| {
                ui.add(Image::new(self.get_texture()).fit_to_exact_size(vec2(60., 60.)).rounding(Rounding::same(15.)));
                ui.add(Label::new(RichText::new(&self.title).font(font).color(TEXT_COLOR)));
                if ui.button("go back").clicked() {
                    application.gui_settings.location = crate::gui::model::UserLocation::Home;
                    application.gui_settings.page = Pages::Home;
                }
            }).response
        }).inner;


        // Adjust the height of list_rect to have at least 5 pixels difference with res.rect
        let padding_rect = res.rect.expand(15.);
        if list_rect.intersects(padding_rect) {
            list_rect.min.y = padding_rect.max.y + 15.0;
        }

        let paint_rect = list_rect.expand(10.0);

        ui.painter().rect_filled(paint_rect, 10.0, BACKGROUND_COLOR);
       // ui.painter().text(pos2(paint_rect.min.x, paint_rect.min.y - 10.0), Align2::LEFT_CENTER, "Albums - WorkInProgress", FontId::proportional(14.), Color32::RED);

        let mut container = ui.child_ui(list_rect, egui::Layout::default());

        egui::ScrollArea::new([false, true]).show(&mut container, |container: &mut egui::Ui| {
            tracks.iter().for_each(|song| {
                let response = container.add(SongWidget::new(song.clone()));

                if response.clicked() {
                    song.on_clicked(application.app.clone(), crate::gui::model::UserLocation::Album(self.clone()));
                }

                song.context_menu(response, ui, application);
            });
        });

    }

    fn get_page_icon(&self) -> egui::ImageSource {
        todo!()
    }
}

        


