use std::sync::Arc;

use egui::ImageSource;

use crate::app::AppImpl;



pub trait RenderablePage {
    fn get_title(&self) -> String;

    fn render(&self, app:Arc<AppImpl>, ui:&mut egui::Ui, max_rect:egui::Rect);

    fn get_icon(&self) -> ImageSource;
}

