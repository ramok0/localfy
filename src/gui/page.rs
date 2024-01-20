

pub trait RenderablePage {
    fn get_title(&self) -> String;

    fn render(&mut self, ui:&mut egui::Ui, max_rect:egui::Rect);

    fn get_icon(&self) -> Option<egui::TextureHandle>;
}

