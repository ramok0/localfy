use std::{sync::{Arc, Mutex}, ops::Deref, path::PathBuf, collections::VecDeque};


use vlc::{Instance, MediaPlayer, Media};

use crate::{app::AppImpl, database::Song, gui::model::DrawableSong};
use vlc::MediaPlayerAudioEx;

#[derive(Clone)]
pub struct PlayerQueue {
    pub current_title:Option<DrawableSong>
}

impl Default for PlayerQueue {
    fn default() -> Self {
        Self {
            current_title:None
        }
    }
}

pub struct Player(pub Arc<PlayerImpl>);

impl Deref for Player {
    type Target = PlayerImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Player {
    pub fn new(app:Arc<AppImpl>) -> Self {
        Player(Arc::new(PlayerImpl::new(app)))
    }
}

#[derive(Debug)]
pub struct PerSongGuiSettings {
    pub title_font_size:f32
}

impl Default for PerSongGuiSettings {
    fn default() -> Self {
        Self {
            title_font_size: 16.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum PlaybackMode {
    Repeat,
    Shuffle,
    Normal
}

pub struct PlayerImpl {
    pub app: Arc<AppImpl>,
    pub instance:Instance,
    pub media_player: vlc::MediaPlayer,
    pub per_song_gui_settings:Mutex<PerSongGuiSettings>,
    pub queue:Mutex<PlayerQueue>,
    pub playback_mode:Mutex<PlaybackMode>,
}

impl PlayerImpl {
    pub fn new(app:Arc<AppImpl>) -> Self {
        let instance = Instance::new().unwrap();
        let media_player = MediaPlayer::new(&instance).expect("failed to create media player");

        PlayerImpl {
            app,
            instance,
            media_player,
            per_song_gui_settings: Mutex::new(PerSongGuiSettings::default()),
            queue: Mutex::new(PlayerQueue::default()),
            playback_mode: Mutex::new(PlaybackMode::Normal)
        }
    }

    pub fn set_playback_mode(&self, playback_mode:PlaybackMode) {
        *self.playback_mode.lock().unwrap() = playback_mode;
    }

    pub fn playback_mode(&self) -> PlaybackMode {
        *self.playback_mode.lock().unwrap()
    }

    pub fn get_queue(&self) -> PlayerQueue {
        self.queue.lock().unwrap().clone()
    }

    pub fn raw_queue(&self) -> std::sync::MutexGuard<'_, PlayerQueue> {
        self.queue.lock().unwrap()
    }

    pub fn is_playing(&self) -> bool {
        self.media_player.is_playing()
    }

    pub fn play(&self) {
        if self.has_media() {
            //TODO: add error handling
            let _result = self.media_player.play();
        }
    }

    pub fn play_previous(&self) {
        todo!();
    }

    pub fn play_next(&self) {
        todo!();
    }

    pub fn pause(&self) {
        self.media_player.pause();
    }

    pub fn stop(&self) {
        self.media_player.stop();
    }

    pub fn set_position(&self, position:f32) {
        self.media_player.set_position(position);
    }

    pub fn has_media(&self) -> bool {
        self.media_player.get_media().is_some()
    }

    pub fn get_position(&self) -> Option<f32> {
        self.media_player.get_position()
    }

    pub fn get_duration(&self) -> Option<i64> {
        self.media_player.get_media().and_then(|media| media.duration())
    }

    pub fn get_progress(&self) -> Option<i64> {
        self.media_player.get_time()
    }

    pub fn set_progress(&self, time:i64) {
        self.media_player.set_time(time);
    }

    pub fn get_volume(&self) -> i32 {
        self.media_player.get_volume()
    }

    pub fn set_volume(&self, volume:i32) {
        let _ = self.media_player.set_volume(volume);
    }

    pub fn set_media(&self, song:&DrawableSong) -> Result<(), tidal_rs::error::Error>{
        let media = Media::new_path(&self.instance, song.song.path.clone()).ok_or(tidal_rs::error::Error::NotFound)?;
        self.media_player.set_media(&media);
        *self.per_song_gui_settings.lock().unwrap() = PerSongGuiSettings::default();
        self.play();
        self.raw_queue().current_title = Some(song.clone());

        Ok(())
    }
}