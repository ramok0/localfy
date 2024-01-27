use std::time::{Duration, Instant};

use egui::{include_image, pos2, text, vec2, Align2, Color32, ComboBox, FontId, Image, Layout, OpenUrl, Pos2, Rect, Rounding, Sense};
use serde::de;
use crate::{app::App, constants::WARNING_COLOR, gui::helper};

impl App {
    pub fn draw_settings_page(&mut self, ui:&mut egui::Ui, max_rect:Rect) {
        let mut ui = ui.child_ui(max_rect, Layout::default());

        if self.gui_settings.should_restart {
            let rect = max_rect.with_max_y(100.).shrink(10.);

            ui.painter().rect_filled(rect, Rounding::same(10.), WARNING_COLOR);
            let center = rect.center();
            let item_spacing = ui.style().spacing.item_spacing.x;

            let warning_rect = Rect::from_center_size(pos2(rect.min.x + item_spacing + 20., center.y), vec2(20.0, 20.0));

            ui.put(warning_rect, Image::new(include_image!("../../../assets/warning.svg")).fit_to_exact_size(vec2(20., 20.)).tint(Color32::BLACK));
            
            let text_pos = pos2(warning_rect.max.x + (rect.width() - warning_rect.width())/2.0, center.y);
            ui.painter().text(text_pos, Align2::CENTER_CENTER, "Please restart the application to apply changes", FontId::monospace(14.), Color32::BLACK);

            let _ = ui.allocate_rect(rect.with_max_y(110.), Sense { click: false, drag: false, focusable: false });

        }

        if self.app.tidal_client.authorization().is_some() {
            ui.horizontal(|ui| {
                ui.label(format!("You are logged in as {}", self.app.tidal_client.authorization().as_ref().unwrap().user.username));
                if ui.button("Disconnect").clicked() {
                    let mut configuration = self.app.configuration.lock().unwrap();
                     configuration.refresh_token = None;
                     configuration.flush();
    
                    self.gui_settings.should_restart = true;
                }       
            });
        } else {

            if self.gui_settings.is_logging_in {
                if self.gui_settings.device_code.is_some() {
                    ui.label(format!("UserCode : {}", self.gui_settings.device_code.as_ref().unwrap().user_code));

                    if ui.button("Login").clicked() {
                        let url = format!("https://{}", self.gui_settings.device_code.as_ref().unwrap().verification_uri.clone());
                        ui.ctx().open_url(OpenUrl::new_tab(url));
                    }
                } else {
                    ui.label("Waiting for device code...");
                }
            } else {
                ui.horizontal(|ui| {
                    ui.label("You are not logged in");
                    if ui.button("Connect").clicked() {
                        let api: tidal_rs::client::TidalApi = self.app.tidal_client.clone();
                        let configuration = self.app.configuration.clone();
                        let event_sender = self.gui_settings.event_manager.0.clone();
                        
                        self.gui_settings.is_logging_in = true;

                        tokio::spawn(async move {
                            let device_code = api.auth().get_device_code().await;
                            if device_code.is_ok() {
                                let device_code = device_code.unwrap();

                                let _ = event_sender.send(crate::gui::model::Event::DeviceCode(Some(device_code.clone()))).await;
                                let created = Instant::now();
                                
                                while created.elapsed().as_secs() <= device_code.expires_in {
                                    let login = api.auth().login_from_device_code(&device_code.device_code).await;
                                    if login.is_err() {
                                        tokio::time::sleep(Duration::from_secs(device_code.interval)).await;
                                        continue;
                                    }

                                    let login = login.unwrap();

                                    if login.refresh_token.is_some() {
                                        let mut configuration = configuration.lock().unwrap();
                                        configuration.refresh_token = login.refresh_token;
                                        configuration.flush();
                                    }

                                    let _ = event_sender.send(crate::gui::model::Event::LogonWithTidal).await;
                                }

                                let _ = event_sender.send(crate::gui::model::Event::DeviceCode(None)).await;
                            }
                        });
                    }
                });
            }
        }

        ui.horizontal(|ui| {
            ui.label("Max downloads at once : ");
            let mut configuration = self.app.configuration.lock().unwrap();
            if ui.add(egui::Slider::new(&mut configuration.max_concurrency, 1..=25)).changed() {
                configuration.flush();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Download quality (maximal) : ");
            //combobox with all the qualities
            let mut configuration = self.app.configuration.lock().unwrap();
            let response = ComboBox::from_id_source("downloadquality").selected_text(configuration.quality.unwrap_or(tidal_rs::model::AudioQuality::Low).to_string()).show_ui(ui, |ui| {
                ui.selectable_value(&mut configuration.quality, Some(tidal_rs::model::AudioQuality::Low), "Low (96kbps)");
                ui.selectable_value(&mut configuration.quality, Some(tidal_rs::model::AudioQuality::High), "High (320kbps)");
                ui.selectable_value(&mut configuration.quality, Some(tidal_rs::model::AudioQuality::Lossless), "Lossless (16bit/44.1kHz)");
                ui.selectable_value(&mut configuration.quality, Some(tidal_rs::model::AudioQuality::Max), "Max (up to 24bit/192kHz)");
            }).response;

            if response.changed() {
                configuration.flush();
            }

        });
    }
}