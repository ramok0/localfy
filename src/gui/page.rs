use std::sync::Arc;

use egui::ImageSource;

use crate::app::{App, AppImpl};



pub trait RenderablePage {
    fn get_page_title(&self) -> String;

    fn render(&self, app:&mut App, ui:&mut egui::Ui, max_rect:egui::Rect);

    fn get_page_icon(&self) -> ImageSource;
}

