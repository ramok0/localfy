use app::App;
use egui::Color32;
use tidal_rs::client::TidalApi;

pub mod app;
pub mod download;
pub mod constants;
pub mod gui;
pub mod database;
pub mod song;
pub mod configuration;
pub mod player;
pub mod time;
pub mod cache;
pub mod playlist;
pub mod renderer;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let mut tidal_api = TidalApi::new().expect("Failed to create Tidal Client");
    let mut configuration = configuration::Configuration::new();

    if let Some(refresh_token) = configuration.get_refresh_token() {
        let authorization = tidal_api.auth().login_from_refresh_token(&refresh_token).await;
        if authorization.is_ok() {
            tidal_api.set_authorization(Some(authorization.unwrap()));

            if configuration.quality.is_none() {
                configuration.quality = Some(tidal_api.user().get_current_account_highest_sound_quality().await.unwrap());
            }
        }
    }

    
    
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        vsync: true,
        ..Default::default()
    };
    eframe::run_native(
        "Localfy",
        options,
        Box::new(|cc| {
  
            let context = cc.egui_ctx.clone();
            std::thread::spawn(move || {
                loop { //redraw the ui at least every 100ms
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    context.request_repaint();
                }
            });

            cc.egui_ctx.style_mut(|style| {
                style.visuals.selection.bg_fill = Color32::WHITE;
            });

            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<crate::app::App>::new(App::new(tidal_api, configuration))
        }),
    )
}
