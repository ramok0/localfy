use egui::{vec2, Color32, Layout, ProgressBar, Rect, Rounding, ScrollArea};
use crate::{app::App, constants::BACKGROUND_COLOR,  download::DownloadStatus};

impl App {
    pub fn draw_downloads_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        ui.vertical_centered(|ui| {
            ui.heading("Downloads");
        });

        let downloads = self.app.download_manager.get_downloads();

        if downloads.len() == 0 {
            ui.label("No downloads");
        } else {
            ui.label(format!("{} downloads", downloads.len()));
        }

        let list_rect = max_rect.expand2(vec2(0., -50.)).expand(-35.);

        //create padding
        let mut download_ui = ui.child_ui(list_rect, Layout::default());
        ui.painter().rect_filled(list_rect, Rounding::same(5.), BACKGROUND_COLOR);

        ScrollArea::new([false, true]).show(&mut download_ui, |download_ui: &mut egui::Ui| {
            downloads.iter().for_each(|download| {
                let res = download_ui.scope(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} - {}", download.download.track.title, download.download.track.get_artist().name));
                        
                  //      progress_bar(ui, download.progress, vec2(ui.available_width() - ui.spacing().item_spacing.x, 30.));

                        ui.add(ProgressBar::new(download.progress).animate(true).show_percentage().fill(Color32::from_rgb(0x1b, 0x6f, 0x06)));
                    });
                }).response;
    
                res.on_hover_ui_at_pointer(|ui| {
                    if download.status == DownloadStatus::Downloading {
                        ui.label(format!("{}%", (download.progress * 100.0).round()));
                        if download.speed.as_kbps() > 1000.0 {
                            ui.label(format!("{} MB/s", (download.speed.as_mbps()*100.0).round() / 100.0));
                        } else {
                            ui.label(format!("{} KB/s", (download.speed.as_kbps()*100.0).round() / 100.0));
                        }
                    }
                    //calculer bytes => mb
                    ui.label(format!("File Size : {} MB", (download.total_size as f32 / 10000 as f32).round() / 100.0 ));
                    ui.label(format!("Status : {}", download.status.to_string()));
                });
            });
        });
    }
}