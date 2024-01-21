use std::{
    path::PathBuf,
    fs::File,
    sync::{ Arc, Mutex },
    collections::hash_map::DefaultHasher,
    hash::Hasher,
    io::{Write, Seek, Read, BufRead},
    os::windows::fs::OpenOptionsExt,
};

use serde::{ Serialize, Deserialize };
use tidal_rs::model::Track;
use std::hash::Hash;

use crate::{ song::Song, playlist::Playlist };

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CachedHashSong {
    pub hash: u64,
    pub song: Song,
}

impl PartialEq for CachedHashSong {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl From<Song> for CachedHashSong {
    fn from(value: Song) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        CachedHashSong {
            hash: hasher.finish(),
            song: value,
        }
    }
}
impl From<&Song> for CachedHashSong {
    fn from(value: &Song) -> Self {
        CachedHashSong::from(value.clone())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseDataContainer {
    #[serde(default)]
    tracks: Vec<CachedHashSong>,
    #[serde(default)]
    playlists: Vec<Playlist>,
}

impl DatabaseDataContainer {
    pub fn tracks(&self) -> &Vec<CachedHashSong> {
        &self.tracks
    }
}

impl Default for DatabaseDataContainer {
    fn default() -> Self {
        DatabaseDataContainer {
            tracks: Vec::new(),
            playlists: Vec::new(),
        }
    }
}

pub type DatabaseData = std::sync::Arc<std::sync::Mutex<DatabaseDataContainer>>;

pub struct Database {
    data: Arc<Mutex<DatabaseDataContainer>>,
}

impl Drop for Database {
    fn drop(&mut self) {
        self.flush();
    }
}

pub struct SongController {
    data: Arc<Mutex<DatabaseDataContainer>>,
}

impl SongController {
    pub fn add_song(&self, song: Song) {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.data.lock().unwrap();
        let cached_song = CachedHashSong::from(&song);

        if data.tracks.contains(&cached_song) {
            let position = data.tracks
                .iter()
                .position(|x| *x == cached_song)
                .unwrap();
            data.tracks[position] = cached_song;
        } else {
            data.tracks.push(cached_song);
        }
    }

    pub fn get_songs(&self) -> Vec<Song> {
        self.data
            .lock()
            .unwrap()
            .tracks.iter()
            .map(|x| x.song.clone())
            .collect()
    }

    pub fn remove_song(&self, song: Song) -> Result<(), std::io::Error> {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.data.lock().unwrap();
        let cached_song = CachedHashSong::from(&song);

        if data.tracks.contains(&cached_song) {
            let position = data.tracks
                .iter()
                .position(|x| *x == cached_song)
                .unwrap();
            data.tracks.remove(position);
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Song not found"));
        }

        Ok(())
    }
}

impl Database {
    fn get_database_path() -> PathBuf {
        if let Ok(program_data) = std::env::var("PROGRAMDATA") {
            let path_buf = PathBuf::from(program_data).join("Localfy").join("localfy.db");

            if !path_buf.exists() {
                std::fs::create_dir_all(path_buf.parent().unwrap()).unwrap();
            }

            return path_buf;
        }

        PathBuf::from("localfy.db")
    }

    pub fn read_database(
        file: &File,
        data: &mut DatabaseDataContainer
    ) -> Result<(), std::io::Error> {
        let mut reader = std::io::BufReader::new(file);
        
        // Check if the file is empty
        if reader.fill_buf()?.is_empty() {
            println!("buffer is empty");
            return Ok(());
        }
    
        let config = serde_json::from_reader(reader);
        if let Ok(config) = config {
            *data = config;
        }
    
        return Ok(());
    }

    pub fn new() -> Self {
        let path = Database::get_database_path();
        

        let mut data_container = DatabaseDataContainer::default();
        if let Ok(file) = File::options().read(true).open(&path) {
            Database::read_database(&file, &mut data_container).unwrap();
        }

        Database {
            data: Arc::new(Mutex::new(data_container)),
        }
    }

    pub fn flush(&mut self) {
        let data = self.data.lock().unwrap();
        let str = serde_json::to_string(&data.clone()).unwrap();
        std::fs::write(Self::get_database_path(), str);
    }

    pub fn songs(&self) -> SongController {
        SongController {
            data: self.data.clone(),
        }
    }
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}
