use std::{sync::{Arc, Mutex}, ops::Deref, collections::VecDeque};

use rand::seq::SliceRandom;
use vlc::{Instance, MediaPlayer, Media};
use crate::song::Song;
use vlc::MediaPlayerAudioEx;

pub struct PlayerQueue {
    pub current_index: Option<usize>,
    pub current_title:Option<Song>,
    pub playlist:Vec<Song>,
    pub queue:VecDeque<Song>, //one time queue
    pub library:Vec<Song>, //every songs
    pub shuffle_positions:Vec<usize>,
    pub playback_mode:PlaybackMode,
    /* 
        shuffle_positions:

        Once the shuffle mode is activated, the shuffle_positions vector is filled with the indexes of the songs in the playlist.
        When we want to get the next song of the queue, we just have to use playlist[shuffle_positions[current_index + 1].
     */
}

impl Default for PlayerQueue {
    fn default() -> Self {
        Self {
            current_title:None,
            playlist: Vec::new(),
            queue: VecDeque::new(),
            library: Vec::new(),
            current_index: None,
            shuffle_positions: Vec::new(),
            playback_mode: PlaybackMode::Normal
        }
    }
}

impl PlayerQueue {
    pub fn set_library(&mut self, songs:Vec<Song>) {
        self.library = songs;
    }

    pub fn get_library(&self) -> &Vec<Song> {
        &self.library
    }

    pub fn set_current_media(&mut self, song:Option<&Song>) {
        self.current_title = song.cloned();
    }

    pub fn add_to_queue(&mut self, song:&Song) {
        self.queue.push_back(song.clone());
    }

    pub fn set_playlist(&mut self, songs:&Vec<Song>) {
        self.playlist = songs.clone();

        //when playlist change, shuffle positions should change too.
        self.generate_shuffle_positions();
    }

    pub fn get_playlist(&self) -> &Vec<Song> {
        &self.playlist
    }

    //TODO: impl shuffle.
    pub fn generate_shuffle_positions(&mut self) {
        let mut shuffle_positions:Vec<usize> = (0..self.playlist.len()).collect();
        shuffle_positions.shuffle(&mut rand::thread_rng());
        self.shuffle_positions = shuffle_positions;
    }

    pub fn get_next_song(&mut self) -> Option<Song> {
        self.queue.pop_front().or_else(|| {
            self.current_index.and_then(|index| {

                let inarray_index = if self.playback_mode == PlaybackMode::Shuffle {
                    *self.shuffle_positions.get(index+1).unwrap_or(&0)
                } else {
                    index+1
                };

                if let Some(song) = self.playlist.get(inarray_index) {
                    self.current_index = Some(index + 1);
                    return Some(song);
                } else {
                    None
                }
            }).or_else(|| {
                self.current_index = Some(0);
                self.playlist.first()
            }).cloned()
        })
    }


    pub fn get_previous_song(&mut self) -> Option<Song> {
        self.current_index.and_then(|index| {

            let inarray_index = if self.playback_mode == PlaybackMode::Shuffle {
                self.shuffle_positions[index-1]
            } else {
                index-1
            };

            if let Some(song) = self.playlist.get(inarray_index) {
                self.current_index = Some(index - 1);
                return Some(song);
            } else {
                None
            }
        }).or_else(|| {
            self.current_index = Some(0);
            self.playlist.first()
        }).cloned()
    }

    pub fn get_current_title(&self) -> Option<Song> {
        self.current_title.clone()
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
    pub fn new() -> Self {
        Player(Arc::new(PlayerImpl::new()))
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
    pub instance:Instance,
    pub media_player: vlc::MediaPlayer,
    pub per_song_gui_settings:Mutex<PerSongGuiSettings>,
    pub queue:Mutex<PlayerQueue>,
    pub event_manager:(std::sync::mpsc::Sender<vlc::EventType>, std::sync::mpsc::Receiver<vlc::EventType>)
}

impl PlayerImpl {
    pub fn new() -> Self {
        let instance = Instance::new().unwrap();
        let media_player = MediaPlayer::new(&instance).expect("failed to create media player");
        let event_manager = std::sync::mpsc::channel::<vlc::EventType>();

        let player = PlayerImpl {
            instance: instance,
            media_player: media_player,
            per_song_gui_settings: Mutex::new(PerSongGuiSettings::default()),
            queue: Mutex::new(PlayerQueue::default()),
            event_manager
        };


        let tx = player.event_manager.0.clone();
        let _ = player.media_player.event_manager().attach(vlc::EventType::MediaPlayerEndReached, move |_vlc, _event| {
            let _ = tx.send(vlc::EventType::MediaPlayerEndReached).unwrap();
        });

        player
    }

    pub fn tick(&self) {
        if let Ok(event) = self.event_manager.1.try_recv() {
            match event {
                vlc::EventType::MediaPlayerEndReached => {

                    let song = {
                        self.queue.lock().unwrap().get_next_song()
                    };

                    if song.is_some() {
                        let _ = self.set_media(song.as_ref().unwrap(), false);
                    }
                },
                _ => {}
            }
        }
    }

    pub fn set_playback_mode(&self, playback_mode:PlaybackMode) {
        self.queue().playback_mode = playback_mode;
    }

    pub fn playback_mode(&self) -> PlaybackMode {
        self.queue().playback_mode
    }

    pub fn queue(&self) -> std::sync::MutexGuard<'_, PlayerQueue> {
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
        let previous_song = {
            self.queue().get_previous_song()
        };

        if previous_song.is_some() {
            let _ = self.set_media(previous_song.as_ref().unwrap(), false);
        }
    }

    pub fn play_next(&self) {
        let next_song = {
            self.queue().get_next_song()
        };

        if next_song.is_some() {
            let _ = self.set_media(next_song.as_ref().unwrap(), false);
        }
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

    pub fn play_song(&self, song:&Song) -> Result<(), tidal_rs::error::Error> {
        let media = Media::new_path(&self.instance, song.path.clone()).ok_or(tidal_rs::error::Error::NotFound)?;
        self.media_player.set_media(&media);

        *self.per_song_gui_settings.lock().unwrap() = PerSongGuiSettings::default();
        self.play();

        {
            let mut queue = self.queue();
            queue.current_title = Some(song.clone());
        }

        Ok(())
    }

    pub fn set_media(&self, song:&Song, override_index:bool) -> Result<(), tidal_rs::error::Error>{
        self.play_song(&song)?;
        {   

             if override_index {
                 let mut queue = self.queue();
             queue.current_index = queue.playlist.iter().position(|x| x == song);
             }

        }
        //self.queue()().set_playlist(&vec![song.clone()]);
        //self.queue()().set_current_media(song);

        Ok(())
    }
}

unsafe impl Sync for PlayerImpl {}
unsafe impl Send for PlayerImpl {}