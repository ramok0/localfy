use std::sync::{Arc, Mutex};
use tidal_rs::{client::TidalApi, model::AudioQuality};

use crate::{download::DownloadManager, configuration::Configuration, gui::model::GuiInput, database::Database, player::Player, cache::CacheManager};

pub struct UserSettings {
    pub volume: i32,
}

pub struct App {
    pub app: Arc<AppImpl>,
    pub gui_settings: GuiInput,
    pub user_settings:UserSettings
}

impl App {
    pub fn new(client:TidalApi, configuration:Configuration) -> Self {
        let app = Arc::new(AppImpl::new(client, configuration));

        let mut result = App {
            app: app.clone(),
            gui_settings: GuiInput::default(),
            user_settings: UserSettings {
                volume: 100
            }
        };

        result.user_settings.volume = result.app.player.get_volume();

        result
    }
}

pub struct AppImpl {
    pub tidal_client: TidalApi,
    pub download_manager: DownloadManager,
    pub configuration: Arc<Mutex<Configuration>>,
    pub database:Mutex<Database>,
    pub cache_manager:Arc<tokio::sync::Mutex<CacheManager>>,
    pub player: Player
}



impl AppImpl {
    pub fn new(tidal_client:TidalApi, configuration:Configuration) -> Self {
        let app = Self {
            tidal_client: tidal_client,
            player: Player::new(),
            download_manager: DownloadManager::new(configuration.max_concurrency()),
            configuration: Arc::new(Mutex::new(configuration)),
            database: Mutex::new(Database::new()),
            cache_manager: Arc::new(tokio::sync::Mutex::new(CacheManager::new()))
        };



        app.download_manager.work();

        app
    }

    pub fn get_quality_or_highest_avaliable(&self) -> AudioQuality
    {
        let configuration = self.configuration.lock().unwrap();
        if let Some(quality) = configuration.quality {
            quality
        } else {
            drop(configuration); //release lock
            let tidal_client = self.tidal_client.clone();
            let configuration = self.configuration.clone();
            tokio::spawn(async move {
                let quality = tidal_client.user().get_current_account_highest_sound_quality().await.unwrap_or(AudioQuality::Lossless);
                let mut configuration = configuration.lock().unwrap();
                configuration.quality = Some(quality);
                configuration.flush();
            });

            
            AudioQuality::Lossless
        }
    }

    pub fn database(&self) -> std::sync::MutexGuard<'_, Database, > {
        self.database.lock().unwrap()
    }
}

