use std::sync::{Arc, Mutex};
use tidal_rs::client::TidalApi;

use crate::{download::DownloadManager, configuration::Configuration, gui::model::GuiInput, database::DatabaseWrapper, player::Player, cache::CacheManager};

pub struct UserSettings {
    pub volume: i32,
}

pub struct App {
    pub app: Arc<AppImpl>,
    pub player:Player,
    pub gui_settings: GuiInput,
    pub user_settings:UserSettings
}

impl App {
    pub fn new(client:TidalApi, configuration:Configuration) -> Self {
        let app = Arc::new(AppImpl::new(client, configuration));

        let mut result = App {
            app: app.clone(),
            player: Player::new(app.clone()),
            gui_settings: GuiInput::default(),
            user_settings: UserSettings {
                volume: 100
            }
        };

        result.user_settings.volume = result.player.get_volume();

        result
    }
}

pub struct AppImpl {
    pub tidal_client: TidalApi,
    pub download_manager: DownloadManager,
    pub configuration: Arc<Mutex<Configuration>>,
    pub database:DatabaseWrapper,
    pub cache_manager:Arc<tokio::sync::Mutex<CacheManager>>
}


impl AppImpl {
    pub fn new(tidal_client:TidalApi, configuration:Configuration) -> Self {
        let app = Self {
            tidal_client: tidal_client,
            download_manager: DownloadManager::new(10),
            configuration: Arc::new(Mutex::new(configuration)),
            database: DatabaseWrapper::new(),
            cache_manager: Arc::new(tokio::sync::Mutex::new(CacheManager::new()))
        };

        app.download_manager.work();

        app
    }
}

